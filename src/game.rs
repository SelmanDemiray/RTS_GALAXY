use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    ChatMessage(String),
    GameState(Vec<Unit>),
    PlayerAction { unit_id: u32, target_x: f32, target_y: f32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub health: i32,
    pub unit_type: UnitType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UnitType {
    Worker,
    Soldier,
    Building,
}

pub struct GameState {
    pub units: Vec<Unit>,
    pub selected_unit: Option<u32>,
    pub messages: Vec<String>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            units: vec![
                Unit { id: 1, x: 100.0, y: 100.0, health: 100, unit_type: UnitType::Worker },
                Unit { id: 2, x: 200.0, y: 150.0, health: 100, unit_type: UnitType::Soldier },
            ],
            selected_unit: None,
            messages: Vec::new(),
        }
    }
    
    pub fn update(&mut self) {
        // Basic game logic update - would handle unit movement, combat, etc.
    }
    
    pub fn draw(&self) {
        // Draw all game units
        for unit in &self.units {
            let color = match unit.unit_type {
                UnitType::Worker => BLUE,
                UnitType::Soldier => RED,
                UnitType::Building => GRAY,
            };
            
            let is_selected = self.selected_unit == Some(unit.id);
            let border_size = if is_selected { 2.0 } else { 0.0 };
            
            draw_circle_lines(unit.x, unit.y, 15.0, border_size, GREEN);
            draw_circle(unit.x, unit.y, 10.0, color);
        }
    }
    
    pub fn handle_network_message(&mut self, msg: NetworkMessage) {
        match msg {
            NetworkMessage::ChatMessage(text) => {
                self.messages.push(text);
            },
            NetworkMessage::GameState(new_units) => {
                self.units = new_units;
            },
            NetworkMessage::PlayerAction { .. } => {
                // Handle remote player actions
            }
        }
    }
    
    pub fn select_unit_at(&mut self, x: f32, y: f32) {
        self.selected_unit = None;
        
        for unit in &self.units {
            let distance = ((unit.x - x).powi(2) + (unit.y - y).powi(2)).sqrt();
            if distance < 15.0 {
                self.selected_unit = Some(unit.id);
                break;
            }
        }
    }
    
    pub fn move_selected_unit(&mut self, x: f32, y: f32) {
        if let Some(selected_id) = self.selected_unit {
            for unit in &mut self.units {
                if unit.id == selected_id {
                    unit.x = x;
                    unit.y = y;
                    break;
                }
            }
        }
    }
}
