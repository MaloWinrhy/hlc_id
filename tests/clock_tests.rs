use hlc_id::clock::HybridLogicalClock;

#[test]
fn test_hlc_initialization() {
    let node_id = 42;
    let mut hlc = HybridLogicalClock::new(node_id);

    let initial_timestamp = hlc.current_timestamp();
    let initial_sequence = hlc.current_sequence();

    println!("Initial timestamp: {}", initial_timestamp);
    println!("Initial sequence: {}", initial_sequence);

    hlc.update(initial_timestamp);
    println!(
        "After first update: timestamp={}, sequence={}",
        hlc.current_timestamp(),
        hlc.current_sequence()
    );

    assert_eq!(hlc.current_sequence(), initial_sequence);

    hlc.update(initial_timestamp);
    println!(
        "After second update: timestamp={}, sequence={}",
        hlc.current_timestamp(),
        hlc.current_sequence()
    );

    assert_eq!(hlc.current_sequence(), initial_sequence + 1);
}

#[test]
fn test_hlc_update() {
    let node_id = 42;
    let mut hlc = HybridLogicalClock::new(node_id);

    let intial_timestamp = hlc.current_timestamp();

    hlc.update(intial_timestamp + 1);

    assert_eq!(hlc.current_sequence(), 0);
    assert_eq!(hlc.current_timestamp(), intial_timestamp + 1);
}