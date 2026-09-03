# BuildSweep

Find the wasted space hiding inside your development projects.

BuildSweep is a minimal, cross-platform desktop utility for developers on **macOS** and **Windows**. It scans project directories, identifies regeneratable development artifacts, calculates reclaimable storage, and safely moves selected items to Trash / Recycle Bin.

## What BuildSweep is

- A focused developer project waste analyzer and cleanup tool
- Local-first, offline-first, no account required
- Safety-first: unknown files are never auto-classified as safe

## What BuildSweep is NOT

- Not a generic Mac/Windows cleaner
- Not a RAM optimizer, duplicate finder, or background daemon
- No telemetry, analytics, or cloud backend

## Supported artifact types (v1)

| Ecosystem | Artifacts |
|-----------|-----------|
| JavaScript/TypeScript | `node_modules`, `.next`, `.nuxt`, `dist`, `coverage`, `.turbo` |
| Python | `.venv`, `venv`, `__pycache__`, `.pytest_cache`, `.mypy_cache` |
| Rust | `target` |
| Java/Gradle | `target`, `.gradle` |
| .NET | `bin`, `obj` |
| Flutter | `.dart_tool`, `build` |
| macOS | `Pods`, Xcode DerivedData |
| Windows | MSBuild `bin`/`obj` (with project context) |

## Privacy model

- No network requests for scanning or cleanup
- No telemetry or crash reporting
- Cleanup manifest stored locally only (append-only JSONL)
- License verification is fully offline (Ed25519)

## Safety model

Every artifact receives a classification: **SAFE**, **REVIEW**, **PROTECTED**, or **UNKNOWN**.

- Only SAFE and REVIEW are cleanup-eligible
- UNKNOWN and PROTECTED are never offered for cleanup
- All paths are revalidated immediately before moving to Trash
- Files are never permanently deleted in normal operation

See [docs/safety-model.md](docs/safety-model.md) for details.

## Development setup

### Prerequisites

- Rust 1.77+
- Node.js 18+
- pnpm

### Commands

```bash
# Install frontend dependencies
cd apps/desktop && pnpm install

# Run tests
cargo test --workspace

# Run in development
cd apps/desktop && pnpm tauri dev

# Build production app
cd apps/desktop && pnpm tauri build
```

### Dev Pro mode

Set `BUILDSWEEP_DEV_PRO=1` to unlock Pro features during development.

### License signing (publishers only)

```bash
cargo run -p license-signer -- generate
cargo run -p license-signer -- sign LIC-001
```

Private keys must never be committed.

## Project structure

```
buildsweep/
  apps/desktop/          # Tauri + React UI
  crates/
    buildsweep-core/     # Domain models, waste score
    buildsweep-detectors/# Artifact detection rules
    buildsweep-scanner/  # Project discovery, scanning
    buildsweep-cleaner/  # Path safety, trash, manifest
    buildsweep-license/  # Offline license verification
  tests/fixtures/        # Fixture projects for tests
  docs/                  # Architecture and safety docs
```

## Licensing

- **Free:** unlimited scanning and analysis
- **Pro ($7.99 lifetime):** batch cleanup, presets, exclusions, cleanup history

## Platforms

- macOS 10.15+
- Windows 10+

Linux is not supported in v1.
