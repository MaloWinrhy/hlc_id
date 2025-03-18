# HLC ID - Hybrid Logical Clock Identifiers

[![Crates.io](https://img.shields.io/crates/v/hlc_id.svg)](https://crates.io/crates/hlc_id)
[![Documentation](https://docs.rs/hlc_id/badge.svg)](https://docs.rs/hlc_id)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

HLC ID is a Rust library designed to generate, manage, and encode Hybrid Logical Clock (HLC)-based identifiers. These unique, distributed, and compact identifiers facilitate event ordering in decentralized systems.

## Features

- Hybrid Logical Clock (HLC): Generates timestamped event IDs based on logical time.
- Compact Representation: Stores IDs as 128-bit integers.
- Base64 Encoding: Encodes IDs for easier transmission and storage.
- Custom Node IDs: Assigns unique identifiers per instance.
- Persistence Support: Save and restore clocks for continuity.
- Event Comparison: Compare events to determine their relative order.

## Installation

Add `hlc_id` to your `Cargo.toml`:

```toml
[dependencies]
hlc_id = "0.1.4"
```

Run `cargo build` to download and compile the library.

## Usage

### Generating and Managing HLC IDs

```rust
use hlc_id::{clock::HybridLogicalClock, id::HLCId};
use chrono::Utc;

fn main() {
    let mut clock = HybridLogicalClock::new(42);
    let timestamp = Utc::now().timestamp_millis() as u64;

    let hlc_id = HLCId::generate(&mut clock, timestamp);
    println!("Generated ID: {:?}", hlc_id);

    let id_as_u128 = hlc_id.to_u128();
    println!("ID as 128-bit integer: {}", id_as_u128);

    let encoded = hlc_id.encode_base64();
    println!("Encoded ID (Base64): {}", encoded);

    let decoded_id = HLCId::decode_base64(&encoded).unwrap();
    println!("Decoded ID: {:?}", decoded_id);
}
```

## Running Tests & Examples

Run the built-in tests:
```sh
cargo test
```

Run the example program:
```sh
cargo run --example generate_id
```

## API Overview

### HybridLogicalClock
- `new(node_id: u16) -> HybridLogicalClock`: Creates a new logical clock.
- `update(&mut self, external_timestamp: u64)`: Updates the clock with an external timestamp.
- `process_timestamp(&mut self, received_timestamp: u64)`: Adjusts the clock based on incoming event timestamps.
- `save_state(&self, path: &str) -> std::io::Result<()>`: Saves clock state to a file.
- `load_state(path: &str) -> std::io::Result<Self>`: Loads a clock from a saved state.

### HLCId
- `generate(clock: &mut HybridLogicalClock, timestamp: u64) -> HLCId`: Generates a new HLC-based identifier.
- `to_u128(&self) -> u128`: Converts an ID to a 128-bit integer.
- `from_u128(id: u128) -> HLCId`: Reconstructs an ID from a 128-bit integer.
- `encode_base64(&self) -> String`: Encodes the ID in Base64.
- `decode_base64(encoded: &str) -> Result<HLCId, String>`: Decodes a Base64-encoded ID.
- `is_before(&self, other: &HLCId) -> bool`: Compares two IDs to determine event order.

## Documentation

Full API documentation is available at [docs.rs](https://docs.rs/hlc_id).

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome. Submit issues, pull requests, or suggest improvements.

## Author

Developed by Malo Henry  
GitHub: [MaloHenry](https://github.com/MaloHenry)

