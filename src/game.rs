use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use crate::resources::ResourceManager;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum GameMode {
    Offline,
    Online,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum GameScreen {
    MainMenu,
    Playing,
    Settings,
    Credits,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    ChatMessage(String),
    GameState(Vec<Unit>),
    PlayerAction { unit_id: u32, target_x: f32, target_y: f32 },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UnitType {
    Worker,
    Fighter,
    Ranger,
    Tank,
    Building,
    Headquarters,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BuildingType {
    Headquarters,
    Barracks,
    Factory,
    ResearchCenter,
    TurretDefense,
    ResourceCollector,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub health: i32,
    pub max_health: i32,
    pub unit_type: UnitType,
    pub player_id: u8,
    pub target_x: Option<f32>,
    pub target_y: Option<f32>,
    pub speed: f32,
    pub attack_damage: i32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub current_cooldown: f32,
    pub building_type: Option<BuildingType>,
    pub construction_progress: Option<f32>,
    pub resource_capacity: Option<i32>,
    pub current_resources: Option<i32>,
}

impl Unit {
    pub fn new(id: u32, x: f32, y: f32, unit_type: UnitType, player_id: u8) -> Self {
        let (health, speed, damage, range) = match unit_type {
            UnitType::Worker => (50, 2.5, 5, 20.0),
            UnitType::Fighter => (80, 3.0, 15, 30.0),
            UnitType::Ranger => (60, 2.8, 20, 150.0),
            UnitType::Tank => (150, 1.5, 30, 60.0),
            UnitType::Building => (200, 0.0, 0, 0.0),
            UnitType::Headquarters => (500, 0.0, 0, 0.0),
        };

        let resource_capacity = if unit_type == UnitType::Worker {
            Some(50)
        } else {
            None
        };

        let building_type = if unit_type == UnitType::Building || unit_type == UnitType::Headquarters {
            Some(BuildingType::Headquarters) 
        } else {
            None
        };

        Self {
            id,
            x,
            y,
            health,
            max_health: health,
            unit_type,
            player_id,
            target_x: None,
            target_y: None,
            speed,
            attack_damage: damage,
            attack_range: range,
            attack_cooldown: 1.0,
            current_cooldown: 0.0,
            building_type,
            construction_progress: None,
            resource_capacity,
            current_resources: if resource_capacity.is_some() { Some(0) } else { None },
        }
    }

    #[allow(dead_code)]
    pub fn is_completed_building(&self) -> bool {
        if let (Some(_building_type), Some(progress)) = (&self.building_type, self.construction_progress) {
            progress >= 100.0
        } else {
            false
        }
    }
}

pub struct ResourceNode {
    pub x: f32,
    pub y: f32,
    pub resources: i32,
    pub resource_type: ResourceType,
    pub radius: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResourceType {
    Minerals,
    Energy,
}

pub struct Player {
    #[allow(dead_code)]
    pub id: u8,
    pub minerals: i32,
    pub energy: i32,
    pub color: Color,
    #[allow(dead_code)]
    pub is_ai: bool,
}

pub struct GameState {
    pub units: Vec<Unit>,
    pub selected_units: Vec<u32>,
    pub messages: Vec<String>,
    pub game_mode: GameMode,
    pub current_screen: GameScreen,
    pub world_address: String,
    pub camera_x: f32,
    pub camera_y: f32,
    pub map_width: f32,
    pub map_height: f32,
    pub players: Vec<Player>,
    pub current_player_id: u8,
    pub resource_nodes: Vec<ResourceNode>,
    pub next_unit_id: u32,
    pub game_time: f32,
    pub minimap_rect: Rect,
    #[allow(dead_code)]
    pub current_command: Option<Command>,
    pub selection_start: Option<(f32, f32)>,
    pub selection_end: Option<(f32, f32)>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Build(BuildingType),
    Train(UnitType),
    Attack,
    Move,
    Gather,
}

impl GameState {
    pub fn new() -> Self {
        // Create initial players
        let players = vec![
            Player { id: 0, minerals: 500, energy: 200, color: BLUE, is_ai: false },
            Player { id: 1, minerals: 500, energy: 200, color: RED, is_ai: true },
        ];
        
        // Create starting units
        let mut units = vec![
            // Player headquarters
            Unit::new(1, 100.0, 100.0, UnitType::Headquarters, 0),
            // AI headquarters
            Unit::new(2, 900.0, 700.0, UnitType::Headquarters, 1),
        ];
        
        // Add starting workers
        units.push(Unit::new(3, 150.0, 100.0, UnitType::Worker, 0));
        units.push(Unit::new(4, 170.0, 120.0, UnitType::Worker, 0));
        units.push(Unit::new(5, 900.0, 730.0, UnitType::Worker, 1));
        units.push(Unit::new(6, 870.0, 700.0, UnitType::Worker, 1));
        
        // Create resource nodes
        let mut resource_nodes = vec![];
        
        // Add mineral nodes
        for i in 0..5 {
            resource_nodes.push(ResourceNode {
                x: 300.0 + i as f32 * 60.0,
                y: 300.0,
                resources: 1000,
                resource_type: ResourceType::Minerals,
                radius: 25.0,
            });
            
            resource_nodes.push(ResourceNode {
                x: 700.0 - i as f32 * 60.0,
                y: 500.0,
                resources: 1000,
                resource_type: ResourceType::Minerals,
                radius: 25.0,
            });
        }
        
        // Add energy nodes
        for i in 0..3 {
            resource_nodes.push(ResourceNode {
                x: 500.0,
                y: 200.0 + i as f32 * 70.0,
                resources: 1000,
                resource_type: ResourceType::Energy,
                radius: 20.0,
            });
        }

        Self {
            units,
            selected_units: Vec::new(),
            messages: Vec::new(),
            game_mode: GameMode::Offline,
            current_screen: GameScreen::MainMenu,
            world_address: String::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            map_width: 1000.0,
            map_height: 800.0,
            players,
            current_player_id: 0,
            resource_nodes,
            next_unit_id: 7,
            game_time: 0.0,
            minimap_rect: Rect::new(screen_width() - 210.0, screen_height() - 210.0, 200.0, 200.0),
            current_command: None,
            selection_start: None,
            selection_end: None,
        }
    }
    
    pub fn update(&mut self) {
        // Update game time
        self.game_time += get_frame_time();
        
        // Handle camera movement
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.camera_y -= 5.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.camera_y += 5.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.camera_x -= 5.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.camera_x += 5.0;
        }
        
        // Clamp camera position
        self.camera_x = self.camera_x.clamp(0.0, self.map_width - screen_width());
        self.camera_y = self.camera_y.clamp(0.0, self.map_height - screen_height());
        
        // Process unit movement and actions
        self.update_units();
        
        // Handle box selection
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            // If clicking on the minimap, move the camera instead
            if self.minimap_rect.contains(Vec2::new(mouse_x, mouse_y)) {
                // Convert minimap coordinates to world coordinates
                let map_ratio_x = self.map_width / self.minimap_rect.w;
                let map_ratio_y = self.map_height / self.minimap_rect.h;
                
                let world_x = (mouse_x - self.minimap_rect.x) * map_ratio_x;
                let world_y = (mouse_y - self.minimap_rect.y) * map_ratio_y;
                
                self.camera_x = world_x - screen_width() / 2.0;
                self.camera_y = world_y - screen_height() / 2.0;
                
                // Clamp camera position
                self.camera_x = self.camera_x.clamp(0.0, self.map_width - screen_width());
                self.camera_y = self.camera_y.clamp(0.0, self.map_height - screen_height());
            } else {
                self.selection_start = Some((mouse_x + self.camera_x, mouse_y + self.camera_y));
                self.selection_end = Some((mouse_x + self.camera_x, mouse_y + self.camera_y));
            }
        }
        
        if is_mouse_button_down(MouseButton::Left) && self.selection_start.is_some() {
            let (mouse_x, mouse_y) = mouse_position();
            self.selection_end = Some((mouse_x + self.camera_x, mouse_y + self.camera_y));
        }
        
        if is_mouse_button_released(MouseButton::Left) && self.selection_start.is_some() {
            let (mouse_x, mouse_y) = mouse_position();
            let end = (mouse_x + self.camera_x, mouse_y + self.camera_y);
            self.selection_end = Some(end);
            
            if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
                // Clear previous selection if not holding shift
                if !is_key_down(KeyCode::LeftShift) {
                    self.selected_units.clear();
                }
                
                // Check if it's just a click (with small tolerance for movement)
                let is_click = ((end.0 - start.0).powi(2) + (end.1 - start.1).powi(2)).sqrt() < 5.0;
                
                if is_click {
                    // Single click selection
                    self.select_unit_at(start.0, start.1);
                } else {
                    // Box selection
                    let selection_rect = Rect::new(
                        start.0.min(end.0),
                        start.1.min(end.1),
                        (start.0 - end.0).abs(),
                        (start.1 - end.1).abs()
                    );
                    
                    // Select all player's units in the box
                    for unit in &self.units {
                        if unit.player_id == self.current_player_id {
                            let unit_pos = Vec2::new(unit.x, unit.y);
                            if selection_rect.contains(unit_pos) {
                                if !self.selected_units.contains(&unit.id) {
                                    self.selected_units.push(unit.id);
                                }
                            }
                        }
                    }
                }
            }
            
            self.selection_start = None;
            self.selection_end = None;
        }
        
        // Right-click actions
        if is_mouse_button_pressed(MouseButton::Right) && !self.selected_units.is_empty() {
            let (mouse_x, mouse_y) = mouse_position();
            let target_x = mouse_x + self.camera_x;
            let target_y = mouse_y + self.camera_y;
            
            // Check if clicked on an enemy for attack order
            let mut target_enemy = None;
            for unit in &self.units {
                if unit.player_id != self.current_player_id {
                    let distance = ((unit.x - target_x).powi(2) + (unit.y - target_y).powi(2)).sqrt();
                    if distance < 20.0 {
                        target_enemy = Some(unit.id);
                        break;
                    }
                }
            }
            
            // Check if clicked on a resource node for gather order
            let mut target_resource = None;
            for (i, node) in self.resource_nodes.iter().enumerate() {
                let distance = ((node.x - target_x).powi(2) + (node.y - target_y).powi(2)).sqrt();
                if distance < node.radius {
                    target_resource = Some(i);
                    break;
                }
            }
            
            // Process orders based on what was clicked
            if let Some(enemy_id) = target_enemy {
                // Attack order
                for &unit_id in &self.selected_units {
                    for unit_idx in 0..self.units.len() {
                        if self.units[unit_idx].id == unit_id {
                            // Find enemy position first
                            let enemy_pos = if let Some(enemy) = self.units.iter().find(|u| u.id == enemy_id) {
                                (enemy.x, enemy.y)
                            } else {
                                continue;  // Skip if enemy not found
                            };
                            
                            // Then update the unit with the position
                            self.units[unit_idx].target_x = Some(enemy_pos.0);
                            self.units[unit_idx].target_y = Some(enemy_pos.1);
                            break;
                        }
                    }
                }
            } else if let Some(resource_id) = target_resource {
                // Gather order (only for workers)
                for &unit_id in &self.selected_units {
                    for unit in &mut self.units {
                        if unit.id == unit_id && unit.unit_type == UnitType::Worker {
                            let node = &self.resource_nodes[resource_id];
                            unit.target_x = Some(node.x);
                            unit.target_y = Some(node.y);
                            break;
                        }
                    }
                }
            } else {
                // Move order
                for &unit_id in &self.selected_units {
                    if let Some(unit) = self.units.iter_mut().find(|u| u.id == unit_id) {
                        if unit.unit_type != UnitType::Building && unit.unit_type != UnitType::Headquarters {
                            unit.target_x = Some(target_x);
                            unit.target_y = Some(target_y);
                        }
                    }
                }
            }
        }
    }
    
    pub fn update_units(&mut self) {
        // Copy player indices of units that need to gather resources
        let mut units_to_update = Vec::new();
        for i in 0..self.units.len() {
            units_to_update.push(i);
        }
        
        for unit_idx in units_to_update {
            if unit_idx >= self.units.len() {
                continue; // Unit may have been removed
            }
            
            // Skip buildings or dead units
            let unit_type = self.units[unit_idx].unit_type.clone();
            if unit_type == UnitType::Building || 
               unit_type == UnitType::Headquarters || 
               self.units[unit_idx].health <= 0 {
                continue;
            }
            
            // Update attack cooldown
            if self.units[unit_idx].current_cooldown > 0.0 {
                self.units[unit_idx].current_cooldown -= get_frame_time();
            }
            
            // Move unit toward target if it has one
            if let (Some(target_x), Some(target_y)) = (self.units[unit_idx].target_x, self.units[unit_idx].target_y) {
                let unit_x = self.units[unit_idx].x;
                let unit_y = self.units[unit_idx].y;
                let unit_speed = self.units[unit_idx].speed;
                let unit_player_id = self.units[unit_idx].player_id;
                
                let dx = target_x - unit_x;
                let dy = target_y - unit_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // If close enough to target, stop moving
                if distance < 5.0 {
                    self.units[unit_idx].target_x = None;
                    self.units[unit_idx].target_y = None;
                } else {
                    // Move toward target
                    self.units[unit_idx].x += dx / distance * unit_speed;
                    self.units[unit_idx].y += dy / distance * unit_speed;
                }
                
                // Check for resource gathering
                if unit_type == UnitType::Worker {
                    // Try to gather resources if near a resource node
                    for node_idx in 0..self.resource_nodes.len() {
                        let node = &mut self.resource_nodes[node_idx];
                        let distance_to_node = ((node.x - unit_x).powi(2) + (node.y - unit_y).powi(2)).sqrt();
                        
                        if distance_to_node < node.radius + 10.0 {
                            if let Some(current_resources) = self.units[unit_idx].current_resources {
                                if let Some(capacity) = self.units[unit_idx].resource_capacity {
                                    if current_resources < capacity && node.resources > 0 {
                                        // Gather resources
                                        let amount_to_gather = (capacity - current_resources).min(5).min(node.resources);
                                        self.units[unit_idx].current_resources = Some(current_resources + amount_to_gather);
                                        node.resources -= amount_to_gather;
                                        
                                        // Once full, return to HQ
                                        if current_resources + amount_to_gather >= capacity {
                                            // Find player's headquarters
                                            let mut hq_pos = None;
                                            for other_unit in &self.units {
                                                if other_unit.unit_type == UnitType::Headquarters && 
                                                   other_unit.player_id == unit_player_id {
                                                    hq_pos = Some((other_unit.x, other_unit.y));
                                                    break;
                                                }
                                            }
                                            
                                            if let Some((hq_x, hq_y)) = hq_pos {
                                                self.units[unit_idx].target_x = Some(hq_x);
                                                self.units[unit_idx].target_y = Some(hq_y);
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    
                    // Check if at headquarters to deposit resources
                    if self.units[unit_idx].current_resources.unwrap_or(0) > 0 {
                        // Find headquarters positions first
                        let mut hq_positions = Vec::new();
                        for hq in &self.units {
                            if hq.unit_type == UnitType::Headquarters && hq.player_id == unit_player_id {
                                hq_positions.push((hq.x, hq.y));
                            }
                        }
                        
                        // Then check distances and deposit resources if close to HQ
                        for (hq_x, hq_y) in hq_positions {
                            let distance_to_hq = ((hq_x - unit_x).powi(2) + (hq_y - unit_y).powi(2)).sqrt();
                            
                            if distance_to_hq < 30.0 {
                                // Deposit resources
                                let resources_to_deposit = self.units[unit_idx].current_resources.unwrap_or(0);
                                self.units[unit_idx].current_resources = Some(0);
                                
                                // Update player resources
                                if let Some(player) = self.players.get_mut(unit_player_id as usize) {
                                    player.minerals += resources_to_deposit;
                                }
                                break;
                            }
                        }
                    }
                }
                
                // Check for combat with nearby enemies
                let mut nearest_enemy_dist = f32::MAX;
                let mut nearest_enemy_idx = None;
                
                let attack_range = self.units[unit_idx].attack_range;
                
                // Find nearest enemy
                for (i, other_unit) in self.units.iter().enumerate() {
                    if other_unit.player_id != unit_player_id && other_unit.health > 0 {
                        let dist = ((other_unit.x - unit_x).powi(2) + (other_unit.y - unit_y).powi(2)).sqrt();
                        if dist < attack_range && dist < nearest_enemy_dist {
                            nearest_enemy_dist = dist;
                            nearest_enemy_idx = Some(i);
                        }
                    }
                }
                
                // Attack the enemy if cooldown is ready
                if let Some(enemy_idx) = nearest_enemy_idx {
                    if self.units[unit_idx].current_cooldown <= 0.0 {
                        let damage = self.units[unit_idx].attack_damage;
                        self.units[enemy_idx].health -= damage;
                        self.units[unit_idx].current_cooldown = self.units[unit_idx].attack_cooldown;
                        
                        // No need to handle enemy death here as we'll clean up in a separate pass below
                    }
                }
            }
        }
        
        // Remove dead units
        self.units.retain(|unit| unit.health > 0);
    }
    
    pub fn draw(&self, _resource_manager: &ResourceManager) {
        // Draw terrain background
        draw_rectangle(0.0, 0.0, self.map_width, self.map_height, Color::new(0.1, 0.4, 0.1, 1.0));
        
        // Draw grid lines
        let grid_size = 50.0;
        let start_x = (self.camera_x / grid_size).floor() * grid_size;
        let start_y = (self.camera_y / grid_size).floor() * grid_size;
        
        for x in (start_x as i32..=(self.camera_x + screen_width()) as i32).step_by(grid_size as usize) {
            draw_line(
                x as f32 - self.camera_x, 
                0.0, 
                x as f32 - self.camera_x, 
                screen_height(),
                1.0, 
                Color::new(0.2, 0.5, 0.2, 0.5)
            );
        }
        
        for y in (start_y as i32..=(self.camera_y + screen_height()) as i32).step_by(grid_size as usize) {
            draw_line(
                0.0, 
                y as f32 - self.camera_y, 
                screen_width(),
                y as f32 - self.camera_y, 
                1.0, 
                Color::new(0.2, 0.5, 0.2, 0.5)
            );
        }
        
        // Draw resource nodes
        for node in &self.resource_nodes {
            let color = match node.resource_type {
                ResourceType::Minerals => Color::new(0.1, 0.1, 0.8, 1.0),
                ResourceType::Energy => Color::new(0.9, 0.9, 0.1, 1.0),
            };
            
            draw_circle(
                node.x - self.camera_x, 
                node.y - self.camera_y, 
                node.radius, 
                color
            );
            
            // Draw resource amount
            let text_size = 12.0;
            let text = node.resources.to_string();
            let text_size = measure_text(&text, None, text_size as u16, 1.0);
            
            draw_text(
                &text,
                node.x - self.camera_x - text_size.width / 2.0,
                node.y - self.camera_y + text_size.height / 2.0,
                text_size.height,
                WHITE
            );
        }
        
        // Draw all game units
        for unit in &self.units {
            let base_color = self.players[unit.player_id as usize].color;
            let is_selected = self.selected_units.contains(&unit.id);
            let border_size = if is_selected { 2.0 } else { 0.0 };
            
            match unit.unit_type {
                UnitType::Worker => {
                    draw_circle_lines(
                        unit.x - self.camera_x, 
                        unit.y - self.camera_y, 
                        12.0, 
                        border_size, 
                        GREEN
                    );
                    draw_circle(
                        unit.x - self.camera_x, 
                        unit.y - self.camera_y, 
                        10.0, 
                        base_color
                    );
                    
                    // Draw resources being carried
                    if let Some(resources) = unit.current_resources {
                        if resources > 0 {
                            draw_circle(
                                unit.x - self.camera_x + 5.0, 
                                unit.y - self.camera_y - 5.0, 
                                5.0, 
                                BLUE
                            );
                        }
                    }
                },
                UnitType::Fighter => {
                    draw_circle_lines(
                        unit.x - self.camera_x, 
                        unit.y - self.camera_y, 
                        12.0, 
                        border_size, 
                        GREEN
                    );
                    draw_rectangle(
                        unit.x - self.camera_x - 8.0, 
                        unit.y - self.camera_y - 8.0, 
                        16.0, 
                        16.0, 
                        base_color
                    );
                },
                UnitType::Ranger => {
                    draw_circle_lines(
                        unit.x - self.camera_x, 
                        unit.y - self.camera_y, 
                        12.0, 
                        border_size, 
                        GREEN
                    );
                    draw_triangle(
                        Vec2::new(unit.x - self.camera_x, unit.y - self.camera_y - 10.0),
                        Vec2::new(unit.x - self.camera_x - 8.0, unit.y - self.camera_y + 5.0),
                        Vec2::new(unit.x - self.camera_x + 8.0, unit.y - self.camera_y + 5.0),
                        base_color
                    );
                },
                UnitType::Tank => {
                    draw_circle_lines(
                        unit.x - self.camera_x, 
                        unit.y - self.camera_y, 
                        15.0, 
                        border_size, 
                        GREEN
                    );
                    draw_rectangle(
                        unit.x - self.camera_x - 12.0, 
                        unit.y - self.camera_y - 8.0, 
                        24.0, 
                        16.0, 
                        base_color
                    );
                    
                    // Draw tank turret
                    draw_circle(
                        unit.x - self.camera_x, 
                        unit.y - self.camera_y, 
                        6.0, 
                        Color::new(
                            base_color.r * 0.8,
                            base_color.g * 0.8,
                            base_color.b * 0.8,
                            1.0
                        )
                    );
                },
                UnitType::Building => {
                    let building_size = 30.0;
                    
                    draw_rectangle_lines(
                        unit.x - self.camera_x - building_size,
                        unit.y - self.camera_y - building_size, 
                        building_size * 2.0, 
                        building_size * 2.0, 
                        border_size,
                        GREEN
                    );
                    
                    draw_rectangle(
                        unit.x - self.camera_x - building_size, 
                        unit.y - self.camera_y - building_size, 
                        building_size * 2.0, 
                        building_size * 2.0, 
                        base_color
                    );
                    
                    // Draw construction progress if applicable
                    if let Some(progress) = unit.construction_progress {
                        if progress < 100.0 {
                            draw_rectangle(
                                unit.x - self.camera_x - building_size,
                                unit.y - self.camera_y - building_size - 10.0,
                                (building_size * 2.0) * (progress / 100.0),
                                5.0,
                                GREEN
                            );
                        }
                    }
                },
                UnitType::Headquarters => {
                    let hq_size = 40.0;
                    
                    draw_rectangle_lines(
                        unit.x - self.camera_x - hq_size,
                        unit.y - self.camera_y - hq_size, 
                        hq_size * 2.0, 
                        hq_size * 2.0, 
                        border_size,
                        GREEN
                    );
                    
                    draw_rectangle(
                        unit.x - self.camera_x - hq_size, 
                        unit.y - self.camera_y - hq_size, 
                        hq_size * 2.0, 
                        hq_size * 2.0, 
                        base_color
                    );
                    
                    // Draw an 'H' in the center
                    let h_text = "HQ";
                    let h_text_size = measure_text(h_text, None, 24, 1.0);
                    draw_text(
                        h_text,
                        unit.x - self.camera_x - h_text_size.width / 2.0,
                        unit.y - self.camera_y + h_text_size.height / 2.0,
                        24.0,
                        WHITE
                    );
                },
            }
            
            // Draw health bar
            let health_width = 20.0;
            let health_height = 3.0;
            let health_x = unit.x - self.camera_x - health_width / 2.0;
            let health_y = unit.y - self.camera_y - 15.0;
            
            // Health bar background
            draw_rectangle(
                health_x,
                health_y,
                health_width,
                health_height,
                Color::new(0.3, 0.3, 0.3, 0.8)
            );
            
            // Health bar fill
            let health_ratio = unit.health as f32 / unit.max_health as f32;
            draw_rectangle(
                health_x,
                health_y,
                health_width * health_ratio,
                health_height,
                Color::new(0.1, 0.9, 0.1, 0.8)
            );
        }
        
        // Draw selection box if dragging
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let x = (start.0 - self.camera_x).min(end.0 - self.camera_x);
            let y = (start.1 - self.camera_y).min(end.1 - self.camera_y);
            let width = (start.0 - end.0).abs();
            let height = (start.1 - end.1).abs();
            
            draw_rectangle_lines(x, y, width, height, 1.0, GREEN);
        }
        
        // Draw minimap
        draw_rectangle(
            self.minimap_rect.x,
            self.minimap_rect.y,
            self.minimap_rect.w,
            self.minimap_rect.h,
            Color::new(0.0, 0.0, 0.0, 0.7)
        );
        
        draw_rectangle_lines(
            self.minimap_rect.x,
            self.minimap_rect.y,
            self.minimap_rect.w,
            self.minimap_rect.h,
            1.0,
            Color::new(0.8, 0.8, 0.8, 0.5)
        );
        
        // Draw units on minimap
        let map_ratio_x = self.minimap_rect.w / self.map_width;
        let map_ratio_y = self.minimap_rect.h / self.map_height;
        
        for unit in &self.units {
            let minimap_x = self.minimap_rect.x + unit.x * map_ratio_x;
            let minimap_y = self.minimap_rect.y + unit.y * map_ratio_y;
            let minimap_size = 2.0;
            
            draw_rectangle(
                minimap_x - minimap_size / 2.0,
                minimap_y - minimap_size / 2.0,
                minimap_size,
                minimap_size, // Add missing height parameter
                self.players[unit.player_id as usize].color
            );
        }
        
        // Draw camera viewport on minimap
        let viewport_x = self.minimap_rect.x + self.camera_x * map_ratio_x;
        let viewport_y = self.minimap_rect.y + self.camera_y * map_ratio_y;
        let viewport_w = screen_width() * map_ratio_x;
        let viewport_h = screen_height() * map_ratio_y;
        
        draw_rectangle_lines(
            viewport_x,
            viewport_y,
            viewport_w,
            viewport_h,
            1.0,
            WHITE
        );
        
        // Draw resources display
        let player = &self.players[self.current_player_id as usize];
        draw_text(
            &format!("Minerals: {}", player.minerals),
            10.0,
            30.0,
            20.0,
            WHITE
        );
        
        draw_text(
            &format!("Energy: {}", player.energy),
            10.0,
            55.0,
            20.0,
            Color::new(1.0, 1.0, 0.0, 1.0)
        );
        
        // Draw selected unit commands
        if !self.selected_units.is_empty() {
            let mut y_pos = 100.0;
            
            // Show different commands based on selected unit types
            let mut has_workers = false;
            let mut has_buildings = false;
            let mut has_combat_units = false;
            
            for &unit_id in &self.selected_units {
                if let Some(unit) = self.units.iter().find(|u| u.id == unit_id) {
                    match unit.unit_type {
                        UnitType::Worker => has_workers = true,
                        UnitType::Building | UnitType::Headquarters => has_buildings = true,
                        _ => has_combat_units = true,
                    }
                }
            }
            
            // Display command info based on current selection
            if has_workers || has_combat_units || has_buildings {
                draw_text(
                    "Commands:",
                    10.0,
                    y_pos,
                    18.0,
                    WHITE
                );
                y_pos += 25.0;
                
                if has_workers {
                    draw_text(
                        "G - Gather resources",
                        15.0,
                        y_pos,
                        16.0,
                        WHITE
                    );
                    y_pos += 20.0;
                    
                    draw_text(
                        "B - Build structure",
                        15.0,
                        y_pos,
                        16.0,
                        WHITE
                    );
                    y_pos += 20.0;
                }
                
                if has_combat_units || has_workers {
                    draw_text(
                        "A - Attack",
                        15.0,
                        y_pos,
                        16.0,
                        WHITE
                    );
                    y_pos += 20.0;
                    
                    draw_text(
                        "M - Move",
                        15.0,
                        y_pos,
                        16.0,
                        WHITE
                    );
                    if has_buildings && y_pos > 0.0 {
                        y_pos += 20.0;
                    }
                }
                
                if has_buildings {
                    draw_text(
                        "T - Train units",
                        15.0,
                        y_pos,
                        16.0,
                        WHITE
                    );
                }
            }
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
        // Clear previous selection if not holding shift
        if !is_key_down(KeyCode::LeftShift) {
            self.selected_units.clear();
        }
        
        // Find unit at position
        for unit in &self.units {
            if unit.player_id == self.current_player_id {
                let distance = ((unit.x - x).powi(2) + (unit.y - y).powi(2)).sqrt();
                let selection_radius = match unit.unit_type {
                    UnitType::Building | UnitType::Headquarters => 40.0,
                    _ => 15.0
                };
                
                if distance < selection_radius {
                    if !self.selected_units.contains(&unit.id) {
                        self.selected_units.push(unit.id);
                    }
                    break;
                }
            }
        }
    }
    
    pub fn move_selected_unit(&mut self, x: f32, y: f32) {
        if self.selected_units.is_empty() {
            return;
        }

        for &unit_id in &self.selected_units {
            if let Some(unit) = self.units.iter_mut().find(|u| u.id == unit_id) {
                if unit.unit_type != UnitType::Building && unit.unit_type != UnitType::Headquarters {
                    unit.target_x = Some(x);
                    unit.target_y = Some(y);
                }
            }
        }
    }

    pub fn set_game_mode(&mut self, mode: GameMode) {
        self.game_mode = mode;
        
        if mode == GameMode::Offline {
            self.messages.push("Playing in offline mode.".to_string());
        }
    }

    pub fn spawn_unit(&mut self, unit_type: UnitType, x: f32, y: f32, player_id: u8) -> u32 {
        let id = self.next_unit_id;
        self.next_unit_id += 1;

        self.units.push(Unit::new(id, x, y, unit_type, player_id));
        id
    }

    pub fn can_afford(&self, player_id: u8, unit_type: &UnitType) -> bool {
        let player = &self.players[player_id as usize];
        
        match unit_type {
            UnitType::Worker => player.minerals >= 50,
            UnitType::Fighter => player.minerals >= 100 && player.energy >= 20,
            UnitType::Ranger => player.minerals >= 80 && player.energy >= 40,
            UnitType::Tank => player.minerals >= 200 && player.energy >= 50,
            UnitType::Building => player.minerals >= 150,
            UnitType::Headquarters => false, // Can't build headquarters
        }
    }

    pub fn deduct_cost(&mut self, player_id: u8, unit_type: &UnitType) {
        let player = &mut self.players[player_id as usize];
        
        match unit_type {
            UnitType::Worker => player.minerals -= 50,
            UnitType::Fighter => {
                player.minerals -= 100;
                player.energy -= 20;
            },
            UnitType::Ranger => {
                player.minerals -= 80;
                player.energy -= 40;
            },
            UnitType::Tank => {
                player.minerals -= 200;
                player.energy -= 50;
            },
            UnitType::Building => player.minerals -= 150,
            UnitType::Headquarters => {}, // Free (can't be built)
        }
    }
}
