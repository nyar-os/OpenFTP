# OpenFTP

FTP/FTPS endpoint management and file synchronization. In this skeleton round
every domain capability refuses with `not_implemented`, and no external `ftp`,
`sftp`, `rsync`, or `openssl` process is ever started.

## Capabilities

| Capability | Target behaviour |
|------------|------------------|
| `endpoint-list` | Inventory configured FTP and FTPS endpoints with redacted connection metadata |
| `endpoint-ports` | Configure control, TLS, data, and passive ports and preview the firewall impact. Never starts or stops a remote FTP service |
| `ftp-connect` | Open the control channel, authenticate, and negotiate explicit or implicit TLS |
| `ftp-list` | List a remote directory with encoding negotiation and a deterministic order |
| `ftp-transfer` | Upload or download files with resume, checksum verification, and progress |
| `ftp-mirror` | Mirror a tree with an explicit delete policy and conflict confirmation |
| `rsync-plan` | Compute the block-level delta plan without writing or rendering a preview |
| `sync-direction` | Declare local-to-remote, remote-to-local, or bidirectional and reject a plan that disagrees |
| `sync-conflict-policy` | Resolve conflicts with skip, overwrite, newer-wins, or rename, with confirmation for destructive choices |
| `sync-preview` | Report the dry-run preview: direction, change and conflict counts, planned bytes, and undo scope |
| `sync-undo` | Revert only files this plan created or overwrote. Pre-existing remote files are never deleted automatically |
| `rsync-apply` | Apply a delta plan with a transaction and rollback boundary |
| `sync-schedule` | Schedule a sync task with idempotent retries and stable run ids |
| `host-install` | Install the host integration and desktop entry |
| `host-uninstall` | Remove the host integration and clean residual state |

Plan, preview, apply, and undo are separate capabilities. A plan never writes,
an apply requires the plan identity it was previewed from, and an undo touches
only the files recorded by that plan. Sync direction and conflict policy are
their own rows, so a single vague `sync` command does not exist.

## Layout

| Path | Responsibility |
|------|----------------|
| `projects/open-ftp.rs/crates/core` | Portable domain model, capability matrix, audit fields |
| `projects/open-ftp.rs/crates/napi` | Platform-neutral N-API surface over the `wae` facade |
| `projects/open-ftp.rs/crates/cli` | Command ids, exit codes, and the machine-readable contract |
| `projects/open-ftp.ts/packages/open-ftp` | Typed library over the native boundary. Owns no `bin` |
| `projects/open-ftp.ts/packages/open-ftp-cli` | The `open-ftp` command entry and locales |
| `projects/open-ftp.ts/packages/open-ftp-ui` | VMZ page and design tokens. Staged into the CLI `dist/gui` |
| `projects/open-ftp.ts/packages/open-ftp-skills` | Agent skills with frontmatter and safety prerequisites |
| `projects/open-ftp.ts/packages/open-ftp-win32-x64` | First native platform package |
| `wae.config.ts` | Native targets, platform packages, TS/UI/skills steps, staging and release set |

## First run

```sh
npm install --global @wae/wae
pnpm install
```

Commit `pnpm-lock.yaml` and `Cargo.lock` before enabling CI. `check-typescript.yml`
runs `pnpm install --frozen-lockfile`, so a missing lockfile fails the job by
design.

## Commands

```sh
pnpm run build         # wae build: native → TS → UI → skills → staging
pnpm run build:napi    # build one configured native target and stage it
pnpm run stage:napi    # restage an already built native artifact
pnpm run smoke:napi    # load the staged addon and verify capabilities
pnpm run check:rust    # fmt + clippy -D warnings + test
```

CLI surface:

```sh
open-ftp capabilities --json
open-ftp version
open-ftp ftp-connect --json
open-ftp ftp-transfer --dry-run --json
open-ftp sync-direction --json
open-ftp sync-conflict-policy --dry-run --json
open-ftp sync-preview --json
open-ftp rsync-plan --json
open-ftp rsync-apply --json
open-ftp sync-undo --json
open-ftp audit --kind agent --limit 20 --json
```

Exit codes: `0` success, `1` failure, `2` usage, `3` capability unavailable,
`4` canceled. `--json` emits `wae.cli-result/v0` envelopes.

## Audit

Every exported N-API capability is wrapped at the boundary through `wae::audit`
(`wae.audit/v0`). The domain supplies the capability id, the target count, the
redacted target summary, and a stable error code. Actor, host, session, schema,
timestamps, duration, and sanitisation stay in the shared facility. Do not add a
second audit schema.

## License

Apache-2.0. See the single repository-level `License.md`.
