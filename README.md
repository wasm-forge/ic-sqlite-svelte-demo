[![Tests](https://github.com/wasm-forge/ic-sqlite-svelte-demo/actions/workflows/build-check.yml/badge.svg)](https://github.com/wasm-forge/ic-sqlite-svelte-demo/actions/workflows/build-check.yml)

# `ic-sqlite-svelte-demo`

A simple demo canister of using Sqlite with a Svelte frontend.

## Prerequisites

* [rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
* [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install/)

Also, install some extra tools. For development:
```bash
cargo install wasi2ic

cargo install ic-wasm
```

For testing:
```bash
cargo install ic-test
```

## Features

* Backend using [`ic-rusqlite`](https://github.com/wasm-forge/ic-rusqlite)
* Svelte frontend
* Server-side input validation
* [`ic-test`](https://github.com/wasm-forge/ic-test) testing framework for testing the backend functions
* No client-side input validation - this is to allow experimenting with the wrong data input

## Running the project locally

If you want to deploy the project locally, you can use the following commands:

```bash
dfx start --background

npm install

dfx deploy
```

## Testing

To run tests, you need to first compile the project, you can do it with:
```bash
dfx build
```

Then start the integration tests with:
```bash
cargo test -p tests
```
