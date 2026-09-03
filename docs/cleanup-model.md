# Cleanup Model

## Flow

```
Scan → Candidate list → User selection → Cleanup plan → Review → Confirm → Revalidate → Trash → Manifest
```

## Cleanup plan

Contains:
- `plan_id` (UUID)
- List of items with path, size, safety class
- Approved scan roots for path validation

## Manifest

Append-only JSONL at `{app_data}/cleanup-manifest.jsonl`:

- timestamp
- original path
- artifact type
- size
- success/failure
- platform
- app version

No file contents are recorded. Manifest is never sent anywhere.

## History (Pro)

Summary entries at `{app_data}/cleanup-history.jsonl`:

- timestamp
- total bytes moved
- item count
