#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/apps/desktop"
export BUILDSWEEP_FLAVOR=store
# Set your payment page (LemonSqueezy, Paddle, Stripe, Gumroad, etc.)
export BUILDSWEEP_PURCHASE_URL="${BUILDSWEEP_PURCHASE_URL:-https://buildsweep.app/buy}"
pnpm install
pnpm build:store
echo ""
echo "Store build ready:"
echo "  $ROOT/target/release/bundle/macos/BuildSweep.app"
echo "  Purchase URL baked in: $BUILDSWEEP_PURCHASE_URL"
echo ""
echo "After a customer pays, issue a license:"
echo "  cargo run -p license-signer -- sign ORDER-ID"
