# recipetwo

## Development guide

- Install `cargo-cli` and sql driver

```shell
cargo install diesel-cli --no-default-features --features postgres
```

Replace `postgres` by `sqlite`, `mysql`, whatever.

## Code stucture

- `bin`: code for web server
- `libs.rs`: library crate for the underlying stuff.
