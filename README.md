# zk-square-proof

Rust mini-project demonstrating a **Groth16 zk-SNARK** that proves knowledge of a secret integer `x` such that `x² = y`, without revealing `x`.

## Prerequisites
* Rust 1.79+ (`rustup update stable`)
* No other dependencies—Arkworks crates are fetched automatically by Cargo.

## Build & Run

```bash
cargo run
