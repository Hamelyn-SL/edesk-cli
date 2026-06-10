# edesk-cli

[![CI](https://github.com/Hamelyn-SL/edesk-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/Hamelyn-SL/edesk-cli/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Hamelyn-SL/edesk-cli)](https://github.com/Hamelyn-SL/edesk-cli/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A `gh`-style command-line interface for the [eDesk](https://www.edesk.com/) helpdesk API.
Work with tickets, messages, sales orders, tags, templates and more from your terminal —
with human-friendly tables, raw JSON, an embedded jq, and full pagination support.

```console
$ edesk ticket list --status Open --limit 5
ID         SUBJECT                          STATUS  TYPE        CHANNEL  CONTACT     UPDATED
701581498  Where is my order?               Open    OrderQuery  388355   3478494326  2026-06-10 08:42:55
...

$ edesk ticket view 701581498 --json | jq .subject
$ edesk order list --status OrderShipped --jq '.[].seller_order_id'
$ edesk api /whoami
```

Covers **all 40 endpoints** of the [eDesk Open API](https://developers.edesk.com/), each one
validated against the live API.

## Installation

### macOS / Linux (shell installer)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Hamelyn-SL/edesk-cli/releases/latest/download/edesk-installer.sh | sh
```

### macOS (Homebrew)

```sh
brew install Hamelyn-SL/tap/edesk
```

### Windows (PowerShell)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/Hamelyn-SL/edesk-cli/releases/latest/download/edesk-installer.ps1 | iex"
```

### Prebuilt binaries

Download `.tar.xz`/`.zip` archives (with sha256 checksums) from the
[releases page](https://github.com/Hamelyn-SL/edesk-cli/releases/latest):
macOS (Apple Silicon + Intel) and Windows x64.

### From source

```sh
cargo install --git https://github.com/Hamelyn-SL/edesk-cli edesk
```

## Getting started

Generate an API token at [dashboard.edesk.com/api-token](https://dashboard.edesk.com/api-token), then:

```console
$ edesk auth login
Paste your eDesk API token: ********
✓ Logged in as you@company.com (token stored in system keychain)

$ edesk whoami
```

The token is stored in the OS keychain (macOS Keychain / Windows Credential Manager /
Secret Service), falling back to a `0600` file under `~/.config/edesk/` on headless systems.
Scripts and CI can use the `EDESK_TOKEN` environment variable or `--token` instead;
precedence is `--token` > `$EDESK_TOKEN` > keychain > token file.

## Commands

Commands follow a `noun verb` grammar:

| Resource | Verbs |
|---|---|
| `ticket` | `list` `view` `create` `update` `update-data` `delete` |
| `message` | `list --ticket <id>` `view` `create` `update` `delete` |
| `order` | `list` `view` `create` `update` `delete` |
| `tracking` | `view` `add` `set` `clear` (tracking links of a sales order) |
| `note` | `list` `view` `create` `update` `attach` `delete` (order notes) |
| `tag` | `list` `view` `create` `update` `delete` |
| `tag-group` | `list` |
| `template` | `list` `view` `create` `update` `delete` |
| `contact` | `list` (with `--query`, `--email`, ...) |
| `channel` | `list` |
| `user` | `list` |
| `whoami` | show the identity behind the token |
| `auth` | `login` `status` `logout` |
| `config` | `get` `set` `unset` `list` `path` |
| `api` | raw authenticated requests to any endpoint |
| `completion` | shell completions (bash, zsh, fish, powershell, elvish) |

Some examples:

```sh
# Filtering and sorting
edesk ticket list --status Pending --channel 354531 --created-after 2026-01-01 --sort last_updated_at --direction desc

# Create a ticket with an inline contact
edesk ticket create --subject "Where is my order?" --channel 372698 --contact-email buyer@example.com

# Add an internal note to a ticket (Note = not visible to the customer)
edesk message create --ticket 12345 --type Note --body "Customer called, refund approved"

# Reply to the customer by email
edesk message create --ticket 12345 --body "On its way!" --direction Outgoing --send

# Sales orders take a JSON body (wide schema: items, addresses, tracking)
edesk order create --body-file order.json

# Tracking links
edesk tracking add 3777826195 --link "https://carrier.example/track/XYZ" --carrier DHL

# Attach a file to an order note
edesk note attach 5345233 --file invoice.pdf --kind Invoice

# Raw escape hatch — any endpoint, any method
edesk api /tickets -F filter_status_equals=Open -F itemsPerPage=50
edesk api /tags --method POST --body '{"name":"vip","tag_group_id":644646}'
edesk api /channels --paginate --jq '.[].name'
```

## Output

- **TTY**: human-readable tables. **Piped**: tab-separated values (works with `cut`/`awk`).
- `--json`: raw API JSON, byte-faithful to the response.
- `--jq <expr>`: filter with an embedded jq ([jaq](https://github.com/01mf02/jaq)) — no external `jq` needed.
- `--fields id,subject,user.name`: project table columns / JSON keys (dot-paths supported).
- stdout carries data; progress and confirmations go to stderr — safe to pipe.

Pagination: `--limit N` (default 30), `--all` to exhaust every page, or `--page N [--per-page M]`
for a specific page.

Exit codes (same convention as `gh`): `0` success · `1` failure · `2` usage error ·
`4` authentication problem.

Destructive commands (`delete`, `tracking clear`) prompt for confirmation on a TTY and
require `--yes` when scripted.

## Notes on the eDesk API

The OpenAPI specs published at [developers.edesk.com](https://developers.edesk.com/) diverge
from the live API in several places. This CLI follows the **observed behavior** (verified
against production in June 2026):

- List pagination params are undocumented but work: `page` (1-based) and `itemsPerPage` (default 20).
- Validation `errorCode`s arrive as strings (`"4003"`), not integers.
- Deletes return top-level `{message, ok}` instead of the documented `{data: {ok, message}}` envelope.
- `PUT /tags/{id}` applies partial updates and **rejects** re-sending an unchanged `name`
  (4003 must-be-unique) — the CLI therefore only sends the flags you pass.
- Order-note attachments only materialize via `multipart/form-data`; the documented base64
  JSON variant is accepted but silently dropped upstream.
- Deleting a ticket archives it (status `Archived`) rather than removing it.
- Booleans are sometimes returned as `0`/`1`, and timestamp types vary per resource —
  which is why responses are passed through as raw JSON instead of strictly typed models.

## Library

The repo is a workspace: [`crates/edesk-client`](crates/edesk-client) is a standalone Rust
client for the eDesk API (reqwest + rustls, retry with backoff, typed validation errors)
that can be used independently of the CLI.

## Development

```sh
cargo test --workspace        # unit + integration tests (httpmock, no network needed)
cargo clippy --workspace --all-targets
cargo fmt --all
```

Releases are built by [cargo-dist](https://github.com/axodotdev/cargo-dist): push a `v*` tag
and CI produces the binaries, installers, checksums and the Homebrew formula.

## License

[MIT](LICENSE)
