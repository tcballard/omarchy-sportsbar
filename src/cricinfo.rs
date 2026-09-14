//! ESPNcricinfo public RSS adapter. Strict title parsing: ambiguous formats fail closed.
use serde_json::{json, Value};
const FEED_HOSTS: &[&str] = &[
    "www.espncricinfo.com",
    "www.cricinfo.com",
    "static.espncricinfo.com",
    "static.cricinfo.com",
];
pub fn allowed_url(url: &reqwest::Url) -> bool {
    url.scheme() == "https"
        && FEED_HOSTS.contains(&url.host_str().unwrap_or(""))
        && url.path() == "/rss/livescores.xml"
        && url.username().is_empty()
        && url.password().is_none()
        && url.port().is_none()
        && url.query().is_none()
}
fn match_id(link: &str) -> Option<String> {
    let u = reqwest::Url::parse(link).ok()?;
    if !["http", "https"].contains(&u.scheme()) || !FEED_HOSTS.contains(&u.host_str()?) {
        return None;
    }
    u.path_segments()?.rev().find_map(|s| {
        let n = s.trim_end_matches(".html").rsplit('-').next()?;
        if !n.is_empty() && n.len() <= 16 && n.bytes().all(|b| b.is_ascii_digit()) {
            Some(n.into())
        } else {
            None
        }
    })
}
fn side(raw: &str) -> Result<(Value, Vec<Value>, Option<String>), &'static str> {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    let index = parts
        .iter()
        .position(|t| {
            t.trim_end_matches('*')
                .split('/')
                .next()
                .is_some_and(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
        })
        .unwrap_or(parts.len());
    let name = parts[..index]
        .join(" ")
        .trim_end_matches('*')
        .trim()
        .to_owned();
    if name.is_empty() || name.len() > 120 {
        return Err("Unsupported cricket team name");
    }
    let mut innings = Vec::new();
    let mut active = None;
    let score = parts[index..].join(" ");
    if !score.is_empty() {
        for (slot, value) in score.split('&').enumerate() {
            if slot > 1 {
                return Err("Unsupported cricket innings format");
            }
            let value = value.trim();
            let is_active = value.contains('*');
            let clean = value.replace('*', "");
            let mut words = clean.split_whitespace();
            let count = words.next().ok_or("Missing cricket score")?;
            let declared = count.ends_with('d');
            let count = count.trim_end_matches('d');
            let fields: Vec<&str> = count.split('/').collect();
            if fields.len() > 2 {
                return Err("Unsupported cricket score");
            }
            let runs = fields[0]
                .parse::<u32>()
                .map_err(|_| "Unsupported cricket runs")?;
            if runs > 10000 {
                return Err("Invalid cricket runs");
            }
            let wickets = if fields.len() == 2 {
                Some(
                    fields[1]
                        .parse::<u8>()
                        .map_err(|_| "Unsupported cricket wickets")?,
                )
            } else {
                None
            };
            if wickets.is_some_and(|w| w > 10) {
                return Err("Invalid cricket wickets");
            }
            let rest = words.collect::<Vec<_>>().join(" ");
            let overs = if rest.is_empty() {
                String::new()
            } else {
                let token = rest
                    .strip_prefix('(')
                    .ok_or("Unsupported cricket score suffix")?
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_end_matches(')');
                if token.is_empty() || !token.chars().all(|c| c.is_ascii_digit() || c == '.') {
                    return Err("Unsupported cricket overs");
                }
                token.to_owned()
            };
            let id = format!("{name} Inning {}", slot + 1);
            if is_active {
                if active.is_some() {
                    return Err("Ambiguous batting innings");
                }
                active = Some(id.clone())
            }
            innings.push(
                json!({"id":id,"runs":runs,"wickets":wickets,"overs":overs,"declared":declared}),
            );
        }
    }
    Ok((
        json!({"id":name.to_lowercase(),"name":name,"score":null}),
        innings,
        active,
    ))
}
pub fn parse(xml: &str) -> Result<Vec<Value>, &'static str> {
    if xml.len() > 2 * 1024 * 1024 {
        return Err("Cricket RSS exceeds response limit");
    }
    let doc = roxmltree::Document::parse(xml).map_err(|_| "Invalid cricket RSS XML")?;
    let root = doc.root_element();
    if !root.has_tag_name("rss") {
        return Err("Expected cricket RSS feed");
    }
    let channel = root
        .children()
        .find(|n| n.has_tag_name("channel"))
        .ok_or("Missing RSS channel")?;
    let mut result = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for item in channel.children().filter(|n| n.has_tag_name("item")) {
        if result.len() >= 500 {
            return Err("Too many cricket matches");
        }
        let field = |name| {
            item.children()
                .find(|n| n.has_tag_name(name))
                .and_then(|n| n.text())
                .unwrap_or("")
                .trim()
        };
        let title = field("title");
        if title.len() > 1000 {
            return Err("Cricket title exceeds limit");
        }
        let (left, right) = title
            .split_once(" v ")
            .ok_or("Unsupported cricket RSS title; scores withheld")?;
        let (a, mut innings, active_a) = side(left)?;
        let (b, other, active_b) = side(right)?;
        if active_a.is_some() && active_b.is_some() {
            return Err("Ambiguous batting team");
        }
        innings.extend(other);
        let active = active_a.or(active_b);
        let id = match_id(field("link")).ok_or("Missing cricket match identity")?;
        if !seen.insert(id.clone()) {
            return Err("Duplicate cricket match identity");
        }
        let description = field("description");
        // Absence of the batting marker does not establish that a match is finished.
        let lower = description.to_lowercase();
        let finished = [
            " won by ",
            "match drawn",
            "match tied",
            "no result",
            "match abandoned",
        ]
        .iter()
        .any(|s| lower.contains(s));
        let state = if finished {
            "finished"
        } else if active.is_some() {
            "live"
        } else {
            "unknown"
        };
        result.push(json!({"id":format!("cricket:espncricinfo:{id}"),"sport":"cricket","name":format!("{} v {}",a["name"].as_str().unwrap(),b["name"].as_str().unwrap()),"state":state,"detail":description.chars().take(300).collect::<String>(),"start":"","teams":[a,b],"innings":innings,"activeInningsId":active,"source":"espncricinfo-rss"}));
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn rss(title: &str) -> String {
        format!("<rss><channel><item><title>{title}</title><link>https://www.espncricinfo.com/ci/engine/match/12345.html</link><description>Live cricket</description></item></channel></rss>")
    }
    #[test]
    fn live_two_teams() {
        let m = parse(&rss("England 186/4 * v Australia 228")).unwrap();
        assert_eq!(m[0]["teams"][0]["name"], "England");
        assert_eq!(m[0]["innings"][0]["wickets"], 4);
        assert!(m[0]["innings"][1]["wickets"].is_null());
        assert_eq!(m[0]["activeInningsId"], "England Inning 1");
    }
    #[test]
    fn second_innings_and_entities() {
        let m = parse(&rss("England 200 &amp; 20/1 * v Australia 350/8d")).unwrap();
        assert_eq!(m[0]["activeInningsId"], "England Inning 2");
        assert_eq!(m[0]["innings"].as_array().unwrap().len(), 3);
    }
    #[test]
    fn absent_marker_is_unknown() {
        assert_eq!(
            parse(&rss("England 200 v Australia 228")).unwrap()[0]["state"],
            "unknown"
        );
    }
    #[test]
    fn cdata_overs() {
        let m = parse(&rss("<![CDATA[England 186/4 (42.3 ov) * v Australia]]>")).unwrap();
        assert_eq!(m[0]["innings"][0]["overs"], "42.3");
    }
    #[test]
    fn rejects_unknown_formats() {
        for title in [
            "England vs Australia",
            "England 20/12 * v Australia",
            "England 20/1 xyz v Australia",
            "England 20/1 * v Australia 10/1 *",
        ] {
            assert!(parse(&rss(title)).is_err(), "{title}")
        }
    }
    #[test]
    fn rejects_html_and_dtd() {
        assert!(parse("<html/>").is_err());
        assert!(parse(
            "<!DOCTYPE rss [<!ENTITY x SYSTEM 'file:///etc/passwd'>]><rss><channel/></rss>"
        )
        .is_err());
    }
    #[test]
    fn redirect_allowlist() {
        assert!(allowed_url(
            &reqwest::Url::parse("https://www.cricinfo.com/rss/livescores.xml").unwrap()
        ));
        for u in [
            "http://www.cricinfo.com/rss/livescores.xml",
            "https://evil.test/rss/livescores.xml",
            "https://www.cricinfo.com/other",
        ] {
            assert!(!allowed_url(&reqwest::Url::parse(u).unwrap()))
        }
    }
}

// RSS ttl is expressed in minutes. Never poll more often than two minutes.
pub fn poll_interval(xml: &str) -> u64 {
    roxmltree::Document::parse(xml)
        .ok()
        .and_then(|d| {
            d.descendants()
                .find(|n| n.has_tag_name("ttl"))
                .and_then(|n| n.text())
                .and_then(|s| s.trim().parse::<u64>().ok())
        })
        .unwrap_or(2)
        .saturating_mul(60)
        .clamp(120, 86400)
}
#[cfg(test)]
mod recorded_feed {
    use super::*;
    #[test]
    fn real_public_response() {
        let xml = include_str!("../tests/fixtures/cricinfo-2026-09-14.xml");
        let matches = parse(xml).unwrap();
        assert_eq!(matches.len(), 2);
        assert_eq!(
            matches[0]["activeInningsId"],
            "United Arab Emirates Inning 1"
        );
        assert_eq!(matches[0]["innings"][1]["wickets"], 4);
        assert_eq!(matches[1]["state"], "unknown");
        assert_eq!(poll_interval(xml), 120);
        assert_eq!(
            poll_interval("<rss><channel><ttl>10</ttl></channel></rss>"),
            600
        );
    }
}
