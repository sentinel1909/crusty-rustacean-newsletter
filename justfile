init-db:
  cargo run -p project-init

status:
  cargo shuttle project status

new:
  cargo shuttle project new

delete:
  cargo shuttle project rm

run-local:
  cargo shuttle run

run-remote:
  cargo shuttle deploy

