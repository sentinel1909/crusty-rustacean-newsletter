# justfile commands for crusty-rustacean-api

init:
  cargo run -p project-init

dev:
  cd cr-api && cargo watch -x check -x test -x run

ci:
  cargo tarpaulin --ignore-tests && cargo clippy -- -D warnings && cargo fmt -- --check && cargo audit

