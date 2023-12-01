# Use the official Rust image as a builder stage
FROM rust:1.56 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin actix-web-app
WORKDIR /actix-web-app

# Copy our manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependencies are built, copy your source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/actix_web_app*
RUN cargo build --release

# Final stage: the application
# Use the smallest Docker image we can
FROM debian:buster-slim
ARG APP=/usr/src/app

EXPOSE 8080

# Install libssl-dev and ca-certificates
RUN apt-get update \
    && apt-get install -y libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /actix-web-app/target/release/actix-web-app ${APP}/actix-web-app

# Set the current working directory inside the container
WORKDIR ${APP}

# Use an unprivileged user
RUN useradd -m user
USER user

# Run the binary
CMD ["./actix-web-app"]
