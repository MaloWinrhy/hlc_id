use crate::clock::HybridLogicalClock;

pub struct HLCId {
    pub timestamp: u64,
    pub sequence: u16,
    pub node_id: u16,
}

impl HLCId {

    pub fn generate(clock: &mut HybridLogicalClock) -> Self {

        clock.update(clock.current_timestamp());

        HLCId {
            timestamp: clock.current_timestamp(),
            sequence: clock.current_sequence(),
            node_id: clock.node_id(),
        }
    }

    pub fn to_u128(&self) -> u128 {
        ((self.timestamp as u128) << 64)
         | ((self.node_id as u128) << 48)
         | self.sequence as u128
    }

    pub fn from_u128(id: u128) -> Self {
        let timestamp = (id >> 64) as u64;
        let node_id = ((id >> 48) & 0x3FF) as u16;
        let sequence = (id & 0x3FFFF) as u16;
        
        HLCId {
            timestamp,
            node_id,
            sequence,
        }
    }
}

