# justfile commands for crusty-rustacean-api

init-db:
  cargo run -p project-init

build-dkr:
  cargo build -p cr-api

test-dkr:
  cargo test -p cr-api

sh-status:
  cargo shuttle project status

sh-new:
  cargo shuttle project new

sh-delete:
  cargo shuttle project rm

sh-run-local:
  cargo shuttle run

sh-run-remote:
  cargo shuttle deploy

