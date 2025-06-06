use serde::{Deserialize, Serialize};
use crate::entity::BuildingType;
use crate::entity::unit::UnitType;

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
    Train { unit_type: UnitType },
    Stop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameMode {
    Offline,
    Online,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameCommand {
    Move { unit_id: u64, x: f32, y: f32 },
    Attack { attacker_id: u64, target_id: u64 },
    Build { builder_id: u64, building_type: String, x: f32, y: f32 },
    Train { unit_type: UnitType },
    Gather { unit_id: u64, resource_id: u64 },
    Stop { unit_id: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerResources {
    pub minerals: u32,
    pub energy: u32,
}

impl PlayerResources {
    pub fn new() -> Self {
        Self {
            minerals: 500,
            energy: 200,
        }
    }

    pub fn can_afford(&self, cost_minerals: u32, cost_energy: u32) -> bool {
        self.minerals >= cost_minerals && self.energy >= cost_energy
    }

    pub fn spend(&mut self, cost_minerals: u32, cost_energy: u32) -> bool {
        if self.can_afford(cost_minerals, cost_energy) {
            self.minerals -= cost_minerals;
            self.energy -= cost_energy;
            true
        } else {
            false
        }
    }
}
