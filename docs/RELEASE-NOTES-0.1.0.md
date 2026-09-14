# SportsBar v0.1.0 — prepared release notes

Status: unpublished candidate. Use these notes only after the publication
checklist is completed and the final merged source is recorded.

Follow your teams from the Omarchy bar, focus one event's score, and receive
desktop score or wicket updates without keeping a sports website open.

- NFL, football and rugby through public ESPN scoreboards; cricket through
  ESPNcricinfo's public live-scores RSS. No account or API key is required.
- Saved team choices and focused event, with a compact scores/teams/settings panel.
- Default 30-second polling, fair selection of overdue feeds, no overlapping
  requests, server Retry-After support and exponential error backoff.
- Conservative wicket detection, duplicate suppression, mute controls and stale
  score retention. First observations and recovery establish silent baselines.
- Fictional demo mode for trying the interface without live alerts.

## Installation and permissions

Requires Omarchy Quattro, a separately built `sportsbar-feed` Rust helper on the
shell's PATH, libnotify and coreutils. See the root README for the pinned source
installation. Adding the plugin alone does not build or install the helper.

Runs with normal user permissions inside the unsandboxed Omarchy shell. It
launches bounded helper and notification processes, reads public HTTPS feeds,
and saves widget preferences through Omarchy. Match history and retry cooldowns
are session-only. The helper must be removed separately on uninstall.

## Evidence and limitations

40 unit/regression tests, formatting, strict Clippy and simulated QML checks
passed for the audit-correction runtime. The maintainer reports successful demo
display and bar repositioning on an XPS. Exact installed compatibility and full
live lifecycle remain unverified; see docs/VERIFICATION.md and PUBLICATION.md.

Polling is not push delivery. Slow responses, provider caching and retry waits
extend update times; a 30-second alert guarantee is not offered. Cricket metadata
can omit overs, innings identity, wickets or final status; uncertain changes may
be suppressed. Only one football and one rugby competition are fetched at a time.
Rugby choices initially cover English Premiership and Six Nations. Provider
interfaces and availability are not contractual.

The current root preview is a labelled portable Qt capture with fictional data
and simulated host controls, not a photograph of the tested XPS session.
