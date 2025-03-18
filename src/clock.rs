use chrono::Utc;
use base64::{engine::general_purpose::STANDARD as Engine, Engine as _};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HybridLogicalClock {
    timestamp: u64,
    sequence: u16,
    node_id: u16,
    initialized: bool,
}

impl HybridLogicalClock {
    pub fn new(node_id: u16) -> Self {
        Self {
            timestamp: 0,
            sequence: 0,
            node_id,
            initialized: false,
        }
    }

    pub fn update(&mut self, external_timestamp: u64) {
        if !self.initialized || external_timestamp > self.timestamp {
            self.timestamp = external_timestamp;
            self.sequence = 0;
            self.initialized = true;
        } else if external_timestamp == self.timestamp {
            self.sequence += 1;
        } else {
            self.timestamp = Utc::now().timestamp_millis() as u64;
            self.sequence = 0;
        }
    }

    pub fn current_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn node_id(&self) -> u16 {
        self.node_id
    }

    pub fn current_sequence(&self) -> u16 {
        self.sequence
    }
}