# Development

## Setup

```bash
cd buildsweep
cd apps/desktop && pnpm install
```

## Run

```bash
cd apps/desktop
pnpm tauri dev
```

## Test

```bash
cargo test --workspace
```

Security-focused tests:

```bash
cargo test -p buildsweep-cleaner --test security
```

## Build

```bash
cd apps/desktop
pnpm tauri build
```

## Signing (release)

Signing configuration is kept outside the repository. See Tauri docs for:

- macOS: code signing + notarization
- Windows: Authenticode signing

Do not commit certificates or private keys.
