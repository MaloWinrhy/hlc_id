pub struct HybridLogicalClock {
    timestamp: u64,
    node_id: u16,
}

impl HybridLogicalClock {

    pub fn new(node_id : u16) -> HybridLogicalClock {
        HybridLogicalClock {
            timestamp: 0,
            node_id: node_id,
        }
    }

    pub fn current_timestamp(&self) -> u64 {
        self.timestamp
    }
}