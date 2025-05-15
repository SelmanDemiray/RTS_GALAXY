use crate::game::{UnitType, GameState, ResourceType, BuildingType};
use macroquad::rand::gen_range;

pub struct AIController {
    think_timer: f32,
    resource_timer: f32,
    attack_timer: f32,
    build_timer: f32,
    last_unit_built: Option<UnitType>,
    has_barracks: bool,
    has_factory: bool,
}

impl AIController {
    pub fn new() -> Self {
        Self {
            think_timer: 0.0,
            resource_timer: 0.0,
            attack_timer: 0.0,
            build_timer: 10.0,  // Initial delay before building
            last_unit_built: None,
            has_barracks: false,
            has_factory: false,
        }
    }
    
    pub fn update(&mut self, game_state: &mut GameState) {
        // Update AI timers
        let dt = 0.016; // Assuming ~60 FPS
        self.think_timer += dt;
        self.resource_timer += dt;
        self.attack_timer += dt;
        self.build_timer += dt;
        
        // Process resource gathering (more frequently)
        if self.resource_timer >= 0.5 {
            self.resource_timer = 0.0;
            self.manage_resources(game_state);
        }
        
        // Process general AI decision making
        if self.think_timer >= 1.0 {
            self.think_timer = 0.0;
            self.make_decisions(game_state);
        }
        
        // Decide on attacking
        if self.attack_timer >= 30.0 {
            self.attack_timer = 0.0;
            self.plan_attack(game_state);
        }
        
        // Check buildings
        self.check_buildings(game_state);
        
        // Handle building and training
        if self.build_timer >= 15.0 {
            self.build_timer = 0.0;
            self.build_or_train(game_state);
        }
    }
    
    fn check_buildings(&mut self, game_state: &GameState) {
        self.has_barracks = false;
        self.has_factory = false;
        
        for unit in &game_state.units {
            if unit.player_id == 1 && unit.unit_type == UnitType::Building {
                if let Some(building_type) = &unit.building_type {
                    if *building_type == BuildingType::Barracks {
                        self.has_barracks = true;
                    }
                    if *building_type == BuildingType::Factory {
                        self.has_factory = true;
                    }
                }
            }
        }
    }
    
    fn build_or_train(&mut self, game_state: &mut GameState) {
        let ai_player = &game_state.players[1];
        
        // Train units if we have enough resources
        if !self.has_barracks && ai_player.minerals >= 150 {
            // Need to build a barracks
            self.build_structure(game_state, BuildingType::Barracks);
        } else if !self.has_factory && ai_player.minerals >= 200 && self.has_barracks {
            // Build a factory next
            self.build_structure(game_state, BuildingType::Factory);
        } else {
            // Train units
            let ai_headquarters = game_state.units.iter().find(|u| 
                u.player_id == 1 && u.unit_type == UnitType::Headquarters
            );
            
            if let Some(headquarters) = ai_headquarters {
                // Count current units
                let mut worker_count = 0;
                let mut fighter_count = 0;
                let mut ranger_count = 0;
                let mut tank_count = 0;
                
                for unit in &game_state.units {
                    if unit.player_id == 1 {
                        match unit.unit_type {
                            UnitType::Worker => worker_count += 1,
                            UnitType::Fighter => fighter_count += 1,
                            UnitType::Ranger => ranger_count += 1,
                            UnitType::Tank => tank_count += 1,
                            _ => {}
                        }
                    }
                }
                
                // Decide what to train
                let hq_x = headquarters.x;
                let hq_y = headquarters.y;
                
                if worker_count < 5 && game_state.can_afford(1, &UnitType::Worker) {
                    // Train worker
                    let _id = game_state.spawn_unit(UnitType::Worker, hq_x + 30.0, hq_y, 1);
                    game_state.deduct_cost(1, &UnitType::Worker);
                    self.last_unit_built = Some(UnitType::Worker);
                } else if self.has_barracks && fighter_count < 5 && game_state.can_afford(1, &UnitType::Fighter) {
                    // Train fighter
                    let barracks = game_state.units.iter().find(|u| 
                        u.player_id == 1 && u.unit_type == UnitType::Building && 
                        u.building_type == Some(BuildingType::Barracks)
                    );
                    
                    if let Some(barracks) = barracks {
                        let _id = game_state.spawn_unit(UnitType::Fighter, barracks.x + 30.0, barracks.y, 1);
                        game_state.deduct_cost(1, &UnitType::Fighter);
                        self.last_unit_built = Some(UnitType::Fighter);
                    }
                } else if self.has_barracks && ranger_count < 4 && game_state.can_afford(1, &UnitType::Ranger) {
                    // Train ranger
                    let barracks = game_state.units.iter().find(|u| 
                        u.player_id == 1 && u.unit_type == UnitType::Building && 
                        u.building_type == Some(BuildingType::Barracks)
                    );
                    
                    if let Some(barracks) = barracks {
                        let _id = game_state.spawn_unit(UnitType::Ranger, barracks.x + 30.0, barracks.y, 1);
                        game_state.deduct_cost(1, &UnitType::Ranger);
                        self.last_unit_built = Some(UnitType::Ranger);
                    }
                } else if self.has_factory && tank_count < 2 && game_state.can_afford(1, &UnitType::Tank) {
                    // Train tank
                    let factory = game_state.units.iter().find(|u| 
                        u.player_id == 1 && u.unit_type == UnitType::Building && 
                        u.building_type == Some(BuildingType::Factory)
                    );
                    
                    if let Some(factory) = factory {
                        let _id = game_state.spawn_unit(UnitType::Tank, factory.x + 40.0, factory.y, 1);
                        game_state.deduct_cost(1, &UnitType::Tank);
                        self.last_unit_built = Some(UnitType::Tank);
                    }
                }
            }
        }
    }
    
    fn build_structure(&self, game_state: &mut GameState, building_type: BuildingType) {
        // Find headquarters to build near it
        if let Some(hq) = game_state.units.iter().find(|u| 
            u.player_id == 1 && u.unit_type == UnitType::Headquarters
        ) {
            // Choose build position
            let build_x = hq.x + gen_range(-100.0, 100.0);
            let build_y = hq.y + gen_range(-100.0, 100.0);
            
            // Spawn the building
            let id = game_state.spawn_unit(UnitType::Building, build_x, build_y, 1);
            
            // Set building type and mark as under construction
            if let Some(building) = game_state.units.iter_mut().find(|u| u.id == id) {
                building.building_type = Some(building_type);
                building.construction_progress = Some(0.0);
            }
            
            // Deduct cost
            game_state.deduct_cost(1, &UnitType::Building);
            
            // Assign some workers to build it
            let mut assigned_workers = 0;
            for unit in &mut game_state.units {
                if unit.player_id == 1 && unit.unit_type == UnitType::Worker && assigned_workers < 2 {
                    unit.target_x = Some(build_x);
                    unit.target_y = Some(build_y);
                    assigned_workers += 1;
                }
            }
        }
    }
    
    fn manage_resources(&mut self, game_state: &mut GameState) {
        // Count idle workers
        let mut idle_worker_ids = Vec::new();
        
        for unit in &game_state.units {
            if unit.player_id == 1 && unit.unit_type == UnitType::Worker {
                if unit.target_x.is_none() && unit.target_y.is_none() {
                    idle_worker_ids.push(unit.id);
                }
            }
        }
        
        // Find closest resource node for each idle worker
        for &worker_id in &idle_worker_ids {
            if let Some(worker_index) = game_state.units.iter().position(|u| u.id == worker_id) {
                let worker = &game_state.units[worker_index];
                
                // Find closest resource node
                let mut closest_node_idx = None;
                let mut closest_dist = f32::MAX;
                
                for (idx, node) in game_state.resource_nodes.iter().enumerate() {
                    // Prefer mineral nodes if we're low on minerals
                    if node.resources > 0 {
                        let dx = node.x - worker.x;
                        let dy = node.y - worker.y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        
                        let player = &game_state.players[1];
                        let minerals_low = player.minerals < 200;
                        
                        // Prioritize minerals if we're low
                        let is_priority = if minerals_low {
                            node.resource_type == ResourceType::Minerals
                        } else {
                            true
                        };
                        
                        if is_priority && dist < closest_dist {
                            closest_dist = dist;
                            closest_node_idx = Some(idx);
                        }
                    }
                }
                
                // Send worker to closest resource
                if let Some(node_idx) = closest_node_idx {
                    let target_x = game_state.resource_nodes[node_idx].x;
                    let target_y = game_state.resource_nodes[node_idx].y;
                    
                    game_state.units[worker_index].target_x = Some(target_x);
                    game_state.units[worker_index].target_y = Some(target_y);
                }
            }
        }
        
        // Update construction progress for AI buildings
        for unit in &mut game_state.units {
            if unit.player_id == 1 && unit.unit_type == UnitType::Building {
                if let Some(progress) = &mut unit.construction_progress {
                    if *progress < 100.0 {
                        *progress += 0.5; // AI builds faster
                    }
                }
            }
        }
    }
    
    fn make_decisions(&mut self, game_state: &mut GameState) {
        // Example: Each combat unit tries to find an enemy or patrol
        for unit_index in 0..game_state.units.len() {
            // Skip if not an AI controlled combat unit
            let unit = &game_state.units[unit_index];
            
            if unit.player_id != 1 || 
               (unit.unit_type != UnitType::Fighter && 
                unit.unit_type != UnitType::Ranger && 
                unit.unit_type != UnitType::Tank) {
                continue;
            }
            
            // If unit has no target, find one
            if unit.target_x.is_none() || unit.target_y.is_none() {
                // Try to find an enemy unit to attack
                let mut closest_enemy = None;
                let mut closest_dist = f32::MAX;
                
                for enemy in &game_state.units {
                    if enemy.player_id == 0 {
                        let dx = enemy.x - unit.x;
                        let dy = enemy.y - unit.y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        
                        if dist < closest_dist {
                            closest_dist = dist;
                            closest_enemy = Some((enemy.x, enemy.y));
                        }
                    }
                }
                
                // Either attack closest enemy or patrol
                if let Some((enemy_x, enemy_y)) = closest_enemy {
                    // Move toward enemy
                    if let Some(unit) = game_state.units.get_mut(unit_index) {
                        unit.target_x = Some(enemy_x);
                        unit.target_y = Some(enemy_y);
                    }
                } else {
                    // Patrol near base
                    let base_x = 900.0; // AI base location
                    let base_y = 700.0;
                    
                    let patrol_x = base_x + gen_range(-150.0, 150.0);
                    let patrol_y = base_y + gen_range(-150.0, 150.0);
                    
                    if let Some(unit) = game_state.units.get_mut(unit_index) {
                        unit.target_x = Some(patrol_x);
                        unit.target_y = Some(patrol_y);
                    }
                }
            }
        }
    }
    
    fn plan_attack(&mut self, game_state: &mut GameState) {
        // Count available combat units
        let mut combat_units = 0;
        for unit in &game_state.units {
            if unit.player_id == 1 && (
                unit.unit_type == UnitType::Fighter || 
                unit.unit_type == UnitType::Ranger ||
                unit.unit_type == UnitType::Tank
            ) {
                combat_units += 1;
            }
        }
        
        // Only attack if we have enough units
        if combat_units >= 5 {
            // Find player's headquarters
            let mut player_hq = None;
            for unit in &game_state.units {
                if unit.player_id == 0 && unit.unit_type == UnitType::Headquarters {
                    player_hq = Some((unit.x, unit.y));
                    break;
                }
            }
            
            if let Some((hq_x, hq_y)) = player_hq {
                // Send all combat units to attack the headquarters
                for unit in &mut game_state.units {
                    if unit.player_id == 1 && (
                        unit.unit_type == UnitType::Fighter || 
                        unit.unit_type == UnitType::Ranger ||
                        unit.unit_type == UnitType::Tank
                    ) {
                        unit.target_x = Some(hq_x);
                        unit.target_y = Some(hq_y);
                    }
                }
            }
        }
    }
}
