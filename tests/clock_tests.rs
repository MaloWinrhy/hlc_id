use hlc_id::clock::HybridLogicalClock;

#[test]
fn test_hlc_initialization() {
    let node_id = 42;
    let hlc = HybridLogicalClock::new(node_id);
    assert_eq!(hlc.node_id(), node_id);
}

#[test]
fn test_hlc_update() {
    let node_id = 42;
    let mut hlc = HybridLogicalClock::new(node_id);

    let current_timestamp = hlc.current_timestamp();
    let external_timestamp = current_timestamp + 100;

    hlc.update(external_timestamp);

    assert_eq!(hlc.current_timestamp(), external_timestamp);
}