use macroquad::prelude::*;
use crate::game::state::GameState;
use crate::resources::manager::ResourceManager;
use crate::entity::UnitType;
use crate::game::types::ResourceType;

pub fn draw_game(game_state: &GameState, resource_manager: &ResourceManager) {
    // Clear and set camera offset
    let camera_x = game_state.camera_x;
    let camera_y = game_state.camera_y;
    
    // Draw background pattern
    draw_background(camera_x, camera_y);
    
    // Draw resource nodes
    for node in &game_state.resource_nodes {
        draw_resource_node(node, camera_x, camera_y, resource_manager);
    }
    
    // Draw units
    for unit in &game_state.units {
        draw_unit(unit, camera_x, camera_y, resource_manager, game_state);
    }
    
    // Draw selection boxes
    draw_selection_indicators(game_state, camera_x, camera_y);
    
    // Draw UI elements
    draw_ui_overlay(game_state, resource_manager);
    
    // Draw minimap
    draw_minimap(game_state);
}

fn draw_background(camera_x: f32, camera_y: f32) {
    // Draw a simple tiled background
    let tile_size = 128.0;
    let start_x = (camera_x / tile_size).floor() as i32;
    let start_y = (camera_y / tile_size).floor() as i32;
    let end_x = start_x + (screen_width() / tile_size).ceil() as i32 + 2;
    let end_y = start_y + (screen_height() / tile_size).ceil() as i32 + 2;
    
    for x in start_x..end_x {
        for y in start_y..end_y {
            let world_x = x as f32 * tile_size - camera_x;
            let world_y = y as f32 * tile_size - camera_y;
            
            // Alternate between two shades of green for a grass effect
            let color = if (x + y) % 2 == 0 {
                Color::new(0.2, 0.6, 0.2, 1.0)
            } else {
                Color::new(0.15, 0.5, 0.15, 1.0)
            };
            
            draw_rectangle(world_x, world_y, tile_size, tile_size, color);
        }
    }
}

fn draw_resource_node(node: &crate::game::resources::ResourceNode, camera_x: f32, camera_y: f32, _resource_manager: &ResourceManager) {
    let screen_x = node.x - camera_x;
    let screen_y = node.y - camera_y;
    
    // Only draw if on screen
    if screen_x > -50.0 && screen_x < screen_width() + 50.0 && 
       screen_y > -50.0 && screen_y < screen_height() + 50.0 {
        
        let color = match node.resource_type {
            ResourceType::Minerals => BLUE,
            ResourceType::Energy => YELLOW,
        };
        
        // Draw resource node
        draw_circle(screen_x, screen_y, node.radius, color);
        
        // Draw resource amount text
        if node.resources > 0 {
            let text = format!("{}", node.resources);
            draw_text(&text, screen_x - 15.0, screen_y - 30.0, 16.0, WHITE);
        }
    }
}

fn draw_unit(unit: &crate::entity::Unit, camera_x: f32, camera_y: f32, resource_manager: &ResourceManager, game_state: &GameState) {
    let screen_x = unit.x - camera_x;
    let screen_y = unit.y - camera_y;
    
    // Only draw if on screen
    if screen_x > -50.0 && screen_x < screen_width() + 50.0 && 
       screen_y > -50.0 && screen_y < screen_height() + 50.0 {
        
        // Get model from resource manager
        let model_name = match unit.unit_type {
            crate::entity::UnitType::Worker => "worker",
            crate::entity::UnitType::Fighter => "fighter",
            crate::entity::UnitType::Ranger => "ranger",
            crate::entity::UnitType::Tank => "tank",
            crate::entity::UnitType::Building => "building",
            crate::entity::UnitType::Headquarters => "headquarters",
        };
        
        if let Some(model) = resource_manager.get_model(model_name) {
            // Determine animation name based on unit state
            let animation_name = match unit.animation.current_state {
                crate::entity::UnitAnimationState::Idle => "idle",
                crate::entity::UnitAnimationState::Walking => "walking",
                crate::entity::UnitAnimationState::Running => "running",
                crate::entity::UnitAnimationState::Attacking => {
                    match unit.unit_type {
                        crate::entity::UnitType::Fighter => "melee_attack",
                        crate::entity::UnitType::Ranger => "shooting",
                        crate::entity::UnitType::Tank => "firing_cannon",
                        _ => "attacking",
                    }
                },
                crate::entity::UnitAnimationState::Gathering => {
                    if unit.current_resources.unwrap_or(0) > 0 {
                        "carrying_resources"
                    } else {
                        "gathering_minerals" // Default, could be determined by target
                    }
                },
                crate::entity::UnitAnimationState::Building => "building",
                crate::entity::UnitAnimationState::Dying => "dying",
                crate::entity::UnitAnimationState::Special => {
                    match unit.unit_type {
                        crate::entity::UnitType::Fighter => "victory_pose",
                        crate::entity::UnitType::Ranger => "reloading",
                        _ => "idle",
                    }
                },
            };
            
            // Draw the 3D model with animation
            let position = Vec3::new(screen_x, screen_y, 0.0);
            let rotation = Vec3::new(0.0, unit.facing_direction, 0.0);
            let scale = match unit.unit_type {
                crate::entity::UnitType::Worker => 15.0,
                crate::entity::UnitType::Fighter => 18.0,
                crate::entity::UnitType::Ranger => 16.0,
                crate::entity::UnitType::Tank => 25.0,
                crate::entity::UnitType::Building => 40.0,
                crate::entity::UnitType::Headquarters => 50.0,
            };
            
            model.draw_with_animation(position, rotation, scale, animation_name, unit.animation.animation_time);
        } else {
            // Fallback to simple circle rendering
            let base_color = if unit.player_id == game_state.current_player_id {
                BLUE
            } else {
                RED
            };
            
            let size = match unit.unit_type {
                crate::entity::UnitType::Worker => 15.0,
                crate::entity::UnitType::Fighter => 18.0,
                crate::entity::UnitType::Ranger => 16.0,
                crate::entity::UnitType::Tank => 25.0,
                crate::entity::UnitType::Building => 40.0,
                crate::entity::UnitType::Headquarters => 50.0,
            };
            
            draw_circle(screen_x, screen_y, size, base_color);
        }
        
        // Draw health bar if unit is damaged
        if unit.health < unit.max_health {
            let size = match unit.unit_type {
                crate::entity::UnitType::Worker => 15.0,
                crate::entity::UnitType::Fighter => 18.0,
                crate::entity::UnitType::Ranger => 16.0,
                crate::entity::UnitType::Tank => 25.0,
                crate::entity::UnitType::Building => 40.0,
                crate::entity::UnitType::Headquarters => 50.0,
            };
            
            let bar_width = size * 2.0;
            let bar_height = 4.0;
            let health_ratio = unit.health / unit.max_health;
            
            // Background
            draw_rectangle(screen_x - bar_width/2.0, screen_y - size - 10.0, bar_width, bar_height, RED);
            // Health
            draw_rectangle(screen_x - bar_width/2.0, screen_y - size - 10.0, bar_width * health_ratio, bar_height, GREEN);
        }
        
        // Draw resource carrying indicator for workers
        if unit.unit_type == crate::entity::UnitType::Worker {
            if let Some(resources) = unit.current_resources {
                if resources > 0 {
                    draw_circle(screen_x + 8.0, screen_y - 8.0, 4.0, GOLD);
                    let text = format!("{}", resources);
                    draw_text(&text, screen_x + 15.0, screen_y - 5.0, 12.0, WHITE);
                }
            }
        }
    }
}

fn draw_selection_indicators(game_state: &GameState, camera_x: f32, camera_y: f32) {
    // Draw selection circles for selected units
    for &unit_id in &game_state.selected_units {
        if let Some(unit) = game_state.units.iter().find(|u| u.id == unit_id) {
            let screen_x = unit.x - camera_x;
            let screen_y = unit.y - camera_y;
            
            let size = match unit.unit_type {
                UnitType::Worker => 20.0,
                UnitType::Fighter => 23.0,
                UnitType::Ranger => 21.0,
                UnitType::Tank => 30.0,
                UnitType::Building => 45.0,
                UnitType::Headquarters => 55.0,
            };
            
            // Draw selection circle
            draw_circle_lines(screen_x, screen_y, size, 2.0, GREEN);
        }
    }
    
    // Draw selection box if dragging
    if let (Some(start), Some(end)) = (game_state.selection_start, game_state.selection_end) {
        let start_screen_x = start.0 - camera_x;
        let start_screen_y = start.1 - camera_y;
        let end_screen_x = end.0 - camera_x;
        let end_screen_y = end.1 - camera_y;
        
        let rect_x = start_screen_x.min(end_screen_x);
        let rect_y = start_screen_y.min(end_screen_y);
        let rect_w = (start_screen_x - end_screen_x).abs();
        let rect_h = (start_screen_y - end_screen_y).abs();
        
        draw_rectangle_lines(rect_x, rect_y, rect_w, rect_h, 2.0, GREEN);
    }
}

fn draw_ui_overlay(game_state: &GameState, _resource_manager: &ResourceManager) {
    // Draw resource counter
    let player = &game_state.players[game_state.current_player_id as usize];
    let resources_text = format!("Minerals: {} | Energy: {}", player.minerals, player.energy);
    draw_text(&resources_text, 10.0, 30.0, 24.0, WHITE);
    
    // Draw selected unit info
    if !game_state.selected_units.is_empty() {
        let info_text = format!("Selected: {} units", game_state.selected_units.len());
        draw_text(&info_text, 10.0, 60.0, 20.0, WHITE);
    }
}

fn draw_minimap(game_state: &GameState) {
    let minimap = &game_state.minimap_rect;
    
    // Draw minimap background
    draw_rectangle(minimap.x, minimap.y, minimap.w, minimap.h, Color::new(0.0, 0.0, 0.0, 0.7));
    draw_rectangle_lines(minimap.x, minimap.y, minimap.w, minimap.h, 2.0, WHITE);
    
    // Draw units on minimap
    let scale_x = minimap.w / game_state.map_width;
    let scale_y = minimap.h / game_state.map_height;
    
    for unit in &game_state.units {
        let minimap_x = minimap.x + unit.x * scale_x;
        let minimap_y = minimap.y + unit.y * scale_y;
        
        let color = if unit.player_id == game_state.current_player_id {
            BLUE
        } else {
            RED
        };
        
        draw_circle(minimap_x, minimap_y, 2.0, color);
    }
    
    // Draw camera view on minimap
    let view_x = minimap.x + game_state.camera_x * scale_x;
    let view_y = minimap.y + game_state.camera_y * scale_y;
    let view_w = screen_width() * scale_x;
    let view_h = screen_height() * scale_y;
    
    draw_rectangle_lines(view_x, view_y, view_w, view_h, 1.0, WHITE);
}
