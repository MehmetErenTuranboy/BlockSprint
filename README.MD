# Solana Block Height Fetcher

This Rust program demonstrates how to connect to the Solana blockchain using the Solana RPC API and retrieve the current block height. Indicates the length of the blockchain in terms of the number of blocks.

## Prerequisites

Before running this program, ensure you have Rust and Cargo installed on your system. You can install both by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
- For this project cargo 1.76.0 was used.

## Dependencies

```toml
[dependencies]
solana-client = "1.7.11"
solana-sdk = "1.7.11"
tokio = { version = "1", features = ["full"] }
```
- solana-client: This line in the Cargo.toml file specifies the dependency on the solana-client. The solana-client crate provides Rust bindings for interacting with the Solana blockchain, including making RPC calls to Solana nodes.

- solana-sdk: This line indicates the project's dependency on the solana-sdk. The solana-sdk crate contains the Solana blockchain's software development kit, offering tools, types, and functions for building Solana programs and interacting with the blockchain.

- tokio: This specifies the project's dependency on the tokio async runtime. Tokio is an asynchronous runtime for the Rust programming language, allowing for non-blocking I/O operations.

## How to Run
- Build the code
    - In order to build the code use one of following commands
```rust
cargo clippy
cargo build
```
- Run the code
```rust
cargo run
```