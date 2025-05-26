use macroquad::prelude::*;
use crate::entity::{Unit, Player, UnitType};
use crate::game::modes::GameMode;
use crate::game::screens::GameScreen;
use crate::game::types::ResourceType;
use crate::game::commands::Command;
use crate::game::resources::ResourceNode;
use crate::network::NetworkMessage; // Fixed import

pub struct GameState {
    pub units: Vec<Unit>,
    pub selected_units: Vec<u32>,
    pub messages: Vec<String>,
    pub game_mode: GameMode,
    pub current_screen: GameScreen,
    #[allow(dead_code)]
    pub world_address: String,
    pub camera_x: f32,
    pub camera_y: f32,
    pub map_width: f32,
    pub map_height: f32,
    pub players: Vec<Player>,
    pub current_player_id: usize, // Change from u8 to usize
    pub resource_nodes: Vec<ResourceNode>,
    pub next_unit_id: u32,
    pub game_time: f32,
    pub minimap_rect: Rect,
    #[allow(dead_code)]
    pub current_command: Option<Command>,
    pub selection_start: Option<(f32, f32)>,
    pub selection_end: Option<(f32, f32)>,
    // Add new game settings fields
    pub sound_volume: f32,
    pub music_volume: f32,
    #[allow(dead_code)] // Keep for future difficulty implementation
    pub game_difficulty: usize, // 0 = Easy, 1 = Normal, 2 = Hard
    pub sound_muted: bool,
    pub music_muted: bool,
    pub should_quit: bool,
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
            Unit::new(1, UnitType::Headquarters, 100.0, 100.0, 0),
            // AI player headquarters
            Unit::new(2, UnitType::Headquarters, 1500.0, 1200.0, 1),
        ];
        
        // Add some initial workers
        units.push(Unit::new(3, UnitType::Worker, 150.0, 100.0, 0));
        units.push(Unit::new(4, UnitType::Worker, 170.0, 120.0, 0));
        units.push(Unit::new(5, UnitType::Worker, 1500.0, 1230.0, 1));
        units.push(Unit::new(6, UnitType::Worker, 1470.0, 1200.0, 1));
        
        // Create resource nodes - more scattered for infinite feeling
        let mut resource_nodes = vec![];
        
        // Generate resources in clusters across a larger area
        for cluster in 0..10 {
            let cluster_x = cluster as f32 * 400.0 + rand::gen_range(0.0, 200.0);
            let cluster_y = cluster as f32 * 300.0 + rand::gen_range(0.0, 200.0);
            
            // Add mineral nodes in cluster
            for i in 0..3 {
                resource_nodes.push(ResourceNode {
                    x: cluster_x + i as f32 * 60.0 + rand::gen_range(-30.0, 30.0),
                    y: cluster_y + rand::gen_range(-50.0, 50.0),
                    resources: rand::gen_range(800, 1500),
                    resource_type: ResourceType::Minerals,
                    radius: 25.0,
                });
            }
            
            // Add energy nodes
            resource_nodes.push(ResourceNode {
                x: cluster_x + 100.0,
                y: cluster_y + 80.0,
                resources: rand::gen_range(600, 1200),
                resource_type: ResourceType::Energy,
                radius: 20.0,
            });
        }

        // Much larger map for infinite feeling
        let map_width = 5000.0;
        let map_height = 4000.0;

        // Create minimap rect
        let minimap_rect = Rect::new(
            screen_width() - 210.0, 
            screen_height() - 210.0, 
            200.0, 
            200.0
        );

        Self {
            units,
            selected_units: Vec::new(),
            messages: Vec::new(),
            game_mode: GameMode::Offline,
            current_screen: GameScreen::MainMenu,
            world_address: String::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            map_width,
            map_height,
            players,
            current_player_id: 0, // Change from u8 to usize
            resource_nodes,
            next_unit_id: 7,
            game_time: 0.0,
            minimap_rect,
            current_command: None,
            selection_start: None,
            selection_end: None,
            sound_volume: 0.6,
            music_volume: 0.4,
            game_difficulty: 1,
            sound_muted: false,
            music_muted: false,
            should_quit: false,
        }
    }
    
    pub fn update(&mut self) {
        // Update game time
        self.game_time += get_frame_time();
        
        // Dynamically generate more resources as we explore
        self.generate_resources_if_needed();
        
        // Enhanced camera movement with faster speed
        let camera_speed = 12.0;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.camera_y -= camera_speed;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.camera_y += camera_speed;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.camera_x -= camera_speed;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.camera_x += camera_speed;
        }
        
        // Keep camera within reasonable bounds but allow exploration
        self.camera_x = self.camera_x.clamp(-1000.0, self.map_width);
        self.camera_y = self.camera_y.clamp(-1000.0, self.map_height);
        
        // Process unit movement and actions
        self.update_units();
        
        // Enhanced unit AI behavior
        self.update_autonomous_behavior();
        
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
                
                // Safely clamp camera position
                self.ensure_camera_in_bounds();
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
               self.units[unit_idx].health <= 0.0 {
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
                                        let amount_to_gather = (capacity - current_resources).min(5).min(node.resources as u32);
                                        self.units[unit_idx].current_resources = Some(current_resources + amount_to_gather);
                                        node.resources -= amount_to_gather as i32;
                                        
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
                                    player.minerals += resources_to_deposit as i32;
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
                    if other_unit.player_id != unit_player_id && other_unit.health > 0.0 {
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
                    }
                }
            }
        }
        
        // Remove dead units
        let mut units_to_remove = Vec::new();
        for (unit_idx, unit) in self.units.iter().enumerate() {
            if unit.health <= 0.0 {
                units_to_remove.push(unit_idx);
            }
        }
        
        for unit_idx in units_to_remove.iter().rev() {
            self.units.remove(*unit_idx);
        }
    }
    
    pub fn draw(&self, resource_manager: &crate::resources::ResourceManager) {
        // Use the enhanced rendering system
        crate::game::rendering::draw_game(self, resource_manager);
    }
    
    pub fn handle_network_message(&mut self, msg: NetworkMessage) {
        match msg {
            NetworkMessage::ChatMessage { player_id: _, message } => {
                self.messages.push(message);
            },
            NetworkMessage::GameState { units, timestamp: _ } => {
                self.units = units;
            },
            NetworkMessage::UnitUpdate { unit_id, x, y } => {
                if let Some(unit) = self.units.iter_mut().find(|u| u.id == unit_id) {
                    unit.x = x;
                    unit.y = y;
                }
            },
            NetworkMessage::PlayerJoined { player_id: _, name } => {
                self.messages.push(format!("{} joined the game", name));
            },
            NetworkMessage::PlayerLeft { player_id: _ } => {
                self.messages.push("A player left the game".to_string());
            }
        }
    }
    
    pub fn select_unit_at(&mut self, x: f32, y: f32) {
        if !is_key_down(KeyCode::LeftShift) {
            self.selected_units.clear();
        }
        
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
    
    #[allow(dead_code)]
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

    #[allow(dead_code)] // Keep for future networking implementation
    pub fn set_game_mode(&mut self, mode: GameMode) {
        self.game_mode = mode;
    }

    pub fn spawn_unit(&mut self, unit_type: UnitType, x: f32, y: f32, player_id: usize) -> u32 {
        let id = self.next_unit_id;
        self.next_unit_id += 1;
        self.units.push(Unit::new(id, unit_type, x, y, player_id as u8));
        id
    }

    pub fn can_afford(&self, player_id: usize, unit_type: &UnitType) -> bool {
        let player = &self.players[player_id];
        
        match unit_type {
            UnitType::Worker => player.minerals >= 50,
            UnitType::Fighter => player.minerals >= 100 && player.energy >= 20,
            UnitType::Ranger => player.minerals >= 80 && player.energy >= 40,
            UnitType::Tank => player.minerals >= 200 && player.energy >= 50,
            UnitType::Building => player.minerals >= 150,
            UnitType::Headquarters => false,
        }
    }

    pub fn deduct_cost(&mut self, player_id: usize, unit_type: &UnitType) {
        let player = &mut self.players[player_id];
        
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
            UnitType::Headquarters => {},
        }
    }

    // Add a new method to handle screen resizes
    pub fn handle_screen_resize(&mut self) {
        // Update minimap position when screen size changes
        self.minimap_rect.x = screen_width() - 210.0;
        self.minimap_rect.y = screen_height() - 210.0;
        
        // Ensure camera stays within valid bounds after resize
        self.ensure_camera_in_bounds();
    }
    
    // New helper method to safely ensure camera stays within map bounds
    pub fn ensure_camera_in_bounds(&mut self) {
        // Calculate maximum camera positions, ensuring they never go negative
        let max_camera_x = (self.map_width - screen_width()).max(0.0);
        let max_camera_y = (self.map_height - screen_height()).max(0.0);
        
        // Clamp camera position
        self.camera_x = self.camera_x.clamp(0.0, max_camera_x);
        self.camera_y = self.camera_y.clamp(0.0, max_camera_y);
    }

    // Add a method to get the effective sound volume (considering mute state)
    #[allow(dead_code)]
    pub fn get_effective_sound_volume(&self) -> f32 {
        if self.sound_muted {
            0.0
        } else {
            self.sound_volume
        }
    }
    
    // Add a method to get the effective music volume (considering mute state)
    #[allow(dead_code)]
    pub fn get_effective_music_volume(&self) -> f32 {
        if self.music_muted {
            0.0
        } else {
            self.music_volume
        }
    }

    // New method for autonomous unit behavior
    fn update_autonomous_behavior(&mut self) {
        let _dt = get_frame_time(); // Added underscore to indicate intentional unused variable
        
        // Make units more autonomous
        for i in 0..self.units.len() {
            let unit_type = self.units[i].unit_type.clone();
            let player_id = self.units[i].player_id;
            let unit_x = self.units[i].x;
            let unit_y = self.units[i].y;
            
            // Auto-assign tasks based on unit type and situation
            match unit_type {
                UnitType::Worker => {
                    // If not carrying resources and no target, find resources
                    if self.units[i].current_resources.unwrap_or(0) == 0 && 
                       self.units[i].target_x.is_none() {
                        if let Some((node_x, node_y)) = self.find_nearest_resource(unit_x, unit_y) {
                            self.units[i].target_x = Some(node_x);
                            self.units[i].target_y = Some(node_y);
                        }
                    }
                },
                UnitType::Fighter | UnitType::Ranger | UnitType::Tank => {
                    // Auto-attack nearby enemies
                    if self.units[i].target_x.is_none() {
                        if let Some((enemy_x, enemy_y)) = self.find_nearest_enemy(unit_x, unit_y, player_id, 150.0) {
                            self.units[i].target_x = Some(enemy_x);
                            self.units[i].target_y = Some(enemy_y);
                        } else {
                            // Patrol behavior - move randomly around
                            if rand::gen_range(0, 100) < 2 { // 2% chance per frame to get new patrol target
                                let patrol_distance = 200.0;
                                let new_x = unit_x + rand::gen_range(-patrol_distance, patrol_distance);
                                let new_y = unit_y + rand::gen_range(-patrol_distance, patrol_distance);
                                self.units[i].target_x = Some(new_x);
                                self.units[i].target_y = Some(new_y);
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }
    
    // Helper method to find nearest resource
    fn find_nearest_resource(&self, x: f32, y: f32) -> Option<(f32, f32)> {
        let mut nearest_distance = f32::MAX;
        let mut nearest_pos = None;
        
        for node in &self.resource_nodes {
            if node.resources > 0 {
                let distance = ((node.x - x).powi(2) + (node.y - y).powi(2)).sqrt();
                if distance < nearest_distance {
                    nearest_distance = distance;
                    nearest_pos = Some((node.x, node.y));
                }
            }
        }
        
        nearest_pos
    }
    
    // Helper method to find nearest enemy
    fn find_nearest_enemy(&self, x: f32, y: f32, player_id: usize, max_range: f32) -> Option<(f32, f32)> {
        let mut nearest_distance = max_range;
        let mut nearest_pos = None;
        
        for unit in &self.units {
            if unit.player_id != player_id && unit.health > 0.0 {
                let distance = ((unit.x - x).powi(2) + (unit.y - y).powi(2)).sqrt();
                if distance < nearest_distance {
                    nearest_distance = distance;
                    nearest_pos = Some((unit.x, unit.y));
                }
            }
        }
        
        nearest_pos
    }
    
    // Generate more resources as map is explored
    fn generate_resources_if_needed(&mut self) {
        // Check if we need more resources in explored areas
        let view_x = self.camera_x;
        let view_y = self.camera_y;
        let view_range = 1000.0;
        
        // Count resources in current view area
        let resources_in_view = self.resource_nodes.iter()
            .filter(|node| {
                let dx = node.x - view_x;
                let dy = node.y - view_y;
                (dx * dx + dy * dy).sqrt() < view_range
            })
            .count();
        
        // If too few resources in view, generate more
        if resources_in_view < 5 {
            for _ in 0..3 {
                let new_x = view_x + rand::gen_range(-view_range, view_range);
                let new_y = view_y + rand::gen_range(-view_range, view_range);
                
                // Don't place too close to existing resources
                let too_close = self.resource_nodes.iter().any(|node| {
                    ((node.x - new_x).powi(2) + (node.y - new_y).powi(2)).sqrt() < 100.0
                });
                
                if !too_close {
                    self.resource_nodes.push(ResourceNode {
                        x: new_x,
                        y: new_y,
                        resources: rand::gen_range(800, 1500),
                        resource_type: if rand::gen_range(0, 100) < 30 { 
                            ResourceType::Energy 
                        } else { 
                            ResourceType::Minerals 
                        },
                        radius: if rand::gen_range(0, 100) < 30 { 20.0 } else { 25.0 },
                    });
                }
            }
        }
    }

    pub fn request_quit(&mut self) {
        self.should_quit = true;
    }
}
