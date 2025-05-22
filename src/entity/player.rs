use macroquad::prelude::Color;

pub struct Player {
    #[allow(dead_code)]
    pub id: u8,
    pub minerals: i32,
    pub energy: i32,
    pub color: Color,
    #[allow(dead_code)]
    pub is_ai: bool,
}
