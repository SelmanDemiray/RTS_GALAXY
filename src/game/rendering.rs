use macroquad::prelude::*;
use crate::game::state::GameState;
use crate::entity::{UnitType, ResourceType};
use crate::resources::ResourceManager;

pub fn draw_game(game_state: &GameState, _resource_manager: &ResourceManager) {
    // Draw terrain background
    draw_rectangle(0.0, 0.0, game_state.map_width, game_state.map_height, Color::new(0.1, 0.4, 0.1, 1.0));
    
    // Draw grid lines
    let grid_size = 50.0;
    let start_x = (game_state.camera_x / grid_size).floor() * grid_size;
    let start_y = (game_state.camera_y / grid_size).floor() * grid_size;
    
    for x in (start_x as i32..=(game_state.camera_x + screen_width()) as i32).step_by(grid_size as usize) {
        draw_line(
            x as f32 - game_state.camera_x, 
            0.0, 
            x as f32 - game_state.camera_x, 
            screen_height(),
            1.0, 
            Color::new(0.2, 0.5, 0.2, 0.5)
        );
    }
    
    for y in (start_y as i32..=(game_state.camera_y + screen_height()) as i32).step_by(grid_size as usize) {
        draw_line(
            0.0, 
            y as f32 - game_state.camera_y, 
            screen_width(),
            y as f32 - game_state.camera_y, 
            1.0, 
            Color::new(0.2, 0.5, 0.2, 0.5)
        );
    }
    
    // Draw resource nodes
    for node in &game_state.resource_nodes {
        let color = match node.resource_type {
            ResourceType::Minerals => Color::new(0.1, 0.1, 0.8, 1.0),
            ResourceType::Energy => Color::new(0.9, 0.9, 0.1, 1.0),
        };
        
        draw_circle(
            node.x - game_state.camera_x, 
            node.y - game_state.camera_y, 
            node.radius, 
            color
        );
        
        // Draw resource amount
        let text_size = 12.0;
        let text = node.resources.to_string();
        let text_size = measure_text(&text, None, text_size as u16, 1.0);
        
        draw_text(
            &text,
            node.x - game_state.camera_x - text_size.width / 2.0,
            node.y - game_state.camera_y + text_size.height / 2.0,
            text_size.height,
            WHITE
        );
    }
    
    // Draw all game units
    for unit in &game_state.units {
        let base_color = game_state.players[unit.player_id as usize].color;
        let is_selected = game_state.selected_units.contains(&unit.id);
        let border_size = if is_selected { 2.0 } else { 0.0 };
        
        match unit.unit_type {
            UnitType::Worker => {
                draw_circle_lines(
                    unit.x - game_state.camera_x, 
                    unit.y - game_state.camera_y, 
                    12.0, 
                    border_size, 
                    GREEN
                );
                draw_circle(
                    unit.x - game_state.camera_x, 
                    unit.y - game_state.camera_y, 
                    10.0, 
                    base_color
                );
                
                // Draw resources being carried
                if let Some(resources) = unit.current_resources {
                    if resources > 0 {
                        draw_circle(
                            unit.x - game_state.camera_x + 5.0, 
                            unit.y - game_state.camera_y - 5.0, 
                            5.0, 
                            BLUE
                        );
                    }
                }
            },
            UnitType::Fighter => {
                draw_circle_lines(
                    unit.x - game_state.camera_x, 
                    unit.y - game_state.camera_y, 
                    12.0, 
                    border_size, 
                    GREEN
                );
                draw_rectangle(
                    unit.x - game_state.camera_x - 8.0, 
                    unit.y - game_state.camera_y - 8.0, 
                    16.0, 
                    16.0, 
                    base_color
                );
            },
            UnitType::Ranger => {
                draw_circle_lines(
                    unit.x - game_state.camera_x, 
                    unit.y - game_state.camera_y, 
                    12.0, 
                    border_size, 
                    GREEN
                );
                draw_triangle(
                    Vec2::new(unit.x - game_state.camera_x, unit.y - game_state.camera_y - 10.0),
                    Vec2::new(unit.x - game_state.camera_x - 8.0, unit.y - game_state.camera_y + 5.0),
                    Vec2::new(unit.x - game_state.camera_x + 8.0, unit.y - game_state.camera_y + 5.0),
                    base_color
                );
            },
            UnitType::Tank => {
                draw_circle_lines(
                    unit.x - game_state.camera_x, 
                    unit.y - game_state.camera_y, 
                    15.0, 
                    border_size, 
                    GREEN
                );
                draw_rectangle(
                    unit.x - game_state.camera_x - 12.0, 
                    unit.y - game_state.camera_y - 8.0, 
                    24.0, 
                    16.0, 
                    base_color
                );
                
                // Draw tank turret
                draw_circle(
                    unit.x - game_state.camera_x, 
                    unit.y - game_state.camera_y, 
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
                    unit.x - game_state.camera_x - building_size,
                    unit.y - game_state.camera_y - building_size, 
                    building_size * 2.0, 
                    building_size * 2.0, 
                    border_size,
                    GREEN
                );
                
                draw_rectangle(
                    unit.x - game_state.camera_x - building_size, 
                    unit.y - game_state.camera_y - building_size, 
                    building_size * 2.0, 
                    building_size * 2.0, 
                    base_color
                );
                
                // Draw construction progress if applicable
                if let Some(progress) = unit.construction_progress {
                    if progress < 100.0 {
                        draw_rectangle(
                            unit.x - game_state.camera_x - building_size,
                            unit.y - game_state.camera_y - building_size - 10.0,
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
                    unit.x - game_state.camera_x - hq_size,
                    unit.y - game_state.camera_y - hq_size, 
                    hq_size * 2.0, 
                    hq_size * 2.0, 
                    border_size,
                    GREEN
                );
                
                draw_rectangle(
                    unit.x - game_state.camera_x - hq_size, 
                    unit.y - game_state.camera_y - hq_size, 
                    hq_size * 2.0, 
                    hq_size * 2.0, 
                    base_color
                );
                
                // Draw an 'H' in the center
                let h_text = "HQ";
                let h_text_size = measure_text(h_text, None, 24, 1.0);
                draw_text(
                    h_text,
                    unit.x - game_state.camera_x - h_text_size.width / 2.0,
                    unit.y - game_state.camera_y + h_text_size.height / 2.0,
                    24.0,
                    WHITE
                );
            },
        }
        
        // Draw health bar
        let health_width = 20.0;
        let health_height = 3.0;
        let health_x = unit.x - game_state.camera_x - health_width / 2.0;
        let health_y = unit.y - game_state.camera_y - 15.0;
        
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
    if let (Some(start), Some(end)) = (game_state.selection_start, game_state.selection_end) {
        let x = (start.0 - game_state.camera_x).min(end.0 - game_state.camera_x);
        let y = (start.1 - game_state.camera_y).min(end.1 - game_state.camera_y);
        let width = (start.0 - end.0).abs();
        let height = (start.1 - end.1).abs();
        
        draw_rectangle_lines(x, y, width, height, 1.0, GREEN);
    }
    
    // Draw minimap
    draw_rectangle(
        game_state.minimap_rect.x,
        game_state.minimap_rect.y,
        game_state.minimap_rect.w,
        game_state.minimap_rect.h,
        Color::new(0.0, 0.0, 0.0, 0.7)
    );
    
    draw_rectangle_lines(
        game_state.minimap_rect.x,
        game_state.minimap_rect.y,
        game_state.minimap_rect.w,
        game_state.minimap_rect.h,
        1.0,
        Color::new(0.8, 0.8, 0.8, 0.5)
    );
    
    // Draw units on minimap
    let map_ratio_x = game_state.minimap_rect.w / game_state.map_width;
    let map_ratio_y = game_state.minimap_rect.h / game_state.map_height;
    
    for unit in &game_state.units {
        let minimap_x = game_state.minimap_rect.x + unit.x * map_ratio_x;
        let minimap_y = game_state.minimap_rect.y + unit.y * map_ratio_y;
        let minimap_size = 2.0;
        
        draw_rectangle(
            minimap_x - minimap_size / 2.0,
            minimap_y - minimap_size / 2.0,
            minimap_size,
            minimap_size,
            game_state.players[unit.player_id as usize].color
        );
    }
    
    // Draw camera viewport on minimap
    let viewport_x = game_state.minimap_rect.x + game_state.camera_x * map_ratio_x;
    let viewport_y = game_state.minimap_rect.y + game_state.camera_y * map_ratio_y;
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
    let player = &game_state.players[game_state.current_player_id as usize];
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
    if !game_state.selected_units.is_empty() {
        let mut y_pos = 100.0;
        
        // Show different commands based on selected unit types
        let mut has_workers = false;
        let mut has_buildings = false;
        let mut has_combat_units = false;
        
        for &unit_id in &game_state.selected_units {
            if let Some(unit) = game_state.units.iter().find(|u| u.id == unit_id) {
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
