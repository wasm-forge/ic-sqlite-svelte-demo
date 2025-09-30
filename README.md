[![Tests](https://github.com/wasm-forge/ic-sqlite-svelte-demo/actions/workflows/build-check.yml/badge.svg)](https://github.com/wasm-forge/ic-sqlite-svelte-demo/actions/workflows/build-check.yml)

# `ic-sqlite-svelte-demo`

A simple demo canister of using Sqlite with a Svelte frontend.


## Running the project locally

If you want to deploy your project locally, you can use the following commands:

```bash
dfx start --background

dfx deploy
```

## Features

* Backend using [`ic-rusqlite`](https://github.com/wasm-forge/ic-rusqlite)
* Svelte frontend
* Server-side input validation
* [`ic-test`](https://github.com/wasm-forge/ic-test) testing framework for testing the backend functions

## Testing

To run tests, you need to first compile the project, you can do it with:
```bash
dfx build
```

Then start the integration tests with:
```bash
cargo test -p tests
```

