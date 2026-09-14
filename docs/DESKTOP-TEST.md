# Demo and desktop acceptance

Portable component checks do not establish installed Omarchy compatibility.
Record `omarchy-version`, `git rev-parse HEAD`, and results for each step below.
Use the built-in Omarchy bar. No installed version is currently certified.

## Reproducible portable demo

Run `demo/run` for the committed fictional fixture. Run
`demo/run --portable` to render production QML with simulated host controls.
This uses a temporary directory for the host doubles and removes it on exit.
It does not change the running shell. PySide6 6.11.2 is required.
The root preview remains a labelled portable capture, not a desktop screenshot.

## Live capture without desktop mutation

1. Install the reviewed revision using the README. Save your current SportsBar
   settings before manually enabling Fictional demo in Alerts & feeds.
2. Open the Scores panel on an empty workspace. Check the DEMO label and four
   fictional matches. Ensure unrelated windows and notifications are outside
   the capture rectangle.
3. Run `demo/capture-live /absolute/new-preview.png "X,Y WIDTHxHEIGHT"`, replacing
   the rectangle with the panel's screen coordinates. Dependencies: the running
   Omarchy shell, jq, grim and coreutils. The script uses read-only IPC to require
   a completed demo snapshot before and after capture; it refuses existing files.
4. Inspect the image yourself: IPC proves service state, not that your rectangle
   contains the panel. Record source revision, Omarchy version and capture date.
5. Restore the settings and workspace you changed manually. The capture script
   changes neither settings nor shell state and needs no desktop backup/restore.

The IPC invocation follows Omarchy v4.0.3's `bin/omarchy-shell` contract.
This live capture path is prepared but has not been exercised on Omarchy.
The bundle's fixture-only warning for `demo/run` remains accurate: that runner
is deliberately portable; live capture is a separate, explicitly selected script.

## Acceptance sequence

1. Run `omarchy plugin validate "$PWD"`; add disabled, enable, and place the widget.
2. Follow teams and focus a match. Restart the shell and verify saved choices.
3. Open/close the panel; exercise Tab, Enter and Escape, both bar orientations,
   small screens, multiple monitors and theme switching. Confirm one poller.
4. Try demo, then restore real feeds. Verify all four providers load without keys,
   football/rugby selection, stale status, and request-start intervals. Requests
   are serialized: slow feeds can extend intervals beyond the 30-second default.
5. Observe real score and wicket changes; check notification delivery, mute/DND,
   spacing and duplicates. Provider timestamps do not establish delivery latency.
6. Disconnect/reconnect, suspend/resume, reload QML, disable/re-enable. Confirm
   no orphan helper, no overlapping requests and silent recovery baselines.
7. Test an update on a disposable installation, keeping helper and QML on the
   same reviewed revision. Remove using the README and verify polling stops.

Missing helper, malformed response, timeout and notification-service failure
also need real-process checks. Record failures rather than assuming a portable
test covers them. Do not mark this list passed from a screenshot alone.
