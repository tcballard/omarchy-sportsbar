# SportsBar

<p>
<a href="LICENSE"><img alt="MIT licence" height="20" src="https://img.shields.io/badge/license-MIT-blue"></a>
<a href="https://github.com/tcballard/omarchy-badges"><img alt="Built for Omarchy: Plugin" height="20" src="https://raw.githubusercontent.com/tcballard/omarchy-badges/75975e5b5bf75e7ede3764bcd2950046f7abfe2c/badges/v1/omarchy-plugin.svg"></a>
</p>

Follow your teams from the Omarchy bar. See scores and get desktop notifications when a score changes or a wicket falls, without keeping a sports website open.

**First development preview.** Portable tests pass; live Omarchy integration and authenticated rugby feeds are not yet verified. This is not a published marketplace release.

![Portable Qt preview with fictional scores and simulated host controls](docs/portable-preview.png)

## What works in this preview

- NFL, football, rugby and cricket feed adapters, sharing one background service across monitors.
- Search teams in current fixtures, follow/unfollow them, or enter an exact team name. Names are scoped by sport and saved in the widget's inline `shell.json` settings.
- Compact popout and horizontal/vertical bar display. Choose **Focus in bar** on a match to pin its score, e.g. `ENG 186/4 · 42.3 ov` or `ARS 2 – LIV 1`. Focus is saved; changing it leaves all other followed-team alerts running. Final results stay pinned while present in the feed. Missing focused events show unavailable rather than silently switching matches. **Clear bar focus** restores the first-live-match display.
- Per-feed errors and retained stale scores.
- Optional score, wicket, match-start and match-finish notifications. Both sides of a followed match count: get wickets whether your team is batting or bowling.
- Session mute, individual alert switches, provider switches and football competition selection.
- Fictional demo mode: no network requests or notifications.

Score changes are described as **score updates**, not inferred touchdowns, tries or individual goals. Multiple wickets between polls produce one aggregate notification. The first observation, a new innings, restart, settings change or recovery after an error establishes a baseline without historical alerts. Corrected scores that return to an already observed high watermark do not alert again; this favours avoiding duplicates over notifying every reversal. Cricket runs alone do not alert.

## Feeds and limits

| Sport | Adapter | Setup | Polling |
| --- | --- | --- | --- |
| NFL | ESPN public website scoreboard | No key | 60 seconds |
| Football | ESPN public website scoreboard | No key; choose one competition | 60 seconds |
| Rugby | API-Sports Rugby games for today's UTC date | `SPORTSBAR_RUGBY_KEY` | 20 minutes, or 60 seconds in fast mode |
| Cricket | ESPNcricinfo public live-scores RSS | No key or account | 30 seconds |

Alerts arrive **after the provider updates and the next successful poll**. These are not push feeds or guaranteed instant alerts. Cricket defaults to 30-second polling, measured from request start; one request refreshes all cricket matches in the feed. This deliberately uses the product default rather than the RSS TTL hint. The 14 September live-feed experiment observed two score changes 57–60 seconds earlier than a simulated two-minute schedule; it did not establish wicket-alert or ground-to-desktop latency. Rugby remains on an optional keyed adapter: standard mode is 20 minutes, or 60 seconds with sufficient quota. HTTP errors back off; manual refresh respects the current deadline.

The ESPNcricinfo public RSS response was retrieved on 14 September 2026 and is covered by a recorded-response test. It supplies team names, scores, wickets when present and a `*` batting-side marker. Overs, authoritative innings IDs, upcoming-match state and result text are not guaranteed. Missing fields stay unknown; a bare runs total is not assumed to mean all out. Cricket wicket alerts require consecutive live snapshots of the same batting score slot with no run reset. These are best-effort score-change detections: compressed innings, omitted markers and missing final-result updates may suppress notifications. The feed's schema and update latency are not contractual. Unsupported title formats report a feed error and retain stale scores rather than inventing data.

NFL and Premier League endpoints were also inspected publicly. Rugby still requires your own API-Sports account; no rugby key was available for a real response test. The product target is no-account public feeds by default; a no-key rugby replacement remains outstanding. No BBC scraping is used.

The initial scope is current scoreboards, not every competition, historic results, standings or a complete searchable team directory. Football fetches one selected competition at a time (default Premier League); teams playing outside it will not appear. Rugby coverage depends on the API-Sports plan. Cricket follows exact provider team names. Additional provider adapters can reuse the normalized match contract and alert engine.

Sources checked 14 September 2026:

- [ESPN NFL scoreboard](https://site.api.espn.com/apis/site/v2/sports/football/nfl/scoreboard)
- [ESPN Premier League scoreboard](https://site.api.espn.com/apis/site/v2/sports/soccer/eng.1/scoreboard)
- [API-Sports Rugby documentation](https://api-sports.io/documentation/rugby/v1)
- [ESPNcricinfo public live-scores RSS](https://static.cricinfo.com/rss/livescores.xml)

## Build and install for testing

Requires Omarchy's Quattro shell, Rust/Cargo to build the helper, `libnotify` (`notify-send`) and coreutils (`/usr/bin/timeout`). The helper uses HTTPS through rustls and does not need Python or Node at runtime. No install hooks run when adding the plugin.

Clone the repository and enter its directory:

```bash
git clone https://github.com/tcballard/omarchy-sportsbar.git
cd omarchy-sportsbar
```

Build and install the helper explicitly:

```bash
CARGO_TARGET_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/sportsbar-build" cargo install --path . --locked
```

Ensure `sportsbar-feed` is on the **Omarchy shell process's PATH**, not only your terminal's PATH. Test it in a terminal:

```bash
sportsbar-feed --demo
sportsbar-feed nfl
```

Install the plugin from its repository:

```bash
omarchy plugin add https://github.com/tcballard/omarchy-sportsbar --yes
omarchy plugin enable io.github.tcballard.sportsbar --yes
```

The display name is **SportsBar**; the repository is **omarchy-sportsbar**. This first published preview uses plugin ID `io.github.tcballard.sportsbar`, helper `sportsbar-feed` and the `SPORTSBAR_*` environment variables below. If you manually installed the earlier unpublished OmaSports preview, remove `io.github.tcballard.omasports` and its `omasports-feed` helper before enabling this one. Transfer any favourite settings you want to keep.

Click **Sports → Teams**, select a sport and follow your teams. In **Alerts & feeds**, choose a football competition, alert categories and polling speed. For a safe visual trial, enable **Fictional demo** there first. This does not set your favourites to the fictional teams.

Left click opens the panel; right click mutes/unmutes notifications for the session. Tab/Shift+Tab move between controls; Enter/Space activate buttons; Escape closes the panel.

## Provider setup

**Cricket, NFL and football need no API keys.** The former `SPORTSBAR_CRICKET_KEY` is no longer read and can be removed from your session configuration.

Only the current optional rugby adapter uses a key. Supply `SPORTSBAR_RUGBY_KEY` from [API-Sports](https://api-sports.io/) through your login/session environment so the running shell inherits it. Re-login after changing that environment; exporting it in a terminal does not update an already running shell. Disable the rugby feed in **Alerts & feeds** if you want to use only the no-key providers.

Do not put keys in `shell.json`, the repository, screenshots or issues. Only the Rust helper reads the rugby variable; it never emits it in output. Keys are not entered or displayed in the panel.

## Compatibility

Targets the installed Omarchy Quattro shell's hosted `service` + `bar-widget` contract. Installed system versions are reported by `omarchy-version`; ISO and Quickshell engine versions are separate. **No installed Omarchy version has been tested yet**, so no compatibility-range badge is claimed. An ordinary third-party replacement bar can lack access to the widget's service; use Omarchy's built-in bar for the initial test.

## Development and evidence

```bash
./tests/run
# Optional Qt component/lifecycle checks (requires PySide6):
python3 tests/qml_smoke.py --screenshot
```

The Qt test uses explicit doubles for Omarchy/Quickshell host components. It renders the actual `SportsContent.qml` and exercises actual service methods, but does not validate real layer-shell focus, process launch, D-Bus notifications, service injection or theme switching. See [verification](docs/VERIFICATION.md) for the remaining live checks and [design](docs/DESIGN.md) for state ownership and dependencies.

This project was scaffolded with the local **Build Omarchy Plugins v0.4.0 release candidate**, commit `66d894eca17b0eb863bbae5926a9b122fa5a803d`. The remote `v0.4.0` tag was not available when this build began. Applied bundle skills: design, scaffold, bar-widget, service-ipc, QML patterns and test. No bundle files were modified.

Suggested repository topics: `omarchy`, `omarchy-plugin`, `sports`, `cricket`, `live-scores`.

## Removal

Disable/remove the plugin with `omarchy plugin remove io.github.tcballard.sportsbar`. The helper is separately owned: remove it with `cargo uninstall sportsbar-feed` if no longer needed. Remove provider environment variables from your session configuration yourself. Match history is session-only; the plugin creates no separate credential or cache files. Omarchy owns the widget settings.

MIT licensed. Data providers retain their own data rights and terms.
