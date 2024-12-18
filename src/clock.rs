use chrono::Utc;

pub struct HybridLogicalClock {
    timestamp: u64,
    sequence: u16,
    node_id: u16,
}

impl HybridLogicalClock {

    pub fn new(node_id : u16) -> HybridLogicalClock {
        let timestamp = Utc::now().timestamp_millis() as u64;
        HybridLogicalClock {
            timestamp,
            sequence: 0,
            node_id,
        }
    }

    pub fn update(&mut self, external_timestamp: u64) {
        self.timestamp = self.timestamp.max(external_timestamp);
    }

    pub fn current_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn node_id(&self) -> u16 {
        self.node_id
    }
}