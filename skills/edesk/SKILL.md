---
name: edesk
version: 1.1.1
description: "[v1.1.1] Work with the eDesk helpdesk API via the `edesk` CLI — list/view/create/update/delete tickets, messages, sales orders, tracking links, order notes, tags, templates; search contacts; list channels/users. Use whenever the user asks about eDesk tickets, customer support queries, eDesk sales orders, or eDesk automation."
---

# eDesk CLI

`edesk` is a gh-style CLI for the eDesk API. Auth is already configured if
`edesk auth status` succeeds; otherwise ask the user to run `edesk auth login`
(tokens come from https://dashboard.edesk.com/api-token) or set `EDESK_TOKEN`.

## Ground rules

- **Always pass `--json` or `--jq <expr>`** when consuming output programmatically.
  Default output is a table (TTY) or TSV (piped).
- Destructive commands (`* delete`, `tracking clear`) need `--yes` in scripts.
  **Confirm with the user before deleting anything.**
- Exit codes: 0 ok, 1 failure, 2 usage error, 4 auth problem (`edesk auth login`).
- Lists default to 30 items; use `--all` (every page), `--limit N`, or `--page N --per-page M`.
- `edesk <noun> --help` and `edesk <noun> <verb> --help` document every flag.

## Rate limits

The API allows **60 requests/minute per client**, restoring at **2 requests per
second** after that. Exceeding it returns HTTP 429:
`{"error": {"httpCode": 429, "message": "Too many requests", "details": "Out of quota"}}`.

- The CLI already retries GET/PUT/DELETE on 429 with exponential backoff
  (up to 3 attempts); POSTs are NOT retried — on a 429'd create, wait ≥1s and
  re-issue it yourself.
- For bulk work, prefer one `--all` (100-item pages ≈ few requests) over many
  small calls, and avoid tight loops of `view`/`create`: sleep ~500ms between
  requests to stay inside the restoration rate.
- `message list --ticket` issues one request per message — on tickets with
  many messages this can eat quota quickly.
- Sustained 429s on low volume → the account's limit may need raising via
  eDesk support.

## Command map

```
edesk whoami                                  # identity behind the token
edesk ticket   list|view|create|update|update-data|delete
edesk message  list --ticket <id> | view|create|update|delete
edesk order    list|view|create|update|delete         # sales orders
edesk tracking view|add|set|clear <ORDER_ID>          # tracking links
edesk note     list|view|create|update|attach|delete  # order notes
edesk tag      list|view|create|update|delete
edesk tag-group list
edesk template list|view|create|update|delete
edesk contact  list [--query|--email|--name|--phone|--consumer-id]
edesk channel  list
edesk user     list
edesk api <PATH> [-X METHOD] [-F k=v ...] [--body JSON|--body-file F] [--paginate]
```

## Recipes

```bash
# Open tickets updated today, as JSON ids
edesk ticket list --status Open --updated-after $(date +%F) --jq '.[].id'

# Everything about one ticket, including its messages
edesk ticket view 12345 --json
edesk message list --ticket 12345 --json

# Internal note (NOT visible to the customer)
edesk message create --ticket 12345 --type Note --body "..."

# Real reply emailed to the customer — only when the user explicitly wants to send
edesk message create --ticket 12345 --body "..." --direction Outgoing --send

# Find a sales order by marketplace order id, then add tracking
ORDER_ID=$(edesk order list --seller-order "406-1195395-1150767" --jq '.[0].id')
edesk tracking add "$ORDER_ID" --link "https://carrier.example/track/XYZ" --carrier DHL

# Create a sales order (wide JSON schema; ship_to is required in practice)
edesk order create --body-file order.json
# order.json needs: channel_id, seller_order_id (unique), currency, status,
# shipping_amount, order_items[{sku,title,quantity,item_amount,shipping_amount}],
# ship_to{name,line_1,city,country,postcode}, contact{email} or contact_id

# Tag housekeeping (tags live inside a group)
edesk tag-group list --json
edesk tag create --name "VIP" --group 644646 --color 2196F3 --icon star
edesk tag update <id> --color F44336        # partial: only what you pass changes

# Attach a local file to an order note (--file is a flag, not positional)
edesk note attach 999 --file invoice.pdf --kind Invoice

# Custom fields on a ticket
edesk ticket update-data 12345 -f "Order Status=Shipped"

# Raw escape hatch for anything not covered
edesk api /tickets -F filter_status_equals=Open -F itemsPerPage=50 --jq '.data'
```

## If the `edesk` binary is unavailable

In sandboxes without the CLI (e.g. claude.ai), call the API directly with curl.
Ask the user for the token (or read `$EDESK_TOKEN`) — never hardcode it.

```bash
# Base URL https://api.edesk.com/v1, bearer auth, {data, paginator} envelope
curl -s https://api.edesk.com/v1/whoami -H "Authorization: Bearer $EDESK_TOKEN"
curl -s "https://api.edesk.com/v1/tickets?filter_status_equals=Open&page=1&itemsPerPage=50" \
  -H "Authorization: Bearer $EDESK_TOKEN"
```

Endpoints: `/tickets[/{id}][/data]`, `/messages[/{id}]`, `/sales-orders[/{id}]`,
`/sales-orders-tracking-links/{orderId}`, `/order-notes[/{id}][/attachments]`,
`/tags[/{id}]`, `/tag-groups`, `/templates[/{id}]`, `/contacts`, `/channels`,
`/users`, `/whoami`. Full reference: https://developers.edesk.com/ — but trust
the quirks below over the spec when they conflict. The 60 req/min rate limit
applies here too: honor 429 by sleeping ≥1s before retrying.

## Quirks worth knowing

- Ticket "delete" archives the ticket (status `Archived`); it does not remove it.
- Tag update rejects re-sending the same name (must-be-unique validation) — only
  pass the flags you want changed.
- `note attach` uploads multipart; `--kind Invoice|Other` applies to the batch.
- Newly created sales orders can take a moment to appear in `order list` filters
  (search index lag); `order view <id>` is immediate.
- There is no list-messages endpoint upstream; `message list --ticket` fans out
  over the ticket's `messages_ids`.

## Changelog

- **1.1.1** (2026-06-10): explicit `note attach --file` recipe (eval found agents
  guessed a positional file argument).
- **1.1.0** (2026-06-10): rate-limit section (60 req/min, 2 req/s restore, 429
  handling), curl fallback for sandboxes without the binary, versioned frontmatter.
- **1.0.0** (2026-06-10): initial release alongside edesk-cli v0.1.0.
