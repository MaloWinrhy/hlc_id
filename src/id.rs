use chrono::Utc;
use base64::{engine::general_purpose::STANDARD as Engine, Engine as _};

use crate::clock::HybridLogicalClock;

#[derive(Debug)]
pub struct HLCId {
    pub timestamp: u64,
    pub sequence: u16,
    pub node_id: u16,
}

impl HLCId {

    pub fn generate(clock: &mut HybridLogicalClock) -> Self {
        let current_timestamp = Utc::now().timestamp_millis() as u64;
        clock.update(current_timestamp);
    
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

    pub fn encode_base64(&self) -> String {
        let id = self.to_u128();
        Engine.encode(&id.to_be_bytes())
    }

    pub fn decode_base64(encoded: &str) -> Result<Self, String> {
        let decoded = Engine.decode(encoded).map_err(|e| e.to_string())?;
        let id_as_u128 = u128::from_be_bytes(
            decoded
                .try_into()
                .map_err(|_| "Failed to convert decoded bytes to u128".to_string())?,
        );
        Ok(Self::from_u128(id_as_u128))
    }
}

