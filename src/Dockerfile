# STAGE 1 is to build the binary
# Use rust-based image for container
FROM rust:1.74 as builder
 
WORKDIR /app
RUN apt-get update
RUN apt-get update && apt install llvm-dev libclang-dev clang libcurl4 libsodium-dev openssl -y
 
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
 
# Build for release, cache dependencies
RUN cargo build --release
 
# Final stage
FROM debian:bookworm-slim
 
# Install tini and other dependencies
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install --yes \
    tini libcurl4 \
&& apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
 
WORKDIR /app
COPY --from=builder /app/target/release/actix-web-test /app/actix-web-app
 
# Use tini as the entry point to manage signals
ENTRYPOINT ["/usr/bin/tini", "--"]
 
CMD ["/app/actix-web-app"]
EXPOSE 8080
 
# Set up a non-root user for better security
RUN useradd -m user
USER user
