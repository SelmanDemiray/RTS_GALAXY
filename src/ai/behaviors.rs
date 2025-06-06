use crate::game::state::GameState;
use crate::entity::{BuildingType, UnitType};
use macroquad::rand::gen_range;

pub fn manage_resources(game_state: &mut GameState) {
    let ai_player_id = 1;
    
    // Find all AI workers
    let mut worker_positions = Vec::new();
    for unit in &game_state.units {
        if unit.player_id == ai_player_id && unit.unit_type == UnitType::Worker {
            let carrying = unit.current_resources.unwrap_or(0);
            worker_positions.push((unit.id, unit.x, unit.y, carrying));
        }
    }
    
    // Assign workers to nearby resource nodes if they're not carrying resources
    for (worker_id, worker_x, worker_y, carrying) in worker_positions {
        if carrying == 0 {
            // Find nearest resource node
            let mut nearest_distance = f32::MAX;
            let mut nearest_node_pos = None;
            
            for node in &game_state.resource_nodes {
                if node.resources > 0 {
                    let distance = ((node.x - worker_x).powi(2) + (node.y - worker_y).powi(2)).sqrt();
                    if distance < nearest_distance {
                        nearest_distance = distance;
                        nearest_node_pos = Some((node.x, node.y));
                    }
                }
            }
            
            // Assign worker to gather from nearest node
            if let Some((node_x, node_y)) = nearest_node_pos {
                for unit in &mut game_state.units {
                    if unit.id == worker_id {
                        unit.target_x = Some(node_x);
                        unit.target_y = Some(node_y);
                        break;
                    }
                }
            }
        }
    }
}

pub fn build_structure(game_state: &mut GameState, building_type: BuildingType) {
    let ai_player_id = 1; // AI is player 1
    
    // Find AI headquarters position
    let hq_pos = game_state.units.iter()
        .find(|unit| unit.player_id == ai_player_id && 
              unit.building_type == Some(BuildingType::Headquarters))
        .map(|unit| (unit.x, unit.y));
    
    if let Some((hq_x, hq_y)) = hq_pos {
        let cost = building_type.get_cost();
        if game_state.players[ai_player_id].minerals >= cost {
            // Find a suitable build location near headquarters
            let build_x = hq_x + 100.0;
            let build_y = hq_y + 50.0;
            
            // Create the building
            let building_id = game_state.spawn_unit(UnitType::Building, build_x, build_y, ai_player_id);
            
            // Set building type
            for unit in &mut game_state.units {
                if unit.id == building_id {
                    unit.building_type = Some(building_type);
                    unit.construction_progress = Some(0.0);
                    break;
                }
            }
            
            // Deduct cost
            game_state.players[ai_player_id].minerals -= cost;
        }
    }
}

fn get_building_cost(building_type: &BuildingType) -> i32 {
    match building_type {
        BuildingType::Barracks => 150,
        BuildingType::ResourceDepot => 175,
        BuildingType::DefenseTurret => 100,
        _ => 200,
    }
}

pub fn plan_attack(game_state: &mut GameState) {
    let ai_player_id = 1;
    
    // Count combat units
    let mut combat_units = Vec::new();
    for unit in &game_state.units {
        if unit.player_id == ai_player_id && 
           matches!(unit.unit_type, UnitType::Fighter | UnitType::Ranger | UnitType::Tank) {
            combat_units.push(unit.id);
        }
    }
    
    // If we have enough units, launch an attack on enemy base
    if combat_units.len() >= 3 {
        // Find enemy headquarters
        let mut enemy_hq_pos = None;
        for unit in &game_state.units {
            if unit.player_id != ai_player_id && unit.unit_type == UnitType::Headquarters {
                enemy_hq_pos = Some((unit.x, unit.y));
                break;
            }
        }
        
        if let Some((hq_x, hq_y)) = enemy_hq_pos {
            // Send all combat units to attack enemy HQ
            for unit_id in combat_units {
                for unit in &mut game_state.units {
                    if unit.id == unit_id {
                        unit.target_x = Some(hq_x + gen_range(-50.0, 50.0));
                        unit.target_y = Some(hq_y + gen_range(-50.0, 50.0));
                        break;
                    }
                }
            }
        }
    }
}

pub fn make_decisions(game_state: &mut GameState) {
    let ai_player_id = 1;
    
    // Count different unit types
    let mut _worker_count = 0;    // Fixed unused variable warning with underscore
    let mut _fighter_count = 0;   // Fixed unused variable warning with underscore
    let mut ai_units = Vec::new();
    
    for unit in &game_state.units {
        if unit.player_id == ai_player_id {
            ai_units.push((unit.id, unit.x, unit.y, unit.unit_type.clone()));
            match unit.unit_type {
                UnitType::Worker => _worker_count += 1,
                UnitType::Fighter => _fighter_count += 1,
                _ => {}
            }
        }
    }
    
    // Make combat units patrol and attack enemies
    for (unit_id, unit_x, unit_y, unit_type) in ai_units {
        if matches!(unit_type, UnitType::Fighter | UnitType::Ranger | UnitType::Tank) {
            // Look for nearby enemies
            let mut nearest_enemy = None;
            let mut nearest_distance = 200.0; // Attack range
            
            for enemy in &game_state.units {
                if enemy.player_id != ai_player_id {
                    let distance = ((enemy.x - unit_x).powi(2) + (enemy.y - unit_y).powi(2)).sqrt();
                    if distance < nearest_distance {
                        nearest_distance = distance;
                        nearest_enemy = Some((enemy.x, enemy.y));
                    }
                }
            }
            
            // Attack nearest enemy or patrol
            for unit in &mut game_state.units {
                if unit.id == unit_id {
                    if let Some((enemy_x, enemy_y)) = nearest_enemy {
                        unit.target_x = Some(enemy_x);
                        unit.target_y = Some(enemy_y);
                    } else if unit.target_x.is_none() {
                        // Patrol around base
                        let patrol_x = unit_x + gen_range(-100.0, 100.0);
                        let patrol_y = unit_y + gen_range(-100.0, 100.0);
                        unit.target_x = Some(patrol_x);
                        unit.target_y = Some(patrol_y);
                    }
                    break;
                }
            }
        }
    }
}
