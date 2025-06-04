use macroquad::prelude::*;
use crate::entity::{Unit, UnitType, BuildingType, Player};
use crate::game::{ZoomSystem, GameScreen};
use crate::entity::resource_node::ResourceNode;

pub struct GameState {
    pub current_screen: GameScreen,
    pub players: Vec<Player>,
    pub current_player_id: usize,
    pub units: Vec<Unit>,
    pub resource_nodes: Vec<ResourceNode>,
    pub selected_units: Vec<u32>,
    pub camera_x: f32,
    pub camera_y: f32,
    pub zoom_system: ZoomSystem,
    pub should_quit: bool,
    
    // Audio settings
    pub sound_volume: f32,
    pub music_volume: f32,
    pub sound_muted: bool,
    pub music_muted: bool,
    
    // UI state
    pub selection_start: Option<(f32, f32)>,
    pub selection_end: Option<(f32, f32)>,
    pub minimap_rect: Rect,
    
    // Game world properties
    pub map_width: f32,
    pub map_height: f32,
    
    // Game timing
    pub game_time: f32,
    pub next_unit_id: u32,
    pub next_node_id: u32,
    
    // Game state flags
    pub game_paused: bool,
    pub debug_mode: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            current_screen: GameScreen::MainMenu,
            players: Vec::new(),
            current_player_id: 0,
            units: Vec::new(),
            resource_nodes: Vec::new(),
            selected_units: Vec::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            zoom_system: ZoomSystem::new(),
            should_quit: false,
            
            // Audio defaults
            sound_volume: 0.8,
            music_volume: 0.7,
            sound_muted: false,
            music_muted: false,
            
            // UI state
            selection_start: None,
            selection_end: None,
            minimap_rect: Rect::new(screen_width() - 210.0, 10.0, 200.0, 150.0),
            
            // World properties
            map_width: 2000.0,
            map_height: 2000.0,
            
            // Game state
            game_time: 0.0,
            next_unit_id: 1,
            next_node_id: 1,
            
            // Flags
            game_paused: false,
            debug_mode: false,
        };
        
        state.initialize_game();
        state
    }
    
    fn initialize_game(&mut self) {
        // Create players
        self.players.push(Player::new(0, "Player".to_string(), BLUE));
        self.players.push(Player::new(1, "AI".to_string(), RED));
        
        // Create headquarters for each player
        let player_hq_id = self.spawn_unit(UnitType::Headquarters, 100.0, 100.0, 0);
        let ai_hq_id = self.spawn_unit(UnitType::Headquarters, 1500.0, 1500.0, 1);
        
        // Set zoom system home position to player HQ
        self.zoom_system.set_home_position(Vec2::new(100.0, 100.0));
        
        // Create some initial workers
        for i in 0..3 {
            self.spawn_unit(UnitType::Worker, 150.0 + i as f32 * 30.0, 150.0, 0);
        }
        
        for i in 0..2 {
            self.spawn_unit(UnitType::Worker, 1450.0 + i as f32 * 30.0, 1550.0, 1);
        }
        
        // Create resource nodes
        self.create_resource_nodes();
        
        // Set initial camera position
        self.camera_x = 100.0;
        self.camera_y = 100.0;
    }
    
    fn create_resource_nodes(&mut self) {
        // Create mineral nodes
        let mineral_positions = vec![
            (300.0, 200.0),
            (150.0, 350.0),
            (800.0, 600.0),
            (1200.0, 1300.0),
            (1600.0, 1200.0),
            (1300.0, 1700.0),
        ];
        
        for (x, y) in mineral_positions {
            let node = ResourceNode::new(
                self.next_node_id,
                x, y,
                crate::entity::resource_node::ResourceType::Minerals,
                1000
            );
            self.resource_nodes.push(node);
            self.next_node_id += 1;
        }
        
        // Create energy nodes
        let energy_positions = vec![
            (500.0, 400.0),
            (1000.0, 800.0),
            (1400.0, 1400.0),
        ];
        
        for (x, y) in energy_positions {
            let node = ResourceNode::new(
                self.next_node_id,
                x, y,
                crate::entity::resource_node::ResourceType::Energy,
                800
            );
            self.resource_nodes.push(node);
            self.next_node_id += 1;
        }
    }
    
    pub fn update(&mut self) {
        if self.game_paused {
            return;
        }
        
        let dt = get_frame_time();
        self.game_time += dt;
        
        // Update zoom system
        self.zoom_system.update(dt);
        
        // Update units
        self.update_units(dt);
        
        // Update camera based on zoom
        self.update_camera();
        
        // Check win conditions
        self.check_win_conditions();
    }
    
    fn update_units(&mut self, dt: f32) {
        for unit in &mut self.units {
            // Update animation
            unit.animation.update(dt);
            unit.update_animation_state();
            
            // Update cooldowns
            if unit.current_cooldown > 0.0 {
                unit.current_cooldown -= dt;
            }
            
            // Process movement
            if let (Some(target_x), Some(target_y)) = (unit.target_x, unit.target_y) {
                let dx = target_x - unit.x;
                let dy = target_y - unit.y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance > 5.0 {
                    // Move towards target
                    let move_distance = unit.speed * dt;
                    unit.x += (dx / distance) * move_distance;
                    unit.y += (dy / distance) * move_distance;
                    unit.is_moving = true;
                    
                    // Update facing direction
                    unit.facing_direction = dy.atan2(dx);
                } else {
                    // Reached target
                    unit.target_x = None;
                    unit.target_y = None;
                    unit.is_moving = false;
                    
                    // If worker reached resource node, start gathering
                    if unit.unit_type == UnitType::Worker && unit.current_resources.unwrap_or(0) == 0 {
                        self.try_gather_resources(unit.id);
                    }
                }
            } else {
                unit.is_moving = false;
            }
            
            // Process resource gathering for workers
            if unit.unit_type == UnitType::Worker && unit.is_gathering {
                self.process_resource_gathering(unit.id, dt);
            }
            
            // Process combat
            if unit.current_cooldown <= 0.0 {
                self.process_unit_combat(unit.id);
            }
        }
        
        // Remove dead units
        self.units.retain(|unit| unit.health > 0.0);
        
        // Update selected units list (remove dead units)
        self.selected_units.retain(|&id| self.units.iter().any(|u| u.id == id));
    }
    
    fn try_gather_resources(&mut self, unit_id: u32) {
        let unit_pos = if let Some(unit) = self.units.iter().find(|u| u.id == unit_id) {
            (unit.x, unit.y, unit.player_id)
        } else {
            return;
        };
        
        // Find nearby resource node
        for node in &self.resource_nodes {
            let distance = ((node.x - unit_pos.0).powi(2) + (node.y - unit_pos.1).powi(2)).sqrt();
            if distance <= 30.0 && node.resources > 0 {
                // Start gathering
                if let Some(unit) = self.units.iter_mut().find(|u| u.id == unit_id) {
                    unit.is_gathering = true;
                    unit.target_resource_id = Some(node.id);
                }
                break;
            }
        }
    }
    
    fn process_resource_gathering(&mut self, unit_id: u32, dt: f32) {
        let gathering_rate = 20.0; // Resources per second
        let gather_amount = (gathering_rate * dt) as u32;
        
        let unit_info = if let Some(unit) = self.units.iter_mut().find(|u| u.id == unit_id) {
            if let Some(node_id) = unit.target_resource_id {
                let current_carried = unit.current_resources.unwrap_or(0);
                let max_capacity = unit.resource_capacity.unwrap_or(50);
                
                if current_carried >= max_capacity {
                    // Return to base
                    unit.is_gathering = false;
                    unit.target_resource_id = None;
                    self.return_worker_to_base(unit_id);
                    return;
                }
                
                Some((node_id, unit.player_id, current_carried, max_capacity))
            } else {
                None
            }
        } else {
            None
        };
        
        if let Some((node_id, player_id, current_carried, max_capacity)) = unit_info {
            // Find the resource node and extract resources
            for node in &mut self.resource_nodes {
                if node.id == node_id && node.resources > 0 {
                    let can_gather = (max_capacity - current_carried).min(gather_amount).min(node.resources);
                    
                    if can_gather > 0 {
                        node.resources -= can_gather;
                        
                        // Update unit's carried resources
                        if let Some(unit) = self.units.iter_mut().find(|u| u.id == unit_id) {
                            let new_amount = unit.current_resources.unwrap_or(0) + can_gather;
                            unit.current_resources = Some(new_amount);
                        }
                    }
                    break;
                }
            }
        }
    }
    
    fn return_worker_to_base(&mut self, worker_id: u32) {
        let worker_info = if let Some(worker) = self.units.iter().find(|u| u.id == worker_id) {
            (worker.player_id, worker.current_resources.unwrap_or(0))
        } else {
            return;
        };
        
        // Find player's headquarters
        for unit in &self.units {
            if unit.player_id == worker_info.0 && unit.unit_type == UnitType::Headquarters {
                // Move worker to HQ
                if let Some(worker) = self.units.iter_mut().find(|u| u.id == worker_id) {
                    worker.target_x = Some(unit.x + 40.0);
                    worker.target_y = Some(unit.y + 40.0);
                }
                break;
            }
        }
    }
    
    fn process_unit_combat(&mut self, unit_id: u32) {
        let attacker_info = if let Some(unit) = self.units.iter().find(|u| u.id == unit_id) {
            if matches!(unit.unit_type, UnitType::Fighter | UnitType::Ranger | UnitType::Tank) {
                Some((unit.x, unit.y, unit.player_id, unit.attack_damage, unit.attack_range, unit.attack_cooldown))
            } else {
                None
            }
        } else {
            None
        };
        
        if let Some((x, y, player_id, damage, range, cooldown)) = attacker_info {
            // Find nearest enemy
            let mut nearest_enemy = None;
            let mut nearest_distance = range;
            
            for target in &self.units {
                if target.player_id != player_id && target.health > 0.0 {
                    let distance = ((target.x - x).powi(2) + (target.y - y).powi(2)).sqrt();
                    if distance <= nearest_distance {
                        nearest_distance = distance;
                        nearest_enemy = Some(target.id);
                    }
                }
            }
            
            // Attack nearest enemy
            if let Some(target_id) = nearest_enemy {
                // Apply damage
                for target in &mut self.units {
                    if target.id == target_id {
                        target.health -= damage;
                        break;
                    }
                }
                
                // Set attacker cooldown
                for attacker in &mut self.units {
                    if attacker.id == unit_id {
                        attacker.current_cooldown = cooldown;
                        attacker.is_attacking = true;
                        break;
                    }
                }
            }
        }
    }
    
    fn update_camera(&mut self) {
        // Update minimap rect if screen size changed
        self.minimap_rect = Rect::new(screen_width() - 210.0, 10.0, 200.0, 150.0);
        
        // Camera movement with keyboard
        let camera_speed = 200.0 * get_frame_time();
        
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
        
        // Handle zoom controls
        let (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
        if mouse_wheel_y > 0.0 {
            self.zoom_system.zoom_in();
        } else if mouse_wheel_y < 0.0 {
            self.zoom_system.zoom_out();
        }
        
        if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
            self.zoom_system.zoom_in();
        }
        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
            self.zoom_system.zoom_out();
        }
        
        // Home key returns to base
        if is_key_pressed(KeyCode::H) || is_key_pressed(KeyCode::Home) {
            let home_pos = self.zoom_system.go_home();
            self.camera_x = home_pos.x;
            self.camera_y = home_pos.y;
        }
        
        // Clamp camera to map bounds
        self.camera_x = self.camera_x.clamp(0.0, self.map_width);
        self.camera_y = self.camera_y.clamp(0.0, self.map_height);
    }
    
    fn check_win_conditions(&mut self) {
        let mut player_has_hq = false;
        let mut ai_has_hq = false;
        
        for unit in &self.units {
            if unit.unit_type == UnitType::Headquarters {
                if unit.player_id == 0 {
                    player_has_hq = true;
                } else if unit.player_id == 1 {
                    ai_has_hq = true;
                }
            }
        }
        
        if !player_has_hq {
            println!("Game Over - Player defeated!");
            // Could transition to defeat screen
        } else if !ai_has_hq {
            println!("Victory! - AI defeated!");
            // Could transition to victory screen
        }
    }
    
    pub fn spawn_unit(&mut self, unit_type: UnitType, x: f32, y: f32, player_id: usize) -> u32 {
        let unit_id = self.next_unit_id;
        self.next_unit_id += 1;
        
        let unit = Unit::new(unit_id, unit_type, x, y, player_id as u8);
        self.units.push(unit);
        
        unit_id
    }
    
    pub fn can_afford(&self, player_id: usize, unit_type: &UnitType) -> bool {
        if player_id >= self.players.len() {
            return false;
        }
        
        let player = &self.players[player_id];
        let (mineral_cost, energy_cost) = self.get_unit_cost(unit_type);
        
        player.minerals >= mineral_cost && player.energy >= energy_cost
    }
    
    pub fn get_unit_cost(&self, unit_type: &UnitType) -> (i32, i32) {
        match unit_type {
            UnitType::Worker => (50, 0),
            UnitType::Fighter => (75, 0),
            UnitType::Ranger => (100, 25),
            UnitType::Tank => (200, 50),
            UnitType::Building => (150, 0), // Base building cost
            UnitType::Headquarters => (0, 0), // Free (starting unit)
        }
    }
    
    pub fn deduct_cost(&mut self, player_id: usize, unit_type: &UnitType) {
        if player_id < self.players.len() {
            let (mineral_cost, energy_cost) = self.get_unit_cost(unit_type);
            self.players[player_id].minerals -= mineral_cost;
            self.players[player_id].energy -= energy_cost;
        }
    }
    
    pub fn request_quit(&mut self) {
        self.should_quit = true;
    }
}
