# ESPN CDN migration — 14 September 2026

Runtime and tests: `d520fccbb3847226afe20ca842c5f0c19bee6757`, based on main
`b774c4329bff96beb27ee96ae266f4e332983891`. The subsequent documentation commit
updates endpoints, this evidence and the installation pin. No publication or
marketplace submission is part of this change.

## Change

NFL, football and rugby request `cdn.espn.com/core/{sport}/scoreboard?xhr=1`.
NFL uses `limit=50`; football and rugby pass the selected allowlisted `league`.
Parse the exact `content.sbData.events` array; missing/wrong-shaped wrappers
fail instead of reporting a successful empty feed. Preserve match/team IDs,
unknown scores, competition status precedence and the 30-second default.
401/403 now reports `access-denied`, retaining Retry-After handling.
Cricket continues to use the existing Cricinfo RSS adapter.

## Checks

Linux container; Rust 1.98.1. Cargo output kept outside the plugin tree.

- `CARGO_TARGET_DIR=/tmp/sportsbar-cdn-target ./tests/run`: pass, including
  manifest validation, 28 Node tests, 16 Rust tests and formatting.
- `cargo clippy --locked -- -D warnings`: pass.
- `cargo build --locked`: pass.
- Bundle `validate_plugin.py . --json --security`: valid, no structural warnings
  or security findings. Existing build/process/CI capability notices remain;
  this is advisory validation, not certification.
- `git diff --check`: pass.

Added tests exercise CDN routes and rejected league injection, empty versus
malformed wrappers, football score changes with stable IDs, unknown scores,
competition status precedence, and denied-access labels with retry waits.
Existing NFL/rugby parser fixtures now use the CDN wrapper. Scheduler and
notification regression tests remain unchanged and pass.

The first full test-run attempt rejected an in-tree Cargo build directory as
over the manifest scanner's safety limit. Moving build output outside the
checkout resolved this; the validator was not weakened.

## Live evidence and limits

Earlier direct Python HTTPS probes on 14 September around 19:30 UTC returned
HTTP 200 with 16 NFL events, one live Premier League event and two upcoming
Premiership rugby fixtures. They were endpoint research, not this Rust helper.
Observed Cache-Control max-age values were 120, 112 and 240 seconds respectively.
Polling every 30 seconds does not guarantee equally fresh upstream data.

The rebuilt Rust helper was then run serially for nfl, football, rugby and
cricket starting at 19:36:43 UTC. All returned `offline` / `Feed unreachable or
timed out`. The user has independently confirmed working cricket on the XPS.
The container-specific transport failure is unresolved; no end-to-end Rust
provider success or live alert latency is claimed. TLS verification and
redirect restrictions have not been weakened.

No Omarchy host, compositor or notification service is available. PySide6 is
absent locally, so QML smoke validation is left to existing CI; QML files are
unchanged. XPS feed connectivity, focused bar scores and actual notifications
remain required. Rebuild the helper as well as updating the plugin checkout.
