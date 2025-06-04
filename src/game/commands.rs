use crate::entity::{UnitType, BuildingType};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Move { x: f32, y: f32 },
    Attack { target_id: u32 },
    Gather { resource_id: usize },
    Build { building_type: BuildingType, x: f32, y: f32 },
    Train { unit_type: UnitType },
    Stop,
}
