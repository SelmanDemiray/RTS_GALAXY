use serde::{Deserialize, Serialize};
use macroquad::prelude::*;

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
pub enum BuildingType {
    Headquarters,
    Barracks,
    Factory,
    ResourceDepot,
    DefenseTurret,
    ResearchCenter,
    TurretDefense,
    ResourceCollector,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnitAnimationState {
    Idle,
    Walking,
    Running,
    Attacking,
    Gathering,
    Building,
    Dying,
    Special, // For unit-specific animations
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
    
    pub fn set_state(&mut self, new_state: UnitAnimationState, loop_anim: bool) {
        if self.current_state != new_state {
            self.next_state = Some(new_state);
            self.transition_time = 0.2; // Smooth transition
            self.loop_animation = loop_anim;
        }
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
        
        // Handle non-looping animations
        if !self.loop_animation && self.animation_time >= self.get_animation_duration() {
            // Animation finished, return to idle
            self.set_state(UnitAnimationState::Idle, true);
        }
    }
    
    fn get_animation_duration(&self) -> f32 {
        match self.current_state {
            UnitAnimationState::Idle => 2.0,
            UnitAnimationState::Walking => 1.0,
            UnitAnimationState::Running => 0.8,
            UnitAnimationState::Attacking => 0.6,
            UnitAnimationState::Gathering => 1.5,
            UnitAnimationState::Building => 2.0,
            UnitAnimationState::Dying => 1.2,
            UnitAnimationState::Special => 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub unit_type: UnitType,
    pub x: f32,
    pub y: f32,
    pub health: f32,
    pub max_health: f32,
    pub player_id: usize,
    pub speed: f32,
    pub target_x: Option<f32>,
    pub target_y: Option<f32>,
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub current_cooldown: f32,
    pub current_resources: Option<u32>,
    pub resource_capacity: Option<u32>,
    pub building_type: Option<BuildingType>,
    pub construction_progress: Option<f32>,
    
    // Animation system
    pub animation: UnitAnimation,
    pub facing_direction: f32, // Rotation in radians
    pub is_moving: bool,
    pub is_attacking: bool,
    pub is_gathering: bool,
    pub is_building: bool,
}

impl Unit {
    pub fn new(id: u32, unit_type: UnitType, x: f32, y: f32, player_id: u8) -> Self {
        let (health, speed, attack_damage, attack_range, attack_cooldown, current_resources, resource_capacity) = match unit_type {
            UnitType::Worker => (50.0, 2.0, 5.0, 20.0, 2.0, Some(0), Some(50)),
            UnitType::Fighter => (80.0, 3.0, 15.0, 30.0, 1.5, None, None),
            UnitType::Ranger => (60.0, 2.5, 20.0, 50.0, 2.0, None, None),
            UnitType::Tank => (150.0, 1.5, 30.0, 25.0, 3.0, None, None),
            UnitType::Building => (200.0, 0.0, 0.0, 0.0, 0.0, None, None),
            UnitType::Headquarters => (500.0, 0.0, 0.0, 0.0, 0.0, None, None),
        };

        Self {
            id,
            unit_type,
            x,
            y,
            health,
            max_health: health,
            player_id: player_id as usize,
            speed,
            target_x: None,
            target_y: None,
            attack_damage,
            attack_range,
            attack_cooldown,
            current_cooldown: 0.0,
            current_resources,
            resource_capacity,
            building_type: None,
            construction_progress: None,
            animation: UnitAnimation::new(),
            facing_direction: 0.0,
            is_moving: false,
            is_attacking: false,
            is_gathering: false,
            is_building: false,
        }
    }
    
    pub fn update_animation_state(&mut self) {
        // Determine animation state based on unit status
        if self.health <= 0.0 {
            self.animation.set_state(UnitAnimationState::Dying, false);
        } else if self.is_attacking && self.current_cooldown > 0.0 {
            self.animation.set_state(UnitAnimationState::Attacking, false);
        } else if self.is_gathering {
            self.animation.set_state(UnitAnimationState::Gathering, true);
        } else if self.is_building {
            self.animation.set_state(UnitAnimationState::Building, true);
        } else if self.is_moving {
            let anim_state = if self.speed > 2.5 {
                UnitAnimationState::Running
            } else {
                UnitAnimationState::Walking
            };
            self.animation.set_state(anim_state, true);
        } else {
            self.animation.set_state(UnitAnimationState::Idle, true);
        }
    }
    
    pub fn get_required_animations(&self) -> Vec<String> {
        let base_animations = vec![
            "idle".to_string(),
            "walking".to_string(),
            "dying".to_string(),
        ];
        
        let mut animations = base_animations;
        
        match self.unit_type {
            UnitType::Worker => {
                animations.extend(vec![
                    "gathering_minerals".to_string(),
                    "gathering_energy".to_string(),
                    "building".to_string(),
                    "carrying_resources".to_string(),
                ]);
            },
            UnitType::Fighter => {
                animations.extend(vec![
                    "running".to_string(),
                    "melee_attack".to_string(),
                    "blocking".to_string(),
                    "victory_pose".to_string(),
                ]);
            },
            UnitType::Ranger => {
                animations.extend(vec![
                    "running".to_string(),
                    "aiming".to_string(),
                    "shooting".to_string(),
                    "reloading".to_string(),
                ]);
            },
            UnitType::Tank => {
                animations.extend(vec![
                    "turret_rotate".to_string(),
                    "firing_cannon".to_string(),
                    "damaged_idle".to_string(),
                ]);
            },
            UnitType::Building => {
                animations.extend(vec![
                    "construction".to_string(),
                    "working".to_string(),
                    "damaged".to_string(),
                ]);
            },
            UnitType::Headquarters => {
                animations.extend(vec![
                    "production".to_string(),
                    "damaged".to_string(),
                    "upgrading".to_string(),
                ]);
            },
        }
        
        animations
    }
}

// Create a serializable color wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<Color> for SerializableColor {
    fn from(color: Color) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

impl From<SerializableColor> for Color {
    fn from(color: SerializableColor) -> Self {
        Color::new(color.r, color.g, color.b, color.a)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub minerals: i32,
    pub energy: i32,
    #[serde(with = "color_serde")]
    pub color: Color,
    pub is_ai: bool,
}

impl Player {
    pub fn new(id: usize, name: String, color: Color) -> Self {
        Self {
            id,
            name,
            minerals: 0,
            energy: 0,
            color,
            is_ai: id != 0, // AI if not player 0
        }
    }
}

mod color_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serializable_color = SerializableColor::from(*color);
        serializable_color.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serializable_color = SerializableColor::deserialize(deserializer)?;
        Ok(Color::from(serializable_color))
    }
}
