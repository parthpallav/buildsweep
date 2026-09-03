# Architecture

BuildSweep uses a thin UI shell over a Rust core engine.

```
React UI (apps/desktop)
        |
        v
Tauri Commands (IPC)
        |
        v
Rust Core (crates/)
  +-- buildsweep-scanner    Project discovery, traversal, activity
  +-- buildsweep-detectors  Artifact classification rules
  +-- buildsweep-cleaner    Path safety, cleanup, trash, manifest
  +-- buildsweep-license    Offline Ed25519 verification
  +-- buildsweep-core       Shared domain types and waste score
```

## Scan lifecycle

1. User selects folders in UI
2. `start_scan` spawns a Tokio task with cancellation token
3. Scanner discovers projects, detects artifacts, calculates sizes
4. Progress events stream to UI via Tauri events
5. On completion, scan result stored in app state; worker terminates

## Cleanup lifecycle

1. User selects artifacts in UI
2. `build_cleanup_plan` validates selections in Rust
3. Cleanup review screen shows exact plan
4. `execute_cleanup` revalidates paths, moves to Trash/Recycle Bin
5. Manifest and history (Pro) appended locally

## Design principles

- All filesystem-sensitive logic in Rust
- No background daemon
- No persistent scanner worker after job completion
- Structured errors returned to UI
