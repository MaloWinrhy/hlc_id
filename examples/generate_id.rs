use hlc_id::{clock::HybridLogicalClock, id::HLCId};

fn main() {
    let mut clock = HybridLogicalClock::new(42); // Node ID = 42

    // Générer un identifiant HLC
    let hlc_id = HLCId::generate(&mut clock);
    println!("Generated HLC ID: {:?}", hlc_id);

    // Convertir en Base64
    let encoded = hlc_id.encode_base64();
    println!("Encoded Base64: {}", encoded);

    // Décoder depuis Base64
    let decoded = HLCId::decode_base64(&encoded).unwrap();
    println!("Decoded HLC ID: {:?}", decoded);
}