# Stage 1: Build the Rust application
FROM rust:1.81.0 as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy Cargo.toml and Cargo.lock to build dependencies first (using Docker's caching)
COPY Cargo.toml Cargo.lock ./

# Create an empty "src" directory to compile dependencies first
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Pre-build the dependencies
RUN cargo build --release

# Copy the actual source code
COPY ./src ./src
COPY ./proto ./proto
COPY build.rs .

# Build the final application (gRPC server)
RUN cargo build --release --bin server

# Stage 2: Create the minimal runtime image
FROM debian:buster-slim AS runtime

# Install required system libraries
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/server .

# Expose the gRPC port (50051 by default)
EXPOSE 50051

# Command to run the gRPC server
CMD ["./server"]
