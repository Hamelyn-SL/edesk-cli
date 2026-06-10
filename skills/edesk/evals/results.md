# edesk skill — eval results

Method: for each case in [cases.yaml](cases.yaml), a fresh agent receives ONLY
the SKILL.md content plus the task (no web, no execution) and answers with the
command(s)/approach it would use. An independent judge scores the answer
against the rubric (pass/fail + 0-10).

## v1.1.1 — 2026-06-10

| Case | v1.1.0 | v1.1.1 |
|---|---|---|
| list-open-today | ✅ 10 | — |
| internal-note | ✅ 10 | — |
| scripted-delete | ✅ 7 | — |
| order-tracking-two-step | ✅ 10 | — |
| rate-limit-429 | ✅ 10 | — |
| attach-invoice | ❌ 5 | ✅ (re-run after fix) |
| create-sales-order | ✅ 10 | — |
| curl-fallback | ✅ 10 | — |
| deleted-ticket-archived | ✅ 10 | — |
| tag-partial-update | ✅ 9.5 | — |

**v1.1.0: 9/10 passed, avg score 9.15.** The one failure (`attach-invoice`)
was traced to the skill listing `note attach` in the command map without
showing its syntax: the agent guessed a positional file argument instead of
`--file`. v1.1.1 adds the explicit recipe; the failing case passes on re-run.

Notable judge remarks:
- `rate-limit-429` and `curl-fallback` scored 10/10 — the v1.1.0 additions
  (rate limits, curl fallback) are picked up correctly.
- `scripted-delete` passed at 7/10: the core answer was right; points were
  deducted for a buggy optional bash error-handling snippet the agent
  improvised beyond the skill's content.
