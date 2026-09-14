mod cricinfo;
use serde_json::{json, Value};
use std::{env, io::Read, time::Duration};
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
            for e in array(&v["events"])? {
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
fn fetch(sport: &str, league: &str) -> Result<(Vec<Value>, u64), (&'static str, &'static str)> {
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
        .map_err(|_| ("failed", "Could not create HTTPS client"))?;
    let request = match sport {
        "nfl" => {
            client.get("https://site.api.espn.com/apis/site/v2/sports/football/nfl/scoreboard")
        }
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
                return Err(("unsupported", "Football competition is not supported"));
            }
            client.get(format!(
                "https://site.api.espn.com/apis/site/v2/sports/soccer/{league}/scoreboard"
            ))
        }
        "rugby" => {
            if !["267979", "180659"].contains(&league) {
                return Err(("unsupported", "Rugby competition is not supported"));
            }
            client.get(format!(
                "https://site.api.espn.com/apis/site/v2/sports/rugby/{league}/scoreboard"
            ))
        }
        "cricket" => client.get("https://static.cricinfo.com/rss/livescores.xml"),
        _ => return Err(("unsupported", "Unsupported sport")),
    };
    let response = request
        .send()
        .map_err(|_| ("offline", "Feed unreachable or timed out"))?;
    match response.status().as_u16() {
        401 | 403 => return Err(("unauthenticated", "Provider denied access")),
        429 => return Err(("rate-limited", "Provider quota reached; polling backed off")),
        200 => (),
        _ => return Err(("failed", "Provider returned an HTTP error")),
    }
    let mut bytes = Vec::new();
    response
        .take(MAX_BODY + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| ("failed", "Could not read feed"))?;
    if bytes.len() as u64 > MAX_BODY {
        return Err(("failed", "Feed exceeded response size limit"));
    }
    if sport == "cricket" {
        let xml =
            std::str::from_utf8(&bytes).map_err(|_| ("failed", "Cricket feed is not UTF-8"))?;
        return cricinfo::parse(xml)
            .map(|matches| (matches, cricinfo::POLL_INTERVAL_SEC))
            .map_err(|e| ("failed", e));
    }
    let v: Value =
        serde_json::from_slice(&bytes).map_err(|_| ("failed", "Feed returned invalid JSON"))?;
    normalize(sport, &v)
        .map(|matches| (matches, 30))
        .map_err(|e| ("failed", e))
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
        Err((state, message)) => {
            json!({"sport":sport,"state":state,"message":message,"matches":[]})
        }
    };
    println!("{output}");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn espn_fixture() {
        let v = json!({"events":[{"id":"1","name":"A v B","status":{"type":{"state":"in"}},"competitions":[{"competitors":[{"team":{"id":"a","displayName":"A"},"score":"7"},{"team":{"id":"b","displayName":"B"},"score":"0"}]}]}]});
        let m = normalize("nfl", &v).unwrap();
        assert_eq!(m[0]["state"], "live");
        assert_eq!(m[0]["teams"][0]["score"], 7);
        assert!(normalize("football", &json!({})).is_err());
    }
    #[test]
    fn rugby_fixture() {
        let v = json!({"events":[{"id":"602507","name":"Wales vs France","competitions":[{"status":{"type":{"state":"post"}},"competitors":[{"team":{"id":"4","displayName":"Wales"},"score":"12"},{"team":{"id":"9","displayName":"France"},"score":"54"}]}]}]});
        let m = normalize("rugby", &v).unwrap();
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
