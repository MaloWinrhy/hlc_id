use crate::clock::HybridLogicalClock;

pub struct HLCId {
    pub timestamp: u64,
    pub sequence: u16,
    pub node_id: u16,
}

