# Mailboar
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
![License](https://img.shields.io/github/license/aeyoll/toothpaste)

Mailboar provides a web interface for [Tiny MailCatcher](https://github.com/pevdh/tiny-mailcatcher), a [MailCatcher](https://mailcatcher.me/) clone written in Rust.

Requirements
---

- [https://yarnpkg.com/](yarn): Asset management

Build static assets:

```sh
yarn
yarn run build
```

Launch the http server:

```sh
mailboar --ip 127.0.0.1 --smtp-port=1025 --api-port=1080 --http-port=8025 # default values
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
