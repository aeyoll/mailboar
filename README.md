# Mailboar

[![GitHub Actions workflow status](https://github.com/aeyoll/mailboar/workflows/ci/badge.svg)](https://github.com/aeyoll/mailboar/actions)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.62.0+-lightgray.svg)](#rust-version-requirements)
![License](https://img.shields.io/github/license/aeyoll/toothpaste)

Mailboar provides a web interface for [Tiny MailCatcher](https://github.com/pevdh/tiny-mailcatcher), a [MailCatcher](https://mailcatcher.me/) clone written in Rust.

- [Installation](#installation)
  - [Using Cargo, Git and Yarn](#using-cargo-git-and-yarn)
  - [Using Docker](#using-docker)
- [Development](#development)

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
mailboar --ip 127.0.0.1 --smtp-port=1025 --api-port=1080 --http-port=8025 # default values
```

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

1.62.0+
