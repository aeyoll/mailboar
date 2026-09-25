# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

QA and testers on a shared or local Mailboar instance. Job: verify what an application actually sent, without using a real mailbox.

## Product Purpose

Mailboar is an email test server. It accepts mail over local SMTP, stores it for inspection, and exposes it through a web UI and a REST API so testers can confirm content, HTML, plain text, source, and attachments.

Success is: a tester can see the exact message the app sent, as it was sent, without waiting on a real inbox.

## Positioning

Rust MailCatcher-compatible catcher with a real web UI: live SSE, HTML/plain/source, attachments, dark/light themes, and optional forward of a caught message to a real mailbox.

## Operating Context

Testers run Mailboar beside the app under test (local process or Docker). The app sends SMTP to Mailboar (default port 1025). Testers inspect mail in the web UI (default HTTP 8025) or via the API (default 1080). New messages arrive over SSE without a full refresh. Optional forward to a real inbox requires `MAILBOAR_SMTP_DSN` and `MAILBOAR_SMTP_FROM`. Messages currently live in an in-memory repository for the process lifetime; persistence is not a confirmed product promise.

## Capabilities and Constraints

Confirmed:

- Local SMTP server
- MailCatcher / Tiny MailCatcher REST API compatibility — existing clients and tests must keep working
- Web UI: message list, detail, HTML/plain/source, attachments, delete, dark/light theme
- SSE `/events` for new mail in real time
- Optional `POST /messages/:id/send` to forward a caught message when SMTP env is configured
- MIT-licensed Rust workspace (Axum backend, Vue 3 + Tabler/Bootstrap frontend)

Undecided:

- Whether persistence beyond process lifetime is a requirement
- Whether a specific accessibility standard is required
- Whether the existing logo is a binding identity lock beyond the product name

## Brand Commitments

Product name is Mailboar. Existing mark lives at `crates/frontend/assets/images/logo.png`. No other brand, voice, or visual-identity lock was confirmed.

## Evidence on Hand

- README and this repository as the source of product and API facts
- Logo at `crates/frontend/assets/images/logo.png`
- No testimonials, customers, benchmarks, pricing, or press. Future work must not fabricate them.

## Product Principles

- Inspection truth: show the message as sent, including HTML, plain, source, and attachments.
- API contract: keep MailCatcher-compatible endpoints working for existing clients and tests.
- Tester speed: new mail should appear in the UI without a ritual of refresh.
- Optional real-world check: forwarding exists as an opt-in, not as the default path.
- Local/dev tool: do not invent hosted-product, customer, or commercial claims.
