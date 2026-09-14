# Dependencies and preview provenance

SportsBar source is MIT licensed; LICENSE identifies Tom Ballard as copyright
holder. Third-party dependencies retain their own licences; the project licence
does not relicense them or the providers' data.

Direct Rust dependencies are reqwest (HTTPS), serde_json (JSON), chrono (time)
and roxmltree (RSS/XML). Cargo.lock pins the full dependency graph. Crate licence
metadata and any distribution notices must be included in the release SBOM and
checked against the built graph before distributing compiled helper binaries.
No prebuilt helper is included in this publication-preparation PR.

Runtime host dependencies: Omarchy's Qt/Quickshell shell, coreutils timeout and
libnotify's notify-send. Development additionally uses Node, Python, jq and
PySide6 6.11.2 for tests. Python and Node are not plugin runtime dependencies.

preview.png and docs/portable-preview.png were rendered from SportsContent.qml
using tests/qml_smoke.py and committed fictional demo/fixtures/matches.json.
The harness simulates host controls and labels the capture accordingly. No
third-party screenshot or team-logo assets are included. The existing files are
historical portable captures; a live screenshot has not yet been supplied.

The README's external Omarchy badge is pinned to its upstream SVG revision and
is not claimed as SportsBar artwork. ESPN/ESPNcricinfo retain their data rights;
public accessibility does not imply a redistribution licence or service guarantee.
