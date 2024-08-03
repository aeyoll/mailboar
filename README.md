[![Mailboar](https://raw.githubusercontent.com/aeyoll/mailboar/main/crates/frontend/assets/images/logo.png)](https://github.com/aeyoll/mailboar)

# Mailboar

[![GitHub Actions workflow status](https://github.com/aeyoll/mailboar/workflows/ci/badge.svg)](https://github.com/aeyoll/mailboar/actions)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.71.0+-lightgray.svg)](#rust-version-requirements)
![License](https://img.shields.io/github/license/aeyoll/toothpaste)

Mailboar is an email test server written in Rust.

In the backend, it uses a fork of [Tiny MailCatcher](https://github.com/pevdh/tiny-mailcatcher), rewritten with the Axum framework, to provide extra features.

- [Features](#features)
- [REST API](#rest-api)
- [Installation](#installation)
  - [Using Cargo, Git and Yarn](#using-cargo-git-and-yarn)
  - [Using Docker](#using-docker)
  - [Environment variables configuration](#environment-variables-configuration)
- [Development](#development)

Features
---

- Local SMTP server
- REST API
- Web interface to view emails as plain text, HTML or source code, download attachments.
- SSE to receive new emails in real time.
- Dark and light themes.
- Allow to send emails to a specific address to view them in a real mailbox.

REST API
---

The REST API is the same as the one of MailCatcher and Tiny MailCatcher:

- The `GET/DELETE http://localhost:1080/messages` endpoint returns/delete all messages in the repository.
- The `GET http://localhost:1080/messages/:id.json` endpoint returns a single message in JSON format.
- The `GET http://localhost:1080/messages/:id.source` endpoint returns the message source.
- The `GET http://localhost:1080/messages/:id.html` endpoint returns the HTML version of this message.
- The `GET http://localhost:1080/messages/:id.eml` endpoint returns the EML version of this message.
- The `GET http://localhost:1080/messages/:id.plain` endpoint returns the text/plain version of this message.
- The `DELETE http://localhost:1080/messages/:id` endpoint deletes a message.
- The `GET http://localhost:1080/messages/:id/parts/:cid` endpoint returns attachments by content ID.

In addition, the following endpoints are available:

- The `POST http://localhost:1080/messages/:id/send` endpoint sends a message to the specified address.
- The `GET http://localhost:1080/events` endpoint returns SSE events.

Installation
---

### Using Cargo, Git and Yarn

First, install `mailboar` using Cargo:

```sh
cargo install mailboar
```

Then, clone this repository, and build the static assets using [https://yarnpkg.com/](yarn). Node >= 20 is required.

```sh
cd crates/frontend
yarn # Install dependencies
yarn run build # Build static assets
```

This will create a `static` repository with all the assets.

Finally, launch `mailboar` from the cloned repository:

```sh
mailboar --ip 127.0.0.1 --smtp-port=1025 --api-port=1080 --http-port=8025 --assets-path=crates/frontend/static # default values
```

### Environment variables configuration

To be able to send emails to a specific address, you need to configure the SMTP server to use. The following environment variables can be used to do so:

- `MAILBOAR_SMTP_DSN`: The DSN of the SMTP server to use. Defaults to `smtp://127.0.0.1:25`.
- `MAILBOAR_SMTP_FROM`: The email address to use as the sender. Defaults to `mailboar@localhost`.

The MAILBOAR_SMTP_DSN can be configured in multiple ways. Go to [lettre documentation](https://docs.rs/lettre/0.11.7/lettre/transport/smtp/struct.SmtpTransport.html#method.from_url) to see all the possible configurations.

### Using Docker

There is also a small Docker image available, with all batteries included:

```sh
docker run --rm --init -t -p 1080:1080 -p 1025:1025 -p 8025:8025 aeyoll/mailboar:latest
```

Development
---

For easier development, use [https://github.com/watchexec/cargo-watch](cargo-watch) to auto-compile on change:

```sh
cargo watch -x 'run'
```

Auto-compile static assets:

```sh
yarn run start
```

Rust version requirements
---

1.71.0+
