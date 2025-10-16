# sqlite-fsr

A small-from-scratch SQLite-like reader and SQL parser written in Rust for learning and experimentation.

This repository implements a subset of SQLite internals: parsing SQL statements, reading database file structures (pages, tables, schemas), and providing commands for basic introspection and query execution. The project is intended as an educational implementation and testing ground, not a production database.

## Quick facts

- Crate: `sqlite-fsr`
- Rust edition: 2021
- Minimum Rust version: 1.80

## Repository layout

- `src/` — crate source. Main entry is `src/main.rs` which calls `sqlite_fsr::run`.
- `src/command/` — CLI command implementations (e.g. `dbinfo`, `tables`, `sql`).
- `models/` — database file model structures (pages, records, schema parsing).
- `utils/` — helper utilities (varint parsing, etc.).
- `tests/` — integration and unit tests used by `cargo test`.

## Build

You need a recent Rust toolchain (stable channel, Rust >= 1.80). To build the project:

```bash
cargo build --release
```

To run the binary from the workspace (debug build):

```bash
cargo run -- <args>
# example: cargo run -- dbinfo path/to/file.sqlite
```

Alternatively, call the library from another binary — `src/main.rs` calls `sqlite_fsr::run`.

## Tests

Run the test suite with:

```bash
cargo test
```

There are unit and integration tests under `tests/` and many builder helpers under `src` and `models/` used by tests.

## Contribution

Contributions are welcome. Keep changes small and focused. If you add functionality, please include tests that exercise the new behavior.

Suggested workflow:

1. Fork and create a feature branch.
2. Add tests for new behavior in `tests/` or unit tests next to implementation.
3. Run `cargo test` and `cargo clippy` (if you use clippy) locally.
4. Open a PR with a short description and rationale.

## License

This project does not include an explicit license file in the repository root. If you intend to publish or use this code outside of private experiments, consider adding a LICENSE file (for example, MIT or Apache-2.0).

## Notes

This README is intentionally short and focused on getting started. If you'd like, I can expand it with:

- Example commands and sample database files
- A short architecture overview (page layout, varint format, record format)
- Development workflow and a suggested set of linters/formatters

If you want any of those, tell me which and I will add them.
