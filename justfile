# justfile commands for crusty-rustacean-api

init-db:
  cargo run -p project-init

run-local:
  cd cr-api && cargo run

build-dkr:
  docker build --tag cr-api-docker --file Dockerfile .

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

