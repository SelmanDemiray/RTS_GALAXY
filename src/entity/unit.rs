use crate::entity::building::BuildingType;
use crate::entity::types::{UnitType, UnitAnimationState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: u64,
    pub unit_type: UnitType,
    pub x: f32,
    pub y: f32,
    pub health: f32,
    pub max_health: f32,
    pub player_id: usize,
    pub selected: bool,
    pub building_type: Option<BuildingType>,
    pub construction_progress: Option<f32>,
    pub last_attack_time: f32,
    pub animation_time: f32,
    pub current_animation: String,
    pub animation_speed: f32,
    pub energy: f32,
    pub max_energy: f32,
    pub level: i32,
    pub experience: i32,
    
    // Additional fields that were missing
    pub current_resources: Option<i32>,
    pub target_x: Option<f32>,
    pub target_y: Option<f32>,
}

impl Unit {
    pub fn new(id: u64, unit_type: UnitType, x: f32, y: f32, player_id: usize) -> Self {
        let max_health = unit_type.get_health();
        Self {
            id,
            unit_type,
            x,
            y,
            health: max_health,
            max_health,
            player_id,
            selected: false,
            building_type: None,
            construction_progress: None,
            last_attack_time: 0.0,
            animation_time: 0.0,
            current_animation: "idle".to_string(),
            animation_speed: 1.0,
            energy: 100.0,
            max_energy: 100.0,
            level: 1,
            experience: 0,
            current_resources: None,
            target_x: None,
            target_y: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitAnimation {
    pub current_state: UnitAnimationState,
    pub animation_time: f32,
    pub animation_speed: f32,
    pub loop_animation: bool,
    pub transition_time: f32,
    pub next_state: Option<UnitAnimationState>,
}

impl UnitAnimation {
    pub fn new() -> Self {
        Self {
            current_state: UnitAnimationState::Idle,
            animation_time: 0.0,
            animation_speed: 1.0,
            loop_animation: true,
            transition_time: 0.0,
            next_state: None,
        }
    }

    pub fn set_state(&mut self, new_state: UnitAnimationState, should_loop: bool) {
        self.current_state = new_state;
        self.loop_animation = should_loop;
        self.animation_time = 0.0;
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(next_state) = &self.next_state {
            self.transition_time -= dt;
            if self.transition_time <= 0.0 {
                self.current_state = next_state.clone();
                self.next_state = None;
                self.animation_time = 0.0;
            }
        }
        
        self.animation_time += dt * self.animation_speed;
    }
}

impl Unit {
    pub fn update(&mut self, dt: f32) {
        self.animation.update(dt);
        
        // Update cooldown
        if self.current_cooldown > 0.0 {
            self.current_cooldown -= dt;
        }

        // Update animation state based on unit state
        if self.health <= 0.0 {
            self.animation.set_state(UnitAnimationState::Dying, false);
        } else if self.is_attacking && self.current_cooldown > 0.0 {
            self.animation.set_state(UnitAnimationState::Attacking, false);
        } else if self.is_gathering {
            self.animation.set_state(UnitAnimationState::Gathering, true);
        } else if self.is_building {
            self.animation.set_state(UnitAnimationState::Building, true);
        } else if self.is_moving {
            let anim_state = if self.speed > 80.0 {
                UnitAnimationState::Running
            } else {
                UnitAnimationState::Walking
            };
            self.animation.set_state(anim_state, true);
        } else {
            self.animation.set_state(UnitAnimationState::Idle, true);
        }
    }
}
