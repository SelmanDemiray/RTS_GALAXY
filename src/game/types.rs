use serde::{Deserialize, Serialize};
use crate::entity::BuildingType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    Minerals,
    Energy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Move { x: f32, y: f32 },
    Attack { target_id: u32 },
    Gather { resource_id: u32 },
    Build { building_type: BuildingType, x: f32, y: f32 },
    Train { unit_type: crate::entity::UnitType },
    Stop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameMode {
    Offline,
    Online,
}
