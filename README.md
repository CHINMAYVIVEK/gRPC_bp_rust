
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

## References

- [gRPC Documentation](https://grpc.io/docs/)
- [Tonic: A gRPC over HTTP/2 Implementation](https://github.com/hyperium/tonic)
- [Prost: A Protocol Buffers Implementation for Rust](https://github.com/danburkert/prost)

---
