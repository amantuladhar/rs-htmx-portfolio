# Migration

## Table Prefix

- `TABLE_PREFIX = rs_portfolio_`

## Setup

- `cargo install sqlx-cli --no-default-features --features native-tls,postgres`
- `sqlx migrate add -r <name>`
- `sqlx migrate run`
- `sqlx migrate revert`

## Using Make

- `make db-add-migration name=<name>`

## Compile time verification

- https://github.com/launchbadge/sqlx?tab=readme-ov-file#compile-time-verification

## Offline DB Data

- https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query

## References

- https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md
