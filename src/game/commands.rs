use crate::entity::{UnitType, BuildingType};

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    #[allow(dead_code)]
    Move { x: f32, y: f32 },
    Attack { target_id: u32 },
    Gather { resource_id: usize },
    Build { building_type: BuildingType, x: f32, y: f32 },
    #[allow(dead_code)]
    Train { unit_type: UnitType },
    Stop,
}
