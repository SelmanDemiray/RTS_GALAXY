use crate::entity::{Unit, Player, UnitType, BuildingType};
use crate::entity::resource_node::ResourceNode;
use crate::game::zoom::ZoomSystem;
use macroquad::prelude::*;
use macroquad::color::{CYAN, YELLOW, BLUE}; // Add color imports
use crate::resources::manager::ResourceManager;
use serde::{Serialize, Deserialize};
use crate::game::types::{GameCommand, PlayerResources};

#[derive(Debug, Clone, PartialEq)]
pub enum GameScreen {
    MainMenu,
    Settings,
    Credits,
    Playing,
    Quit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableVec2 {
    pub x: f32,
    pub y: f32,
}

impl From<Vec2> for SerializableVec2 {
    fn from(v: Vec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<SerializableVec2> for Vec2 {
    fn from(v: SerializableVec2) -> Self {
        Vec2::new(v.x, v.y)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl From<Rect> for SerializableRect {
    fn from(r: Rect) -> Self {
        Self { x: r.x, y: r.y, w: r.w, h: r.h }
    }
}

impl From<SerializableRect> for Rect {
    fn from(r: SerializableRect) -> Self {
        Rect::new(r.x, r.y, r.w, r.h)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub units: Vec<Unit>,
    pub players: Vec<Player>,
    pub resource_nodes: Vec<ResourceNode>,
    pub selected_units: Vec<u64>,
    pub current_screen: GameScreen,
    pub current_player: usize,
    pub camera_x: f32,
    pub camera_y: f32,
    pub is_running: bool,
    #[serde(with = "serializable_vec2_option")]
    pub selection_start: Option<Vec2>,
    #[serde(with = "serializable_vec2_option")]
    pub selection_end: Option<Vec2>,
    #[serde(with = "serializable_rect")]
    pub minimap_rect: Rect,
    
    // Additional fields that were missing
    pub paused: bool,
    pub game_time: f32,
    pub last_update_time: f32,
    pub sound_muted: bool,
    pub music_muted: bool,
    pub sound_volume: f32,
    pub music_volume: f32,
    pub zoom_system: ZoomSystem,
}

// Serde modules for custom serialization
mod serializable_vec2_option {
    use super::*;
    
    pub fn serialize<S>(value: &Option<Vec2>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match value {
            Some(v) => SerializableVec2::from(*v).serialize(serializer),
            None => serializer.serialize_none(),
        }
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec2>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let opt: Option<SerializableVec2> = Option::deserialize(deserializer)?;
        Ok(opt.map(|v| v.into()))
    }
}

mod serializable_rect {
    use super::*;
    
    pub fn serialize<S>(value: &Rect, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializableRect::from(*value).serialize(serializer)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Rect, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let r: SerializableRect = SerializableRect::deserialize(deserializer)?;
        Ok(r.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomSystem {
    pub current_level: i32,
    pub target_level: i32,
    pub interpolation_progress: f32,
    pub interpolation_speed: f32,
    pub home_position: SerializableVec2,
    pub base_scale: f64,
    pub zoom_factor: f64,
    pub max_level: i32,
}

impl ZoomSystem {
    pub fn new() -> Self {
        Self {
            current_level: 8,
            target_level: 8,
            interpolation_progress: 1.0,
            interpolation_speed: 2.0,
            home_position: SerializableVec2 { x: 400.0, y: 300.0 },
            base_scale: 1.0,
            zoom_factor: 3.55,
            max_level: 50,
        }
    }
    
    pub fn get_zoom_label(&self) -> String {
        format!("Zoom Level: {}", self.current_level)
    }
    
    pub fn get_zoom_description(&self) -> String {
        match self.current_level {
            1..=9 => "Tactical View".to_string(),
            10..=18 => "Strategic View".to_string(),
            19..=30 => "System View".to_string(),
            31..=42 => "Galactic View".to_string(),
            43..=50 => "Cosmic View".to_string(),
            _ => "Unknown Scale".to_string(),
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            units: Vec::new(),
            players: vec![Player::new(0), Player::new(1)],
            resource_nodes: vec![
                ResourceNode {
                    id: 0,
                    x: 200.0,
                    y: 150.0,
                    resource_type: crate::entity::resource_node::ResourceType::Minerals,
                    amount: 1000,
                    max_amount: 1000,
                    resources: 1000,
                    radius: 30.0,
                },
                ResourceNode {
                    id: 1,
                    x: 600.0,
                    y: 450.0,
                    resource_type: crate::entity::resource_node::ResourceType::Energy,
                    amount: 800,
                    max_amount: 800,
                    resources: 800,
                    radius: 25.0,
                },
            ],
            selected_units: Vec::new(),
            current_screen: GameScreen::MainMenu,
            current_player: 0,
            camera_x: 0.0,
            camera_y: 0.0,
            is_running: true,
            selection_start: None,
            selection_end: None,
            minimap_rect: Rect::new(10.0, 10.0, 200.0, 150.0),
            paused: false,
            game_time: 0.0,
            last_update_time: 0.0,
            sound_muted: false,
            music_muted: false,
            sound_volume: 1.0,
            music_volume: 0.7,
            zoom_system: ZoomSystem::new(),
        };
        
        // Initialize with headquarters
        state.spawn_unit(UnitType::Headquarters, 400.0, 300.0, 0);
        state.spawn_unit(UnitType::Headquarters, 800.0, 500.0, 1);
        
        state
    }

    pub fn update(&mut self) {
        let dt = 0.016; // Assume 60 FPS
        if self.paused {
            return;
        }
        
        self.game_time += dt;
        self.last_update_time = dt;
        
        // Update all units
        for unit in &mut self.units {
            unit.update(dt);
        }

        // Process command queue
        self.process_commands();

        // Update camera based on selected units
        self.update_camera();
    }

    pub fn render(&self, _resource_manager: &crate::resources::ResourceManager) {
        // Clear background
        clear_background(Color::new(0.1, 0.2, 0.3, 1.0));

        // Render terrain/background
        self.render_background();

        // Render resource nodes
        for node in &self.resource_nodes {
            let screen_x = node.x - self.camera_x;
            let screen_y = node.y - self.camera_y;
            
            let color = match node.resource_type {
                crate::entity::resource_node::ResourceType::Minerals => BLUE,
                crate::entity::resource_node::ResourceType::Energy => YELLOW,
            };
            
            draw_circle(screen_x, screen_y, node.radius, color);
            
            // Draw resource amount
            let text = format!("{}", node.resources);
            draw_text(&text, screen_x - 20.0, screen_y - 40.0, 16.0, WHITE);
        }

        // Render units using the rendering module
        crate::game::rendering::render_game(self, self.camera_x, self.camera_y, &crate::resources::manager::ResourceManager::new());
    }

    pub fn request_quit(&mut self) {
        self.is_running = false;
    }

    pub fn spawn_unit(&mut self, unit_type: UnitType, x: f32, y: f32, player_id: usize) -> u32 {
        let id = self.next_unit_id;
        self.next_unit_id += 1;
        
        let unit = Unit::new(id, unit_type, x, y, player_id);
        self.units.push(unit);
        
        id
    }

    pub fn can_afford(&self, player_id: usize, unit_type: &UnitType) -> bool {
        let cost = self.get_unit_cost(unit_type);
        self.players[player_id].minerals >= cost
    }

    pub fn deduct_cost(&mut self, player_id: usize, unit_type: &UnitType) {
        let cost = self.get_unit_cost(unit_type);
        self.players[player_id].minerals -= cost;
    }

    pub fn get_unit_cost(&self, unit_type: &UnitType) -> u32 {
        match unit_type {
            UnitType::Worker => 50,
            UnitType::Fighter => 100,
            UnitType::Ranger => 75,
            UnitType::Tank => 150,
            UnitType::Building => 200,
            UnitType::Headquarters => 500,
        }
    }

    pub fn process_commands(&mut self) {
        // Process any pending commands
        // This is a placeholder for command processing logic
    }

    pub fn update_camera(&mut self) {
        // Camera update logic
        // Handle WASD movement, zoom, etc.
    }

    pub fn render_background(&self) {
        clear_background(Color::new(0.1, 0.2, 0.1, 1.0));
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub minerals: u32,
    pub energy: u32,
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            name: format!("Player {}", id + 1),
            minerals: 500,
            energy: 500,
        }
    }
}
