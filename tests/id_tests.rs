use hlc_id::{clock::HybridLogicalClock, id::HLCId};
use chrono::Utc;

#[test]
fn test_hlc_id_generation() {
    let mut clock = HybridLogicalClock::new(42);
    
    // On capture le même timestamp pour s'assurer que la séquence s'incrémente bien.
    let timestamp = Utc::now().timestamp_millis() as u64;

    let hlc_id = HLCId::generate(&mut clock, timestamp);
    println!(
        "Generated ID: timestamp={}, sequence={}, node_id={}",
        hlc_id.timestamp, hlc_id.sequence, hlc_id.node_id
    );

    assert_eq!(hlc_id.sequence, 0);

    let hlc_id2 = HLCId::generate(&mut clock, timestamp);
    
    assert_eq!(hlc_id2.sequence, 1, "Sequence should be incremented!");
    assert_eq!(hlc_id2.timestamp, hlc_id.timestamp);
}


#[test]
fn test_hlc_id_to_u128() {
    let mut clock = HybridLogicalClock::new(42);
    let hlc_id = HLCId::generate(&mut clock, Utc::now().timestamp_millis() as u64);
    let id_as_u128 = hlc_id.to_u128();

    let reconstructed_id = HLCId::from_u128(id_as_u128);
    assert_eq!(reconstructed_id.timestamp, hlc_id.timestamp);
    assert_eq!(reconstructed_id.sequence, hlc_id.sequence);
    assert_eq!(reconstructed_id.node_id, hlc_id.node_id);
}

#[test]
fn test_hlc_id_base64_encoding() {
    let mut clock = HybridLogicalClock::new(42); 
    let hlc_id = HLCId::generate(&mut clock, Utc::now().timestamp_millis() as u64);

    let encoded = hlc_id.encode_base64();
    println!("Encoded HLC ID: {}", encoded);

    let decoded_hlc_id = HLCId::decode_base64(&encoded).unwrap();

    assert_eq!(decoded_hlc_id.timestamp, hlc_id.timestamp);
    assert_eq!(decoded_hlc_id.node_id, hlc_id.node_id);
    assert_eq!(decoded_hlc_id.sequence, hlc_id.sequence);
}