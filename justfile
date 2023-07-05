# justfile commands for crusty-rustacean-api

init:
  cargo run -p project-init

dev:
  cargo watch -x check -x test -s 'cd cr-api && cargo run -p cr-api'

ci:
  cargo tarpaulin --ignore-tests && cargo clippy -- -D warnings && cargo fmt -- --check && cargo audit

