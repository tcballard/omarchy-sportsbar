# Verification — audit corrections, 14 September 2026

## Source identity

Runtime, tests and demo scripts tested at commit `ce6b4daacf6e981c91daa28bb8850a8270e77373`.
Its complete Git tree is `0e9b7e74749435578cd5d01574f244b61ad2c3b1`.
The following documentation-only commit pins README installation to that source
and records this evidence; it does not change runtime, tests or assets.

Bundle validation scripts match v0.4.0, commit
`a12e90568b7e8f28ca7ca92400009758da7eb43b`; installed routing/reference updates
are also present. Structural validity is not security certification or marketplace approval.

## Reproduced checks

Linux container, Rust 1.98.1 and PySide6 6.11.2 (offscreen/software).
The initial missing tool/dependency prerequisites were restored for this patch.

- `./tests/run`: manifest validator, 28 Node tests, 12 Rust tests, formatting;
  all pass. Official Omarchy validation is skipped when its executable is absent.
- `cargo clippy --offline --locked -- -D warnings`: pass; also added to CI.
- `python3 tests/qml_smoke.py`: pass, no QML warnings; uses explicit host doubles.
- Bundle `validate_plugin.py . --json --security`: valid, no structural warnings
  or security findings; process/build/CI capabilities still require manual review.
- Bundle `demo_preflight.py .`: pass with fixture-only warning. The portable
  runner does not claim desktop capture. Separate `demo/capture-live` is prepared.
- `bash -n demo/run demo/capture-live tests/run` and `git diff --check`: pass.
- Live-capture regression uses inert shell/renderer doubles to verify refusal
  outside demo mode, output protection and temporary-file cleanup. No actual
  desktop is captured by this test.

The scheduler regression uses the production methods and a controlled clock:
25 successful 12-second requests rotate across all four sports, including cricket.
Other regressions cover no overlap, disabled feeds, exact deadlines, two-hour
server waits, invalid/short waits, settings changes, late results and notification
spacing. Rust tests cover delay-seconds, a future standard HTTP date rounded up,
past dates, invalid headers and propagation from 429/503 responses.

Reviewed capabilities: README builds a manually selected immutable source;
Service.qml runs only the bounded feed helper and notify-send via argv;
workflow sudo/package installation runs in CI, not inside the plugin.

## Failures and limits

A rebuilt helper's real cricket request returned `offline` / `Feed unreachable
or timed out` in this container. No end-to-end provider success or latency claim
is made for this patch. TLS protections were not weakened. The controlled tests
verify error handling, not the external provider's current availability.

No Omarchy executable, compositor or real notification service is available here.
Official validation, exact host imports, discovery/enablement, saved settings,
bar orientations, monitors, focus, theme switching, live capture, real-process
cleanup, install/update/removal and notification delivery remain unrun. Follow
[desktop acceptance](DESKTOP-TEST.md). The live capture script is prepared, not
proven on Omarchy. Cooldowns are session memory and reset when the service exits.

## Historical evidence, not rerun

The pre-correction main commit `9da10bea93cb6a01d1d355f73c94f88b06be8182`
had successful CI, but its tests missed polling starvation. It is not evidence
for these corrections; consult this PR's CI for the delivered head.

The earlier 14 September cricket experiment observed 21 successful RSS samples
and two score changes detected about 57–60 seconds earlier than simulated
two-minute polling. No wicket was observed, and ground-to-desktop latency was
not established. Earlier direct endpoint checks found public ESPN rugby results
and fixtures. Those observations are not repeated live-provider acceptance.

Existing root preview and docs/portable-preview.png are historical captures of
production content with simulated host controls and fictional fixtures. They
remain labelled portable previews and are not live desktop evidence.
