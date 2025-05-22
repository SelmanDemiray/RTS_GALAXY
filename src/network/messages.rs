use serde::{Deserialize, Serialize};
use crate::entity::Unit;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    ChatMessage(String),
    GameState(Vec<Unit>),
    PlayerAction { unit_id: u32, target_x: f32, target_y: f32 },
}
