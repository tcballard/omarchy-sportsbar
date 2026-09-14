# SportsBar design

- Identity: `io.github.tcballard.sportsbar`; display name SportsBar; development preview 0.1.0.
- User problem: follow teams and receive match events without keeping a browser open. Initial sports: NFL, rugby, football, cricket.
- Entry points: `BarWidget.qml` (`bar-widget`) and `Service.qml` (`service`). The widget hosts the private `SportsPanel.qml`; there is no standalone process/window for UI.
- UI-local state: panel tab, sport filter, team search and keyboard focus.
- Singleton state: current feed snapshots, per-feed deadlines and exponential backoff, a bounded notification queue, score/wicket high watermarks and transient mute. Historical state prunes after 48 hours. Exactly one helper at a time; a generation invalidates results after changed settings. Polling continues when the popout is closed.
- Focus: an inline focused event ID and compact fixture label, independent of polling/alert configuration. No automatic switch away from a missing focused match. Private popout targets 440 × 520 logical pixels, clamped to the screen.
- Durable state: inline widget settings in Omarchy's shell.json, updated via the own-entry facade. At most 40 favourite team names, scoped by sport. No deep merge or separate settings file.
- Dependencies: Qt Quick/Quickshell and Omarchy Ui/Commons; purpose-built Rust `sportsbar-feed`; coreutils timeout; libnotify. No elevated actions or dynamically downloaded executables. The helper is built/installed explicitly.
- Network: fixed HTTPS endpoints at site.api.espn.com, v1.rugby.api-sports.io and static.cricinfo.com/rss/livescores.xml. Only exact HTTPS live-scores RSS paths on allowlisted Cricinfo hosts may redirect (at most two hops), 5-second connect and 12-second request timeout; 2 MiB per response; one cricket RSS request per refresh; 55-second process watchdog. Provider response URLs/errors and keys are not passed to QML.
- Credentials: helper inherits only the rugby credential variable it reads. No key UI or copying from another program's credential store.
- IPC: `io.github.tcballard.sportsbar refresh` respects polling deadlines; `status` returns a bounded summary. The shell's own summon/hide routes handle the bar popout.
- Failure states: missing helper, offline, HTTP/authentication/quota failures, unsupported competition/RSS score formats and malformed data. Last-good scores survive with stale labels. First successful snapshot after a failure is silent. No fake live-data fallback.
- Events: normalized match identity, stable team identity, and per-innings identity. No score-to-event-type inference. Missing numeric data never becomes zero. New innings establishes a baseline. Notification arguments are separate argv elements; remote markup/control characters are stripped. Host notification policy controls DND.
- Lifecycle: initial snapshot/restart/configuration changes establish a new baseline. Disable destroys the singleton and stops owned processes/timers. No permanent background daemon is installed.
- Deferred: multiple simultaneous football leagues, complete team catalogue and provider IDs in saved favourites, ball-by-ball scorer detail, provider-account UI, quiet-hour schedules, paid-feed production verification, marketplace submission and package distribution.

- Cricket RSS uses score-slot continuity and batting markers for conservative wicket detection, not authoritative innings identity. Honour RSS TTL with a two-minute minimum. No inferred wickets from bare run totals; missing status is unknown, not scheduled/final.
