# Verification — 14 September 2026

Development preview, Linux container. No live Omarchy session or paid-provider credentials were available.

## Passed

- Bundle v0.4.0 candidate validator: valid manifest, entry points and paths; zero warnings. Advisory security output identifies QML process execution for manual review, with no flagged findings. This is not marketplace approval.
- `./tests/run`: generated manifest validator, 20 JavaScript alert/focus tests, 11 Rust normalization/RSS tests, Rust formatting. JavaScript cases cover initial load, individual and aggregated wickets, new innings, runs-only updates, duplicate snapshots, score/wicket corrections, muted polling, outage recovery, team/sport filtering, final score changes and unknown score values.
- `cargo clippy --offline --locked -- -D warnings`: passed.
- `python3 tests/qml_smoke.py --screenshot` with PySide6 6.11.2, offscreen/software: loads all production QML entry/content files against explicit host doubles; tests team selection signals, focus changes preserving live alert state, demo data, no demo alerts, late response invalidation, favourite filtering, last-good retention and stale/offline state. No QML warnings. Renders the actual SportsContent, with simulated host controls. Screenshot inspected for layout/clipping at 480 × 620.
- Rugby still requires credentials. Cricket no longer reads a key; its recorded real RSS response passes parsing, including batting-side selection and the two-minute TTL.
- Direct curl retrieval reached the public RSS through the observed www.espncricinfo.com → www.cricinfo.com → static.cricinfo.com redirect chain. The production Rust helper returned offline in this container; real helper HTTPS and notification delivery remain target-desktop checks.
- Public ESPN NFL and Premier League JSON endpoints opened successfully through web retrieval. The execution container could not directly reach provider hosts; this is not an end-to-end helper network test.

## Required on the target desktop

1. Build/install helper into the shell process's PATH. Run installed `omarchy-version`; record it with platform and date.
2. Validate and enable the plugin on Omarchy's built-in bar. Confirm own-service resolution and settings persistence across reload.
3. Open/close panel, keyboard Tab/Enter/Escape, narrow screens, horizontal/vertical bar and multiple monitors; confirm only one poller.
4. Enable fictional demo and confirm no network/notification delivery. Disable demo before following real matches.
5. Supply provider keys through session environment. Confirm rugby schema, coverage and quota. Cricket needs no key: confirm public RSS loading and honour its TTL. Use an appropriate rugby plan before 60-second polling.
6. Observe a real match score change and cricket wicket, checking delivery latency and absence of duplicate alerts. Test DND/session mute and notification failure reporting.
7. Disconnect/reconnect network and suspend/resume. Confirm stale display, backoff and silent re-baseline rather than catch-up alerts.
8. Disable/remove plugin and confirm child process cleanup, no polling, and no duplicate service after re-enable.

Production readiness, exact supported installed Omarchy versions and marketplace distribution remain unverified. The preview is suitable for review and controlled testing, not a claim of dependable live match coverage.

- Prior GitHub CI failed on missing `libEGL.so.1`. Workflow now installs libegl1/libopengl0 before the Qt smoke test.
