mod cricinfo;
use serde_json::{json, Value};
use std::{env, io::Read, time::Duration};
type FeedError = (&'static str, &'static str, Option<u64>);
const MAX_BODY: u64 = 2 * 1024 * 1024;
fn text(v: &Value) -> String {
    v.as_str()
        .map(str::to_owned)
        .unwrap_or_else(|| {
            if v.is_number() {
                v.to_string()
            } else {
                String::new()
            }
        })
        .chars()
        .filter(|c| !c.is_control())
        .take(180)
        .collect()
}
fn number(v: &Value) -> Value {
    v.as_u64()
        .map(|n| json!(n))
        .or_else(|| {
            v.as_str()
                .and_then(|s| s.parse::<u32>().ok())
                .map(|n| json!(n))
        })
        .unwrap_or(Value::Null)
}
fn array(v: &Value) -> Result<&Vec<Value>, &'static str> {
    v.as_array().ok_or("Unexpected provider response")
}
fn team(id: &Value, name: &Value, score: &Value) -> Value {
    json!({"id":text(id),"name":text(name),"score":number(score)})
}
fn normalize(sport: &str, v: &Value) -> Result<Vec<Value>, &'static str> {
    let mut out = Vec::new();
    match sport {
        "nfl" | "football" | "rugby" => {
            // CDN responses wrap the scoreboard in content.sbData. A missing
            // wrapper is an error, never an empty successful refresh.
            for e in array(&v["content"]["sbData"]["events"])? {
                for c in array(&e["competitions"])? {
                    let teams: Vec<Value> = array(&c["competitors"])?
                        .iter()
                        .map(|x| team(&x["team"]["id"], &x["team"]["displayName"], &x["score"]))
                        .collect();
                    if teams.len() != 2 {
                        continue;
                    }
                    let status = if c["status"].is_object() {
                        &c["status"]
                    } else {
                        &e["status"]
                    };
                    let state = match status["type"]["state"].as_str() {
                        Some("in") => "live",
                        Some("post") => "finished",
                        Some("pre") => "scheduled",
                        _ => "unknown",
                    };
                    out.push(json!({"id":format!("{}:{}",sport,text(&e["id"])),"sport":sport,"name":text(&e["name"]),"state":state,"detail":text(&status["type"]["shortDetail"]),"start":text(&e["date"]),"teams":teams,"innings":[]}));
                }
            }
        }
        _ => return Err("Unsupported sport"),
    }
    if out.len() > 500 {
        return Err("Too many matches in response");
    }
    if out.iter().any(|m| {
        text(&m["id"]).ends_with(':')
            || m["teams"]
                .as_array()
                .is_none_or(|ts| ts.iter().any(|t| text(&t["name"]).is_empty()))
    }) {
        return Err("Provider omitted match or team identity");
    }
    Ok(out)
}
fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::from(std::time::SystemTime::now())
}
// Round future HTTP dates up so parsing never shortens the server's wait.
fn retry_after(value: &str, observed: chrono::DateTime<chrono::Utc>) -> Option<u64> {
    let value = value.trim();
    if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) {
        return Some(value.parse::<u64>().unwrap_or(u64::MAX));
    }
    let date = chrono::DateTime::parse_from_rfc2822(value).ok()?;
    let millis = date
        .signed_duration_since(observed)
        .num_milliseconds()
        .max(0) as u64;
    Some(millis.div_ceil(1000))
}
fn response_error(
    status: u16,
    header: Option<&str>,
    observed: chrono::DateTime<chrono::Utc>,
) -> Option<FeedError> {
    let (state, message) = match status {
        200 => return None,
        401 | 403 => ("access-denied", "Provider denied access"),
        429 => ("rate-limited", "Provider quota reached; polling backed off"),
        _ => ("failed", "Provider returned an HTTP error"),
    };
    Some((
        state,
        message,
        header.and_then(|v| retry_after(v, observed)),
    ))
}
fn feed_url(sport: &str, league: &str) -> Result<String, FeedError> {
    let url = match sport {
        "nfl" => "https://cdn.espn.com/core/nfl/scoreboard?xhr=1&limit=50".to_owned(),
        "football" => {
            if ![
                "eng.1",
                "eng.2",
                "eng.3",
                "eng.4",
                "sco.1",
                "uefa.champions",
                "uefa.europa",
                "esp.1",
                "ger.1",
                "ita.1",
                "fra.1",
                "usa.1",
                "fifa.world",
            ]
            .contains(&league)
            {
                return Err(("unsupported", "Football competition is not supported", None));
            }
            format!("https://cdn.espn.com/core/soccer/scoreboard?xhr=1&league={league}")
        }
        "rugby" => {
            if !["267979", "180659"].contains(&league) {
                return Err(("unsupported", "Rugby competition is not supported", None));
            }
            format!("https://cdn.espn.com/core/rugby/scoreboard?xhr=1&league={league}")
        }
        "cricket" => "https://static.cricinfo.com/rss/livescores.xml".to_owned(),
        _ => return Err(("unsupported", "Unsupported sport", None)),
    };
    Ok(url)
}
fn fetch(sport: &str, league: &str) -> Result<(Vec<Value>, u64), FeedError> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(12))
        .connect_timeout(Duration::from_secs(5))
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            if attempt.previous().len() < 3
                && cricinfo::allowed_url(attempt.url())
                && attempt.previous().iter().all(cricinfo::allowed_url)
            {
                attempt.follow()
            } else {
                attempt.stop()
            }
        }))
        .user_agent("SportsBar/0.1.0")
        .build()
        .map_err(|_| ("failed", "Could not create HTTPS client", None))?;
    let request = client.get(feed_url(sport, league)?);
    let response = request
        .send()
        .map_err(|_| ("offline", "Feed unreachable or timed out", None))?;
    if let Some(error) = response_error(
        response.status().as_u16(),
        response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok()),
        now(),
    ) {
        return Err(error);
    }
    let mut bytes = Vec::new();
    response
        .take(MAX_BODY + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| ("failed", "Could not read feed", None))?;
    if bytes.len() as u64 > MAX_BODY {
        return Err(("failed", "Feed exceeded response size limit", None));
    }
    if sport == "cricket" {
        let xml = std::str::from_utf8(&bytes)
            .map_err(|_| ("failed", "Cricket feed is not UTF-8", None))?;
        return cricinfo::parse(xml)
            .map(|matches| (matches, cricinfo::POLL_INTERVAL_SEC))
            .map_err(|e| ("failed", e, None));
    }
    let v: Value = serde_json::from_slice(&bytes)
        .map_err(|_| ("failed", "Feed returned invalid JSON", None))?;
    normalize(sport, &v)
        .map(|matches| (matches, 30))
        .map_err(|e| ("failed", e, None))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.get(1).map(String::as_str) == Some("--demo") {
        println!("{}", include_str!("../demo/fixtures/matches.json"));
        return;
    }
    let sport = args.get(1).map(String::as_str).unwrap_or("");
    let league = args
        .get(2)
        .map(String::as_str)
        .unwrap_or(if sport == "rugby" { "267979" } else { "eng.1" });
    let output = match fetch(sport, league) {
        Ok((matches, poll_interval_sec)) => {
            json!({"sport":sport,"state":if matches.is_empty(){"empty"}else{"ready"},"matches":matches,"updated":now().timestamp_millis(),"pollIntervalSec":poll_interval_sec})
        }
        Err((state, message, retry_after_sec)) => {
            json!({"sport":sport,"state":state,"message":message,"matches":[],"retryAfterSec":retry_after_sec})
        }
    };
    println!("{output}");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cdn_routes_and_league_allowlist() {
        assert_eq!(
            feed_url("nfl", "").unwrap(),
            "https://cdn.espn.com/core/nfl/scoreboard?xhr=1&limit=50"
        );
        for league in [
            "eng.1",
            "eng.2",
            "eng.3",
            "eng.4",
            "sco.1",
            "uefa.champions",
            "uefa.europa",
            "esp.1",
            "ger.1",
            "ita.1",
            "fra.1",
            "usa.1",
            "fifa.world",
        ] {
            assert_eq!(
                feed_url("football", league).unwrap(),
                format!("https://cdn.espn.com/core/soccer/scoreboard?xhr=1&league={league}")
            );
        }
        for league in ["267979", "180659"] {
            assert_eq!(
                feed_url("rugby", league).unwrap(),
                format!("https://cdn.espn.com/core/rugby/scoreboard?xhr=1&league={league}")
            );
        }
        for sport in ["football", "rugby"] {
            for invalid in ["", "unknown", "eng.1&league=other", "../nfl"] {
                assert_eq!(feed_url(sport, invalid).unwrap_err().0, "unsupported");
            }
        }
        assert_eq!(
            feed_url("cricket", "").unwrap(),
            "https://static.cricinfo.com/rss/livescores.xml"
        );
        assert!(feed_url("tennis", "").is_err());
    }
    #[test]
    fn cdn_empty_and_malformed_responses() {
        for sport in ["nfl", "football", "rugby"] {
            assert!(
                normalize(sport, &json!({"content":{"sbData":{"events":[]}}}))
                    .unwrap()
                    .is_empty()
            );
            for malformed in [
                json!({}),
                json!({"events":[]}),
                json!({"content":{"sbData":null}}),
                json!({"content":{"sbData":{"events":{}}}}),
            ] {
                assert!(normalize(sport, &malformed).is_err());
            }
        }
    }
    #[test]
    fn cdn_football_score_updates_preserve_identity() {
        let mut v = json!({"content":{"sbData":{"events":[{"id":"42","name":"A v B","date":"2026-09-14T19:00Z","status":{"type":{"state":"pre"}},"competitions":[{"status":{"type":{"state":"in","shortDetail":"27'"}},"competitors":[{"team":{"id":"a","displayName":"A"},"score":"0"},{"team":{"id":"b","displayName":"B"}}]}]}]}}});
        let before = normalize("football", &v).unwrap();
        assert_eq!(before[0]["id"], "football:42");
        assert_eq!(before[0]["state"], "live");
        assert_eq!(before[0]["detail"], "27'");
        assert!(before[0]["teams"][1]["score"].is_null());
        v["content"]["sbData"]["events"][0]["competitions"][0]["competitors"][0]["score"] =
            json!("1");
        let after = normalize("football", &v).unwrap();
        assert_eq!(after[0]["id"], before[0]["id"]);
        assert_eq!(after[0]["teams"][0]["score"], 1);
    }
    #[test]
    fn denied_access_does_not_request_authentication() {
        for status in [401, 403] {
            assert_eq!(
                response_error(status, Some("120"), now()),
                Some(("access-denied", "Provider denied access", Some(120)))
            );
        }
    }
    #[test]
    fn retry_after_headers() {
        let observed = chrono::DateTime::parse_from_rfc3339("2026-09-14T10:00:00.500Z")
            .unwrap()
            .to_utc();
        assert_eq!(retry_after(" 7200 ", observed), Some(7200));
        assert_eq!(
            retry_after("Mon, 14 Sep 2026 10:02:00 GMT", observed),
            Some(120)
        );
        assert_eq!(
            retry_after("Mon, 14 Sep 2026 09:00:00 GMT", observed),
            Some(0)
        );
        for invalid in ["", "-1", "1.5", "nonsense"] {
            assert_eq!(retry_after(invalid, observed), None);
        }
        for status in [429, 503] {
            assert_eq!(
                response_error(status, Some("7200"), observed).unwrap().2,
                Some(7200)
            );
        }
        assert_eq!(response_error(200, Some("7200"), observed), None);
        assert_eq!(response_error(429, Some("bad"), observed).unwrap().2, None);
    }
    #[test]
    fn espn_fixture() {
        let v = json!({"events":[{"id":"1","name":"A v B","status":{"type":{"state":"in"}},"competitions":[{"competitors":[{"team":{"id":"a","displayName":"A"},"score":"7"},{"team":{"id":"b","displayName":"B"},"score":"0"}]}]}]});
        let m = normalize("nfl", &json!({"content":{"sbData":v}})).unwrap();
        assert_eq!(m[0]["state"], "live");
        assert_eq!(m[0]["teams"][0]["score"], 7);
        assert!(normalize("football", &json!({})).is_err());
    }
    #[test]
    fn rugby_fixture() {
        let v = json!({"events":[{"id":"602507","name":"Wales vs France","competitions":[{"status":{"type":{"state":"post"}},"competitors":[{"team":{"id":"4","displayName":"Wales"},"score":"12"},{"team":{"id":"9","displayName":"France"},"score":"54"}]}]}]});
        let m = normalize("rugby", &json!({"content":{"sbData":v}})).unwrap();
        assert_eq!(m[0]["state"], "finished");
        assert_eq!(m[0]["teams"][1]["score"], 54);
        assert!(normalize("rugby", &json!({"response":[]})).is_err());
    }
    #[test]
    fn input_safety() {
        assert_eq!(text(&json!("a\nb")), "ab");
        assert!(normalize("tennis", &json!({})).is_err());
        assert!(number(&json!("oops")).is_null());
        assert!(number(&json!(-1)).is_null());
    }
}
