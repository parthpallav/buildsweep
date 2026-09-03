# Licensing

BuildSweep uses offline Ed25519 signed licenses.

## Tiers

### Free
- Unlimited scanning and analysis
- Single-project cleanup per operation

### Pro ($7.99 lifetime)
- Batch cleanup across projects
- Cleanup presets
- Exclusions
- Cleanup history

## Verification

- App embeds public key only (`EMBEDDED_PUBLIC_KEY_B64`)
- Private key used by `tools/license-signer` (never shipped)
- No online activation or account required

## License format

```json
{
  "payload": {
    "tier": "pro",
    "license_id": "LIC-001",
    "issued_at": "2024-01-01T00:00:00Z"
  },
  "signature": "<base64-ed25519-signature>"
}
```

## Dev mode

Set `BUILDSWEEP_DEV_PRO=1` in debug builds to test Pro features locally.
