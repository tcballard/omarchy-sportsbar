# v0.1.0 publication preparation

Status: prepared for review, not tagged, released or submitted.
Plugin ID: io.github.tcballard.sportsbar. Manifest and Cargo version: 0.1.0.
Preparation base: b774c4329bff96beb27ee96ae266f4e332983891 (merged audit fixes).
This PR changes documentation only; runtime remains identical to that base.

## Ready for review

- Release notes: RELEASE-NOTES-0.1.0.md.
- Marketplace title/body: MARKETPLACE-DRAFT.md; category Widgets, tags Bar/Quickshell.
- Pinned source installation, removal, placement and support routes in README.
- Existing labelled portable preview and dependency/asset provenance.
- Source-bound portable evidence plus the maintainer's limited desktop demo report.

## Acceptance before publication

- [ ] Record the XPS's full Omarchy revision and installed SportsBar revision together.
- [ ] With demo off, confirm successful live loading of all four enabled sports.
- [ ] Follow a live match and observe a real score change and cricket wicket alert;
  check no duplicates and mute/DND. Cricket runs alone intentionally do not alert.
- [ ] Verify saved settings, focus, theme changes, restart and disable/re-enable.
- [ ] Verify helper PATH, real process cleanup, install/update/removal and rollback
  on a disposable installation. Record retained settings and helper ownership.
- [ ] Capture a real panel screenshot with demo/capture-live, inspect it and record
  source/version. Keep the current image labelled portable until replaced.
- [ ] Confirm ownership/permission and each marketplace acknowledgment with Tom.

Use DESKTOP-TEST.md for the detailed walkthrough. User-reported demo display and
repositioning are already recorded; do not ask to repeat them unless the runtime
or host changes. The existing container's failed helper HTTPS probe is not a
successful production feed test.

## Final source and release boundary

After this PR and any acceptance fixes merge, record the final full main SHA,
require green CI and rerun the bundle release preflight against that clean source.
Recheck the marketplace form and existing requests for this repo/ID to avoid a
duplicate submission. Copy the draft body only after updating its acceptance notes.

Only then, with publication authorized, create an immutable annotated v0.1.0 tag
and publish the release/submission. No final tag or release asset checksum is
claimed now: source changes during acceptance would invalidate them. If shipping
archives or binaries, prepare source/release manifests, SPDX 2.3 SBOM and complete
SHA256SUMS coverage, and validate the downloaded assets against the final tag.
The helper currently requires an explicit source build; binary packaging is not
silently provided by marketplace installation.

Marketplace automated checks, maintainer approval and actual listing are separate
outcomes. A successful local preflight establishes none of those outcomes.
