
# HLC ID

[![Crates.io](https://img.shields.io/crates/v/hlc_id.svg)](https://crates.io/crates/hlc_id)
[![Documentation](https://docs.rs/hlc_id/badge.svg)](https://docs.rs/hlc_id)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

HLC ID is a Rust library for generating, managing, and encoding **Hybrid Logical Clock (HLC)**-based identifiers. These identifiers are globally unique, distributed, and compact, making them ideal for distributed systems.

---

## 📖 Features

- **Hybrid Logical Clock (HLC):** Generate IDs based on logical timestamps.
- **Compact Representation:** Convert IDs to 128-bit integers for storage and transmission.
- **Base64 Encoding:** Encode and decode IDs in Base64 for readable formats.
- **Custom Node IDs:** Assign unique node identifiers to each HLC instance.

---

## 🚀 Getting Started

### Installation

Add `hlc_id` to your `Cargo.toml`:

```toml
[dependencies]
hlc_id = "0.1.0"
```

Run `cargo build` to download the library.

---

## 🔧 Usage

Here's an example of how to generate and manage HLC-based IDs:

```rust
use hlc_id::{clock::HybridLogicalClock, id::HLCId};

fn main() {
    // Create a Hybrid Logical Clock with a unique node ID
    let mut clock = HybridLogicalClock::new(42);

    // Generate a new HLC ID
    let hlc_id = HLCId::generate(&mut clock);
    println!("Generated ID: {:?}", hlc_id);

    // Convert the ID to a compact 128-bit integer
    let id_as_u128 = hlc_id.to_u128();
    println!("ID as 128-bit integer: {}", id_as_u128);

    // Encode the ID to Base64
    let encoded = hlc_id.encode_base64();
    println!("Encoded ID (Base64): {}", encoded);

    // Decode the Base64 back to an HLC ID
    let decoded_id = HLCId::decode_base64(&encoded).unwrap();
    println!("Decoded ID: {:?}", decoded_id);
}
```

---

## 🧪 Examples

You can run examples provided in the `examples` directory:

### Generate and Encode HLC IDs
```bash
cargo run --example generate_id
```

---

## 🛠️ API Overview

### `HybridLogicalClock`

- `new(node_id: u16) -> HybridLogicalClock`: Creates a new clock with a unique node ID.
- `update(&mut self, external_timestamp: u64)`: Updates the clock based on an external timestamp.
- `current_timestamp(&self) -> u64`: Returns the current timestamp of the clock.

### `HLCId`

- `generate(clock: &mut HybridLogicalClock) -> HLCId`: Generates a new HLC-based ID.
- `to_u128(&self) -> u128`: Converts the ID to a 128-bit integer.
- `from_u128(id: u128) -> HLCId`: Reconstructs an ID from a 128-bit integer.
- `encode_base64(&self) -> String`: Encodes the ID to Base64.
- `decode_base64(encoded: &str) -> Result<HLCId, String>`: Decodes a Base64 string back into an ID.

---

## 📚 Documentation

Read the full documentation on [docs.rs](https://docs.rs/hlc_id).

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).

---

## 🌟 Contributing

Contributions are welcome! Feel free to submit issues, create pull requests, or suggest new features.

---

## 🛠️ Author

Developed by **Malo Henry**.  
GitHub: [MaloHenry](https://github.com/MaloHenry)  
