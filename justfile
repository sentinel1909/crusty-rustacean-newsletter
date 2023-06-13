# justfile commands for crusty-rustacean-api

init:
  cargo run -p project-init

dev:
  cd cr-api && cargo watch -x check -x clippy -x fmt -x run

