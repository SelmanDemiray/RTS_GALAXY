use macroquad::prelude::*;
use crate::entity::{Unit, Player, UnitType};
use crate::game::modes::GameMode;
use crate::game::screens::GameScreen;
use crate::game::types::ResourceType;
use crate::game::commands::Command;
use crate::game::resources::ResourceNode;
use crate::game::zoom::ZoomSystem; // Add this import
use crate::network::NetworkMessage;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    // Core game state
    pub units: Vec<Unit>,
    pub players: Vec<Player>,
    pub resource_nodes: Vec<ResourceNode>,
    pub current_player_id: u8,
    pub next_unit_id: u32,
    
    // Camera and view
    pub camera_x: f32,
    pub camera_y: f32,
    pub camera_zoom: f32, // Keep for backward compatibility
    pub zoom_system: ZoomSystem, // New zoom system
    
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

impl GameState {
    pub fn new() -> Self {
        let mut zoom_system = ZoomSystem::new();
        
        // Create initial players
        let mut players = vec![
            Player::new(0, "Player 1", BLUE),
            Player::new(1, "AI Player", RED),
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
        
        // Number keys for quick zoom levels
        for i in 1..=9 {
            if is_key_pressed(match i {
                1 => KeyCode::Key1,
                2 => KeyCode::Key2,
                3 => KeyCode::Key3,
                4 => KeyCode::Key4,
                5 => KeyCode::Key5,
                6 => KeyCode::Key6,
                7 => KeyCode::Key7,
                8 => KeyCode::Key8,
                9 => KeyCode::Key9,
                _ => continue,
            }) {
                // Map number keys to useful zoom levels
                let target_level = match i {
                    1 => 1,   // Unit detail
                    2 => 5,   // Village
                    3 => 10,  // City
                    4 => 15,  // Region
                    5 => 20,  // Continent
                    6 => 25,  // Solar system
                    7 => 30,  // Local cluster
                    8 => 35,  // Galaxy
                    9 => 40,  // Supercluster
                    _ => continue,
                };
                self.zoom_system.set_zoom_level(target_level);
            }
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
        
        // ...existing input handling code...
    }
    
    // ...existing code...
}
