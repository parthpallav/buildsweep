# Safety Model

BuildSweep prioritizes safety over reclaiming additional storage.

## Classifications

| Class | Cleanup | Default selection |
|-------|---------|-------------------|
| SAFE | Eligible | Checked |
| REVIEW | Eligible | Unchecked |
| PROTECTED | Never | Disabled |
| UNKNOWN | Never | Disabled |

## Path validation pipeline

Before any cleanup operation:

1. Normalize path (Unicode NFC)
2. Check safety classification
3. Reject protected names (`.git`, `src`, `.env`, etc.)
4. Reject path traversal (`..`)
5. Reject symlinks and Windows reparse points
6. Verify canonical path is under user-approved scan root
7. Reject filesystem root and home directory
8. Revalidate immediately before trash operation (TOCTOU mitigation)

## Never auto-delete

- Unknown directories
- Source code directories
- Configuration and environment files
- Git repositories

## Trash only

Cleanup uses native OS Trash / Recycle Bin APIs via the `trash` crate. No `rm -rf` or shell commands.
