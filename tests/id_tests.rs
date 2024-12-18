use hlc_id::{clock::HybridLogicalClock, id::HLCId};

#[test]
fn test_hlc_id_generation() {
    let mut clock = HybridLogicalClock::new(42); 
    let hlc_id = HLCId::generate(&mut clock);

    assert_eq!(hlc_id.node_id, 42);
    assert!(hlc_id.timestamp > 0);
    assert_eq!(hlc_id.sequence, 0);
}

#[test]
fn test_hlc_id_to_u128() {
    let mut clock = HybridLogicalClock::new(42);
    let hlc_id = HLCId::generate(&mut clock);
    let id_as_u128 = hlc_id.to_u128();

    // Reconstruire à partir de u128
    let reconstructed_id = HLCId::from_u128(id_as_u128);
    assert_eq!(reconstructed_id.timestamp, hlc_id.timestamp);
    assert_eq!(reconstructed_id.sequence, hlc_id.sequence);
    assert_eq!(reconstructed_id.node_id, hlc_id.node_id);
}