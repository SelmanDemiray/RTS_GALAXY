use serde::{Deserialize, Serialize};
use crate::entity::BuildingType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnitType {
    Worker,
    Fighter,
    Ranger,
    Tank,
    Building,
    Headquarters,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnitStatus {
    Idle,
    Moving,
    Gathering,
    Building,
    Attacking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub unit_type: UnitType,
    pub player_id: usize,
    pub health: f32,
    pub max_health: f32,
    pub status: UnitStatus,
    pub target_x: Option<f32>,
    pub target_y: Option<f32>,
    pub last_move_time: f32,
    pub target_resource_id: Option<u32>,
    pub move_speed: f32,
    pub speed: f32, // Alias for move_speed
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub current_cooldown: f32,
    pub carried_resources: u32,
    pub current_resources: Option<u32>, // Alias for carried_resources
    pub max_carry_capacity: u32,
    pub resource_capacity: Option<u32>, // Alias for max_carry_capacity
    pub building_type: Option<BuildingType>,
    pub construction_progress: Option<f32>,
    pub facing_direction: f32,
    pub is_moving: bool,
    pub is_attacking: bool,
    pub is_gathering: bool,
    pub is_building: bool,
    pub animation: crate::entity::UnitAnimation,
}

impl Unit {
    pub fn new(id: u32, unit_type: UnitType, x: f32, y: f32, player_id: u8) -> Self {
        let (health, max_health, move_speed, attack_damage, attack_range, attack_cooldown, max_carry_capacity) = match unit_type {
            UnitType::Worker => (50.0, 50.0, 80.0, 10.0, 20.0, 1.0, 50),
            UnitType::Fighter => (80.0, 80.0, 100.0, 25.0, 40.0, 0.8, 0),
            UnitType::Ranger => (60.0, 60.0, 90.0, 30.0, 80.0, 1.2, 0),
            UnitType::Tank => (150.0, 150.0, 60.0, 50.0, 50.0, 1.5, 0),
            UnitType::Building => (200.0, 200.0, 0.0, 0.0, 0.0, 0.0, 0),
            UnitType::Headquarters => (500.0, 500.0, 0.0, 0.0, 0.0, 0.0, 0),
        };
        
        Unit {
            id,
            unit_type,
            x,
            y,
            health,
            max_health,
            player_id: player_id as usize,
            status: UnitStatus::Idle,
            target_x: None,
            target_y: None,
            last_move_time: 0.0,
            target_resource_id: None,
            move_speed,
            speed: move_speed, // Alias
            attack_damage,
            attack_range,
            attack_cooldown,
            current_cooldown: 0.0,
            carried_resources: 0,
            current_resources: None, // Alias
            max_carry_capacity,
            resource_capacity: Some(max_carry_capacity), // Alias
            building_type: None,
            construction_progress: None,
            facing_direction: 0.0,
            is_moving: false,
            is_attacking: false,
            is_gathering: false,
            is_building: false,
            animation: crate::entity::UnitAnimation::new(),
        }
    }

    pub fn update_animation_state(&mut self) {
        if self.health <= 0.0 {
            self.animation.set_state(crate::entity::UnitAnimationState::Dying, false);
        } else if self.is_attacking && self.current_cooldown > 0.0 {
            self.animation.set_state(crate::entity::UnitAnimationState::Attacking, false);
        } else if self.is_gathering {
            self.animation.set_state(crate::entity::UnitAnimationState::Gathering, true);
        } else if self.is_building {
            self.animation.set_state(crate::entity::UnitAnimationState::Building, true);
        } else if self.is_moving {
            let anim_state = if self.speed > 80.0 {
                crate::entity::UnitAnimationState::Running
            } else {
                crate::entity::UnitAnimationState::Walking
            };
            self.animation.set_state(anim_state, true);
        } else {
            self.animation.set_state(crate::entity::UnitAnimationState::Idle, true);
        }
    }
}
