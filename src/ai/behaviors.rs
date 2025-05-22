use crate::game::{GameState, BuildingType, UnitType, ResourceType};
use macroquad::rand::gen_range;

pub fn manage_resources(game_state: &mut GameState) {
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

pub fn build_structure(game_state: &mut GameState, building_type: BuildingType) {
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

pub fn plan_attack(game_state: &mut GameState) {
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

pub fn make_decisions(game_state: &mut GameState) {
    // For each combat unit, try to find an enemy or patrol
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
