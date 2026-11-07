# @wae/open-ftp-cli

`open-ftp` command line entry.

- Non-interactive. Decisions are made by exit code, not by prose.
- `--json` emits `wae.cli-result/v0` envelopes on stdout.
- `rsync-plan` computes a delta plan and never writes. `sync-preview` is the machine-readable preview, `rsync-apply` applies a plan.
- Sync is not one vague command: `sync-direction`, `sync-conflict-policy`, `sync-preview`, and `sync-undo` are separate capabilities.
- `endpoint-ports` configures control, TLS, data, and passive ports. It never starts or stops a remote FTP service.
- Mutating commands accept `--dry-run`.

Exit codes: `0` success, `1` failure, `2` usage, `3` capability unavailable, `4` canceled.

In this skeleton round every domain capability refuses with `not_implemented`.
