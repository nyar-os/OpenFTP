---
name: workflow
description: Drive the transport-neutral OpenFTP CLI to read capabilities, preview a sync plan, and apply a transfer only after explicit confirmation.
---

# OpenFTP agent workflow

This skill drives the `open-ftp` CLI. The CLI is non-interactive and every
command returns a stable exit code:

- `0` success
- `1` runtime failure
- `2` usage or validation failure
- `3` structured capability failure (`unsupported_platform`, `not_implemented`,
  `dependency_blocked`)
- `4` canceled

Prefer `--json`. Every result is a `wae.cli-result/v0` envelope:
`{ "schema", "ok": true, "data": ... }` or
`{ "schema", "ok": false, "error": { "code", "capability", "platform", "message" } }`.
Do not parse human prose.

## Plan before apply

`rsync-plan` and `rsync-apply` are separate capabilities. A plan never writes.
An apply requires the plan identity it was previewed from, so a dry run cannot
silently turn into a write.

1. Read the capability table. Rows with a non-null `state` are refusals and must
   not be attempted.

   `open-ftp capabilities --json`

2. Inventory endpoints only. Credentials are redacted in the report.

   `open-ftp endpoint-list --json`

3. Preview a delta. Show the plan, its size, and its delete list to the user.

   `open-ftp rsync-plan --json`

4. Apply only after explicit confirmation. Deletions require their own
   confirmation and are never implied by a mirror or a sync.

   `open-ftp rsync-apply --json`

## File transfer

5. Preview the transfer plan first.

   `open-ftp ftp-transfer --dry-run --json`

6. Transfer with resume and checksum verification. A failed transfer leaves a
   partial file that is reported, never silently accepted as complete.

   `open-ftp ftp-transfer --json`

7. Mirror only after confirming the delete policy. A mirror with deletions must
   list every removed path before applying.

   `open-ftp ftp-mirror --dry-run --json`

## Audit

Every domain call is recorded with the capability id, the target count, and a
redacted target summary. Use a stable `--actor` id so records are attributable.

   `open-ftp audit --kind <user|agent> --capability <id> --limit <n> --json`

## Safety

- In this skeleton round every domain capability refuses with `not_implemented`.
  A refusal is not a success. Do not fall back to an external `ftp`, `sftp`,
  `rsync`, or `openssl` process.
- Do not execute a mutating command without explicit confirmation（确认）from the
  user after the dry-run preview.
- Do not retry a structured capability failure with elevated privileges. Report
  the `code`, `capability`, and `platform` fields instead.
- Treat `--json` output as the contract. Exit code `3` means the capability is
  unavailable on this platform or is not implemented.
