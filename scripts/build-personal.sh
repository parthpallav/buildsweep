#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/apps/desktop"
export BUILDSWEEP_FLAVOR=personal
pnpm install
pnpm build:personal
echo ""
echo "Personal build ready:"
echo "  $ROOT/target/release/bundle/macos/BuildSweep Personal.app"
echo ""
echo "Install: cp -R \"$ROOT/target/release/bundle/macos/BuildSweep Personal.app\" /Applications/"
