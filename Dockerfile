# builder stage
FROM lukemathwalker/cargo-chef:latest-rust-1.68.0 as chef

WORKDIR /app

RUN apt update && apt install lld clang -y

FROM chef as planner

COPY /cr-api/. .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY /cr-api/. .

ENV SQLX_OFFLINE true

RUN cargo build -p cr-api --release

# Runtime stage

FROM debian:bullseye-slim AS Runtime

WORKDIR /app

RUN apt-get update -y && apt-get install -y --no-install-recommends openssl ca-certificates && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/cr_api cr_api

COPY cr-api/configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./cr_api"]
