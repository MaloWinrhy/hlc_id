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
         | ((self.node_id as u128) << 46)
         | self.sequence as u128
    }
}

