use macroquad::prelude::*;
use crate::game::state::GameState;
use crate::resources::manager::ResourceManager;
use crate::entity::{ResourceNode, UnitType};
use crate::game::types::ResourceType;

pub fn render_game(game_state: &GameState, camera_x: f32, camera_y: f32, resource_manager: &ResourceManager) {
    // Clear and set camera offset
    let camera_x = game_state.camera_x;
    let camera_y = game_state.camera_y;
    
    // Draw background pattern
    draw_background(camera_x, camera_y);
    
    // Draw resource nodes
    for node in &game_state.resource_nodes {
        draw_resource_node(node, camera_x, camera_y, resource_manager);
    }
    
    // Render units
    for unit in &game_state.units {
        draw_unit(unit, camera_x, camera_y, resource_manager);
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

fn draw_resource_node(node: &ResourceNode, camera_x: f32, camera_y: f32, _resource_manager: &ResourceManager) {
    let screen_x = node.x - camera_x + screen_width() / 2.0;
    let screen_y = node.y - camera_y + screen_height() / 2.0;
    
    let size = 15.0;
    
    let color = match node.resource_type {
        ResourceType::Minerals => BLUE,
        ResourceType::Energy => YELLOW,
    };
    
    draw_circle(screen_x, screen_y, size, color);
}

fn draw_unit(unit: &Unit, camera_x: f32, camera_y: f32, resource_manager: &ResourceManager) {
    let screen_x = unit.x - camera_x + screen_width() / 2.0;
    let screen_y = unit.y - camera_y + screen_height() / 2.0;

    // Get unit color based on type
    let color = match unit.unit_type {
        UnitType::Worker => BLUE,
        UnitType::Fighter => RED,
        UnitType::Ranger => GREEN,
        UnitType::Tank => DARKGRAY,
        UnitType::Building => BROWN,
        UnitType::Headquarters => PURPLE,
    };

    // Get model name for asset loading
    let model_name = match unit.unit_type {
        UnitType::Worker => "worker",
        UnitType::Fighter => "fighter",
        UnitType::Ranger => "ranger",
        UnitType::Tank => "tank",
        UnitType::Building => "building",
        UnitType::Headquarters => "headquarters",
    };

    // Get appropriate animation based on state and unit type
    let animation_name = match unit.animation.current_state {
        UnitAnimationState::Attacking => {
            match unit.unit_type {
                UnitType::Fighter => "melee_attack",
                UnitType::Ranger => "shooting",
                UnitType::Tank => "firing_cannon",
                _ => "idle",
            }
        },
        UnitAnimationState::Walking => "walking",
        UnitAnimationState::Running => "running",
        UnitAnimationState::Gathering => "gathering_minerals",
        UnitAnimationState::Building => "building",
        UnitAnimationState::Dying => "dying",
        UnitAnimationState::Special => {
            match unit.unit_type {
                UnitType::Fighter => "victory_pose",
                UnitType::Ranger => "reloading",
                _ => "idle",
            }
        },
        _ => "idle",
    };

    // Get unit size for rendering
    let size = match unit.unit_type {
        UnitType::Worker => 15.0,
        UnitType::Fighter => 18.0,
        UnitType::Ranger => 16.0,
        UnitType::Tank => 25.0,
        UnitType::Building => 40.0,
        UnitType::Headquarters => 50.0,
    };

    // For now, render as a simple circle (placeholder for 3D model)
    draw_circle(screen_x, screen_y, size, color);

    // Draw health bar
    let health_ratio = unit.health / unit.max_health;
    let bar_width = size * 1.5;
    let bar_height = 4.0;
    let bar_x = screen_x - bar_width / 2.0;
    let bar_y = screen_y - size - 10.0;

    // Background
    draw_rectangle(bar_x, bar_y, bar_width, bar_height, BLACK);
    // Health
    let health_color = if health_ratio > 0.6 { GREEN } else if health_ratio > 0.3 { YELLOW } else { RED };
    draw_rectangle(bar_x, bar_y, bar_width * health_ratio, bar_height, health_color);

    // Draw selection indicator
    if unit.is_selected {
        draw_circle_lines(screen_x, screen_y, size + 5.0, 2.0, WHITE);
    }

    // Draw resource gathering indicator for workers
    if unit.unit_type == UnitType::Worker {
        if unit.animation.current_state == UnitAnimationState::Gathering {
            // Draw gathering indicator
            draw_circle(screen_x, screen_y - size - 15.0, 3.0, YELLOW);
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
        let start_screen_x = start.x - camera_x;
        let start_screen_y = start.y - camera_y;
        let end_screen_x = end.x - camera_x;
        let end_screen_y = end.y - camera_y;
        
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

pub fn draw_selection_box(game_state: &GameState, camera_x: f32, camera_y: f32) {
    if let (Some(start), Some(end)) = (game_state.selection_start, game_state.selection_end) {
        let start_screen_x = start.x - camera_x;
        let start_screen_y = start.y - camera_y;
        let end_screen_x = end.x - camera_x;
        let end_screen_y = end.y - camera_y;
        
        let rect_x = start_screen_x.min(end_screen_x);
        let rect_y = start_screen_y.min(end_screen_y);
        let rect_w = (start_screen_x - end_screen_x).abs();
        let rect_h = (start_screen_y - end_screen_y).abs();
        
        draw_rectangle_lines(rect_x, rect_y, rect_w, rect_h, 2.0, GREEN);
    }
}

pub fn draw_hud(game_state: &GameState) {
    let player = &game_state.players[game_state.current_player as usize];
    
    // Draw resource counter
    let resources_text = format!("Minerals: {} | Energy: {}", player.minerals, player.energy);
    draw_text(&resources_text, 10.0, 30.0, 24.0, WHITE);
    
    // Draw selected unit info
    if !game_state.selected_units.is_empty() {
        let info_text = format!("Selected: {} units", game_state.selected_units.len());
        draw_text(&info_text, 10.0, 60.0, 20.0, WHITE);
    }
}

pub fn draw_units(game_state: &GameState, resource_manager: &ResourceManager, camera_x: f32, camera_y: f32) {
    for unit in &game_state.units {
        let screen_x = unit.x - camera_x;
        let screen_y = unit.y - camera_y;
        
        // Determine unit color based on player
        let color = if unit.player_id == game_state.current_player {
            GREEN
        } else {
            RED
        };
        
        // Get unit size for rendering
        let size = match unit.unit_type {
            UnitType::Worker => 15.0,
            UnitType::Fighter => 18.0,
            UnitType::Ranger => 16.0,
            UnitType::Tank => 25.0,
            UnitType::Building => 40.0,
            UnitType::Headquarters => 50.0,
        };

        // For now, render as a simple circle (placeholder for 3D model)
        draw_circle(screen_x, screen_y, size, color);

        // Draw health bar
        let health_ratio = unit.health / unit.max_health;
        let bar_width = size * 1.5;
        let bar_height = 4.0;
        let bar_x = screen_x - bar_width / 2.0;
        let bar_y = screen_y - size - 10.0;

        // Background
        draw_rectangle(bar_x, bar_y, bar_width, bar_height, BLACK);
        // Health
        let health_color = if health_ratio > 0.6 { GREEN } else if health_ratio > 0.3 { YELLOW } else { RED };
        draw_rectangle(bar_x, bar_y, bar_width * health_ratio, bar_height, health_color);

        // Draw selection indicator
        if unit.is_selected {
            draw_circle_lines(screen_x, screen_y, size + 5.0, 2.0, WHITE);
        }

        // Draw resource gathering indicator for workers
        if unit.unit_type == UnitType::Worker {
            if unit.animation.current_state == UnitAnimationState::Gathering {
                // Draw gathering indicator
                draw_circle(screen_x, screen_y - size - 15.0, 3.0, YELLOW);
            }
        }
    }
}
