use serde::{Deserialize, Serialize};
use crate::entity::Unit;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetworkMessage {
    GameState {
        units: Vec<Unit>,
        timestamp: f64,
    },
    UnitUpdate {
        unit_id: u32,
        x: f32,
        y: f32,
    },
    PlayerJoined {
        player_id: u8,
        name: String,
    },
    PlayerLeft {
        player_id: u8,
    },
    ChatMessage {
        player_id: u8,
        message: String,
    },
}
