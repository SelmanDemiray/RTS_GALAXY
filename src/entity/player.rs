use macroquad::prelude::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializableColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
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
    pub id: u8,
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

impl Player {
    // ...existing methods...
}
