use serde::{Deserialize, Serialize};
use crate::entity::types::{UnitType, BuildingType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub health: i32,
    pub max_health: i32,
    pub unit_type: UnitType,
    pub player_id: u8,
    pub target_x: Option<f32>,
    pub target_y: Option<f32>,
    pub speed: f32,
    pub attack_damage: i32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub current_cooldown: f32,
    pub building_type: Option<BuildingType>,
    pub construction_progress: Option<f32>,
    pub resource_capacity: Option<i32>,
    pub current_resources: Option<i32>,
}

impl Unit {
    pub fn new(id: u32, x: f32, y: f32, unit_type: UnitType, player_id: u8) -> Self {
        let (health, speed, damage, range) = match unit_type {
            UnitType::Worker => (50, 2.5, 5, 20.0),
            UnitType::Fighter => (80, 3.0, 15, 30.0),
            UnitType::Ranger => (60, 2.8, 20, 150.0),
            UnitType::Tank => (150, 1.5, 30, 60.0),
            UnitType::Building => (200, 0.0, 0, 0.0),
            UnitType::Headquarters => (500, 0.0, 0, 0.0),
        };

        let resource_capacity = if unit_type == UnitType::Worker {
            Some(50)
        } else {
            None
        };

        let building_type = if unit_type == UnitType::Building || unit_type == UnitType::Headquarters {
            Some(BuildingType::Headquarters) 
        } else {
            None
        };

        Self {
            id,
            x,
            y,
            health,
            max_health: health,
            unit_type,
            player_id,
            target_x: None,
            target_y: None,
            speed,
            attack_damage: damage,
            attack_range: range,
            attack_cooldown: 1.0,
            current_cooldown: 0.0,
            building_type,
            construction_progress: None,
            resource_capacity,
            current_resources: if resource_capacity.is_some() { Some(0) } else { None },
        }
    }

    #[allow(dead_code)]
    pub fn is_completed_building(&self) -> bool {
        if let (Some(_building_type), Some(progress)) = (&self.building_type, self.construction_progress) {
            progress >= 100.0
        } else {
            false
        }
    }
}
