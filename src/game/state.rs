use macroquad::prelude::*;
use crate::entity::{Unit, Player, UnitType};
use crate::game::modes::GameMode;
use crate::game::screens::GameScreen;
use crate::game::types::ResourceType;
use crate::game::commands::Command;
use crate::game::resources::ResourceNode;
use crate::game::zoom::ZoomSystem;
use crate::network::NetworkMessage;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Helper struct for Rect serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl From<Rect> for SerializableRect {
    fn from(rect: Rect) -> Self {
        Self { x: rect.x, y: rect.y, w: rect.w, h: rect.h }
    }
}

impl From<SerializableRect> for Rect {
    fn from(rect: SerializableRect) -> Self {
        Rect::new(rect.x, rect.y, rect.w, rect.h)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    // Core game state
    pub units: Vec<Unit>,
    pub players: Vec<Player>,
    pub resource_nodes: Vec<ResourceNode>,
    pub current_player_id: usize, // Changed from u8 to usize
    pub next_unit_id: u32,
    
    // Camera and view
    pub camera_x: f32,
    pub camera_y: f32,
    pub camera_zoom: f32,
    pub zoom_system: ZoomSystem,
    
    // Game state
    pub current_screen: GameScreen,
    pub game_mode: GameMode,
    pub should_quit: bool,
    pub game_time: f32,
    
    // Selection and interaction
    pub selected_units: Vec<u32>,
    pub selection_start: Option<(f32, f32)>,
    pub selection_end: Option<(f32, f32)>,
    pub current_command: Option<Command>,
    
    // UI state
    #[serde(with = "rect_serde")]
    pub minimap_rect: Rect,
    pub map_width: f32,
    pub map_height: f32,
    pub messages: Vec<String>,
    
    // Audio settings
    pub sound_volume: f32,
    pub music_volume: f32,
    pub sound_muted: bool,
    pub music_muted: bool,
}

mod rect_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(rect: &Rect, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serializable_rect = SerializableRect::from(*rect);
        serializable_rect.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Rect, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serializable_rect = SerializableRect::deserialize(deserializer)?;
        Ok(Rect::from(serializable_rect))
    }
}

impl GameState {
    pub fn new() -> Self {
        let mut zoom_system = ZoomSystem::new();
        
        // Create initial players
        let mut players = vec![
            Player::new(0, "Player 1".to_string(), BLUE),
            Player::new(1, "AI Player".to_string(), RED),
        ];
        
        // Create initial units and set home position
        let mut units = Vec::new();
        let hq_x = 400.0;
        let hq_y = 300.0;
        
        // Set zoom system home position to HQ
        zoom_system.set_home_position(Vec2::new(hq_x, hq_y));
        
        // Player 1 starting units
        units.push(Unit::new(0, UnitType::Headquarters, hq_x, hq_y, 0));
        units.push(Unit::new(1, UnitType::Worker, hq_x + 50.0, hq_y + 50.0, 0));
        units.push(Unit::new(2, UnitType::Worker, hq_x - 50.0, hq_y + 50.0, 0));
        
        // AI starting units
        let ai_hq_x = 1200.0;
        let ai_hq_y = 900.0;
        units.push(Unit::new(3, UnitType::Headquarters, ai_hq_x, ai_hq_y, 1));
        units.push(Unit::new(4, UnitType::Worker, ai_hq_x + 50.0, ai_hq_y + 50.0, 1));
        
        // Give starting resources
        players[0].minerals = 500;
        players[0].energy = 200;
        players[1].minerals = 500;
        players[1].energy = 200;
        
        Self {
            units,
            players,
            resource_nodes: Self::generate_resource_nodes(),
            current_player_id: 0,
            next_unit_id: 5,
            
            // Start camera at player HQ
            camera_x: hq_x,
            camera_y: hq_y,
            camera_zoom: 1.0,
            zoom_system,
            
            current_screen: GameScreen::MainMenu,
            game_mode: GameMode::Offline,
            should_quit: false,
            game_time: 0.0,
            
            selected_units: Vec::new(),
            selection_start: None,
            selection_end: None,
            current_command: None,
            
            minimap_rect: Rect::new(screen_width() - 210.0, 10.0, 200.0, 150.0),
            map_width: 2000.0,
            map_height: 1500.0,
            messages: Vec::new(),
            
            sound_volume: 0.7,
            music_volume: 0.5,
            sound_muted: false,
            music_muted: false,
        }
    }
    
    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.game_time += dt;
        
        // Update zoom system
        self.zoom_system.update(dt);
        
        // Handle input
        self.handle_input();
        
        // Update units
        self.update_units(dt);
        
        // Update camera bounds
        self.update_camera_bounds();
    }
    
    pub fn request_quit(&mut self) {
        self.should_quit = true;
    }
    
    pub fn can_afford(&self, player_id: usize, unit_type: &UnitType) -> bool {
        if let Some(player) = self.players.get(player_id) {
            let (mineral_cost, energy_cost) = self.get_unit_cost(unit_type);
            player.minerals >= mineral_cost && player.energy >= energy_cost
        } else {
            false
        }
    }
    
    pub fn spawn_unit(&mut self, unit_type: UnitType, x: f32, y: f32, player_id: usize) -> u32 {
        let unit_id = self.next_unit_id;
        self.next_unit_id += 1;
        
        let unit = Unit::new(unit_id, unit_type, x, y, player_id as u8);
        self.units.push(unit);
        
        unit_id
    }
    
    pub fn deduct_cost(&mut self, player_id: usize, unit_type: &UnitType) {
        let (mineral_cost, energy_cost) = self.get_unit_cost(unit_type);
        if let Some(player) = self.players.get_mut(player_id) {
            player.minerals -= mineral_cost;
            player.energy -= energy_cost;
        }
    }
    
    fn get_unit_cost(&self, unit_type: &UnitType) -> (i32, i32) {
        match unit_type {
            UnitType::Worker => (50, 0),
            UnitType::Fighter => (100, 25),
            UnitType::Ranger => (75, 25),
            UnitType::Tank => (150, 50),
            UnitType::Building => (200, 0),
            UnitType::Headquarters => (500, 0),
        }
    }
    
    fn generate_resource_nodes() -> Vec<ResourceNode> {
        vec![
            ResourceNode::new(300.0, 200.0, 1000, ResourceType::Minerals, 25.0),
            ResourceNode::new(800.0, 400.0, 800, ResourceType::Energy, 20.0),
            ResourceNode::new(1500.0, 700.0, 1200, ResourceType::Minerals, 30.0),
            ResourceNode::new(200.0, 900.0, 600, ResourceType::Energy, 15.0),
        ]
    }
    
    fn handle_input(&mut self) {
        // Zoom controls
        if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
            self.zoom_system.zoom_in();
        }
        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
            self.zoom_system.zoom_out();
        }
        
        // Mouse wheel zoom
        let (_wheel_x, wheel_y) = mouse_wheel();
        if wheel_y > 0.0 {
            self.zoom_system.zoom_in();
        } else if wheel_y < 0.0 {
            self.zoom_system.zoom_out();
        }
        
        // Home key - return to HQ
        if is_key_pressed(KeyCode::Home) || is_key_pressed(KeyCode::H) {
            let home_pos = self.zoom_system.go_home();
            self.camera_x = home_pos.x;
            self.camera_y = home_pos.y;
        }
        
        // Camera movement with zoom-adjusted speed
        let zoom_scale = self.zoom_system.get_current_scale() as f32;
        let base_speed = 200.0;
        let camera_speed = base_speed * (zoom_scale / 1000.0).clamp(0.1, 50.0);
        
        let dt = get_frame_time();
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.camera_y -= camera_speed * dt;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.camera_y += camera_speed * dt;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.camera_x -= camera_speed * dt;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.camera_x += camera_speed * dt;
        }
    }
    
    fn update_units(&mut self, dt: f32) {
        for unit in &mut self.units {
            // Update unit movement
            if let (Some(target_x), Some(target_y)) = (unit.target_x, unit.target_y) {
                let dx = target_x - unit.x;
                let dy = target_y - unit.y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance > 5.0 {
                    let move_distance = unit.speed * dt;
                    unit.x += (dx / distance) * move_distance;
                    unit.y += (dy / distance) * move_distance;
                    
                    // Update facing direction
                    unit.facing_direction = dy.atan2(dx);
                    unit.is_moving = true;
                } else {
                    unit.target_x = None;
                    unit.target_y = None;
                    unit.is_moving = false;
                }
            } else {
                unit.is_moving = false;
            }
            
            // Update cooldowns
            if unit.current_cooldown > 0.0 {
                unit.current_cooldown -= dt;
            }
            
            // Update animation state based on unit status
            unit.update_animation_state();
            
            // Update animation timing
            unit.animation.update(dt);
        }
    }
    
    fn update_camera_bounds(&mut self) {
        // Keep camera within map bounds
        let half_screen_w = screen_width() / 2.0;
        let half_screen_h = screen_height() / 2.0;
        
        self.camera_x = self.camera_x.clamp(half_screen_w, self.map_width - half_screen_w);
        self.camera_y = self.camera_y.clamp(half_screen_h, self.map_height - half_screen_h);
    }
}
