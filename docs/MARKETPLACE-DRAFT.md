# Prepared issue: [Plugin]: SportsBar

Not submitted. Destination: https://github.com/omacom/omarchy-plugin-marketplace
Current submit-plugin.yml headings, category and tags checked 14 September 2026.
The body below is a draft; all owner acknowledgments await confirmation. Complete
PUBLICATION.md and recheck the final repository HEAD before submission.

---

### Repository URL

https://github.com/tcballard/omarchy-sportsbar

### Category

Widgets

### Tags

Bar, Quickshell

### Suggest a missing tag

_No response_

### Maintainer notes

SportsBar is a hosted bar-widget plus singleton service for NFL, football,
rugby and cricket. Follow teams, focus a score in the bar and receive desktop
score/wicket updates. Default polling is 30 seconds, serialized with fair
scheduling, server retry delays and failure backoff. No API keys are required.

Installation requires a separately built sportsbar-feed Rust helper on the
running shell's PATH, libnotify and coreutils. No helper is installed by plugin
discovery or enablement. Network access is limited by code to public ESPN and
ESPNcricinfo endpoints; the plugin launches the helper and notify-send with
bounded processes. Omarchy owns saved widget settings; helper removal is separate.

Portable tests and the maintainer's fictional-demo/placement check are recorded
in docs/VERIFICATION.md. Live feed/notification and full lifecycle acceptance
are still pending at draft time; do not treat this as a verified submission.
The root preview is clearly labelled as a portable Qt capture with fictional
fixtures and simulated host controls. It contains no live personal score data.

### Submission checklist

- [ ] The repository is public and contains installation and removal instructions.
- [ ] I have documented the plugin license and any external dependencies.
- [ ] I confirm that I own or have permission to submit this plugin and its preview assets.
- [ ] The plugin does not overwrite user configuration without explicit consent.
- [ ] I understand that approval is for listing and is not a security review.
