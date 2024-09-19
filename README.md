
# gRPC_bp_rust

A boilerplate for building gRPC microservices in Rust. This project provides an out-of-the-box setup for a gRPC server and client using **`tonic`** for gRPC support and **`tokio`** for async runtime. The project includes auto-generated Protobuf support, making it ideal for creating high-performance, asynchronous services.

## Features

- **Async gRPC server and client** using `tonic` and `tokio`.
- Auto-generated **Protobuf** code with a sample `.proto` file.
- Supports **multi-stage Docker** builds for efficient containerization.
- Example server and client implementations to get you started quickly.

## Prerequisites

- **Rust** installed. You can follow the [Rust installation guide](https://www.rust-lang.org/tools/install).
- **Protobuf Compiler (`protoc`)** installed. Follow the [gRPC Protobuf installation instructions](https://grpc.io/docs/protoc-installation/).
- **Docker** (optional, for containerization).

## Setup and Usage

### Step 1: Clone the Repository

```bash
git clone https://github.com/chinmayvivek/gRPC_bp_rust.git
cd gRPC_bp_rust
```

### Step 2: Install Dependencies

Install Rust dependencies and ensure everything is up to date:

```bash
cargo build
```

This will download and compile the dependencies specified in `Cargo.toml`.

### Step 3: Compile the Protobuf Files

You need to generate the Rust gRPC code from the `.proto` files.

```bash
cargo build --release
```

The `build.rs` script will handle compiling the protobuf files into the necessary Rust modules. Make sure that `protoc` is installed and available in your system's PATH.

### Step 4: Running the Server

Now, you can start the gRPC server:

```bash
cargo run --bin server
```

This will start the server, which listens on the default gRPC port `50051`.

### Step 5: Running the Client

In a separate terminal, run the gRPC client to interact with the server:

```bash
cargo run --bin client
```

The client will send requests to the server and receive responses over gRPC.

### Step 6: Using Docker for Containerization

This project includes a multi-stage Docker build for easy deployment.

#### Step 6.1: Build the Docker Image

```bash
docker build -t grpc_bp_rust .
```

#### Step 6.2: Run the Docker Container

You can run the server in a Docker container as follows:

```bash
docker run -p 50051:50051 grpc_bp_rust
```

This will start the gRPC server in a container and expose it on port `50051`.

## Customization

To customize the service, modify the `.proto` file in the `proto/` directory, then run the following to regenerate the Rust code:

```bash
cargo build --release
```

The gRPC server and client logic in `src/server.rs` and `src/client.rs` can be adapted to your specific use case.

## References

- [gRPC Documentation](https://grpc.io/docs/)
- [Tonic: A gRPC over HTTP/2 Implementation](https://github.com/hyperium/tonic)
- [Prost: A Protocol Buffers Implementation for Rust](https://github.com/danburkert/prost)

---
