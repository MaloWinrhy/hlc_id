use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
            self.timestamp = external_timestamp;
            self.sequence = 0;
        }
    }

    pub fn process_timestamp(&mut self, received_timestamp: u64) {
        if received_timestamp > self.timestamp {
            self.update(received_timestamp);
        }
    }

    pub fn save_state(&self, path: &str) -> std::io::Result<()> {
        let file = OpenOptions::new().write(true).create(true).open(path)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }

    pub fn load_state(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(serde_json::from_str(&data)?)
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