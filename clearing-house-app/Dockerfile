#syntax=docker/dockerfile:1

# Use an official Rust runtime as a parent image
FROM rust:latest AS build-env

# Set the working directory inside the container
WORKDIR /usr/src/chapp

# Copy the Cargo.toml and Cargo.lock files to leverage Docker cache
COPY Cargo.toml Cargo.lock config.toml  ./

# Copy the source code into the container
COPY migrations ./migrations
COPY src ./src

# Build the Rust application with dependencies (this helps to cache dependencies)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/src/chapp/target \
    cargo build --release \
 && cp target/release/clearing-house-app /usr/src/chapp/clearing-house-app

FROM gcr.io/distroless/cc-debian12

# Expose any necessary ports (if your Rust app listens on a port)
EXPOSE 8000

WORKDIR /app
COPY init_db /app/init_db
COPY config.toml /app/

COPY --from=build-env /usr/src/chapp/clearing-house-app /app/
CMD ["/app/clearing-house-app"]
