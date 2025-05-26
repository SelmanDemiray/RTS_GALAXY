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
    Barracks,
    Factory,
    ResourceDepot,
    DefenseTurret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub unit_type: UnitType,
    pub x: f32,
    pub y: f32,
    pub health: f32,
    pub max_health: f32,
    pub player_id: usize, // Changed from u8 to usize
    pub speed: f32,
    pub target_x: Option<f32>,
    pub target_y: Option<f32>,
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub current_cooldown: f32,
    pub current_resources: Option<u32>,
    pub resource_capacity: Option<u32>,
    // Add building-related fields
    pub building_type: Option<BuildingType>,
    pub construction_progress: Option<f32>,
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
            player_id: player_id as usize, // Convert to usize
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
        }
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
    pub id: usize, // Changed from u8 to usize
    pub minerals: i32,
    pub energy: i32,
    #[serde(with = "color_serde")]
    pub color: Color,
    pub is_ai: bool,
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
