use hlc_id::{clock::HybridLogicalClock, id::HLCId};
use chrono::Utc;

fn main() {
    let mut clock = HybridLogicalClock::new(42); // Node ID = 42

    let hlc_id = HLCId::generate(&mut clock, Utc::now().timestamp_millis() as u64);
    println!("Generated HLC ID: {:?}", hlc_id);

    let encoded = hlc_id.encode_base64();
    println!("Encoded Base64: {}", encoded);

    let decoded = HLCId::decode_base64(&encoded).unwrap();
    println!("Decoded HLC ID: {:?}", decoded);
}