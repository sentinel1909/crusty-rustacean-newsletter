# We use the latest Rust Stable release as base image
FROM rust:1.68.0

# Let's switch our working directory to 'app' (equivalent to 'cd app')
# The 'app' folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /cr-api

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from our working environment to our Docker image
COPY . .

# Set SQLX_OFFLINE mode
ENV SQLX_OFFLINE true

# Let's build our binary!
RUN cargo build --release

# When 'docker run' is executed, launch the binary!
ENTRYPOINT ["./target/release/cr-api"]