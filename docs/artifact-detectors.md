# Artifact Detectors

Each detector is explicit and testable. Detectors live in `crates/buildsweep-detectors`.

## Registry

`classify_entry_name(name)` returns `(ArtifactKind, SafetyClass, explanation)` or `None`.

## Per-ecosystem modules

- `js.rs` — Node.js / frontend artifacts
- `python.rs` — Python virtualenvs and caches
- `rust_eco.rs` — Cargo `target/`
- `java.rs` — Gradle/Maven build dirs
- `dotnet.rs` — `bin/`, `obj/`
- `flutter.rs` — `.dart_tool`, `build/`
- `macos.rs` — `Pods`, `DerivedData`
- `windows_eco.rs` — MSBuild output (review)
- `protected.rs` — Never-clean paths

## Adding a detector

1. Add rule to appropriate module
2. Add test in `tests/detectors.rs` or fixture
3. Document in this file

Unknown paths must not be classified as SAFE.
