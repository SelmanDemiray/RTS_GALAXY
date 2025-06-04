use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use crate::game::{GameState, GameMode};
use crate::game::screens::GameScreen;
use crate::network::NetworkClient;
use crate::resources::manager::ResourceManager;
use crate::audio::AudioManager;
use crate::entity::{BuildingType, UnitType};
use crate::game::commands::Command;

pub fn draw_ui(
    game_state: &mut GameState,
    network_client: &mut NetworkClient,
    resource_manager: &ResourceManager,
    audio_manager: &mut AudioManager,
) {
    // Draw top bar with resources
    let bar_height = 80.0; // Increased height for zoom info
    draw_rectangle(0.0, 0.0, screen_width(), bar_height, Color::new(0.1, 0.1, 0.3, 0.8));
    
    if let Some(player) = game_state.players.get(game_state.current_player_id as usize) {
        // Draw minerals
        let minerals_text = format!("Minerals: {}", player.minerals);
        draw_text(&minerals_text, 20.0, 25.0, 20.0, WHITE);
        
        // Draw energy
        let energy_text = format!("Energy: {}", player.energy);
        draw_text(&energy_text, 200.0, 25.0, 20.0, WHITE);
    }
    
    // Draw zoom information
    let zoom_label = game_state.zoom_system.get_zoom_label();
    let zoom_desc = game_state.zoom_system.get_zoom_description();
    draw_text(&zoom_label, 20.0, 50.0, 16.0, YELLOW);
    draw_text(&zoom_desc, 20.0, 70.0, 14.0, LIGHTGRAY);
    
    // Draw zoom controls help
    let zoom_help = "Zoom: +/- or Mouse Wheel | Home: H | Quick: 1-9";
    draw_text(zoom_help, screen_width() - 400.0, 25.0, 14.0, LIGHTGRAY);
    
    // Draw home button
    if draw_button(screen_width() - 120.0, 45.0, 100.0, 25.0, "Home (H)") {
        audio_manager.play_ui_click(resource_manager, game_state);
        let home_pos = game_state.zoom_system.go_home();
        game_state.camera_x = home_pos.x;
        game_state.camera_y = home_pos.y;
    }
    
    // Draw mini-map
    draw_rectangle(
        game_state.minimap_rect.x,
        game_state.minimap_rect.y,
        game_state.minimap_rect.w,
        game_state.minimap_rect.h,
        Color::new(0.1, 0.3, 0.1, 0.8)
    );
    
    // Draw units on minimap
    for unit in &game_state.units {
        let map_ratio_x = game_state.minimap_rect.w / game_state.map_width;
        let map_ratio_y = game_state.minimap_rect.h / game_state.map_height;
        
        let mini_x = game_state.minimap_rect.x + unit.x * map_ratio_x;
        let mini_y = game_state.minimap_rect.y + unit.y * map_ratio_y;
        
        let color = game_state.players[unit.player_id as usize].color;
        
        draw_circle(mini_x, mini_y, 2.0, color);
    }
    
    // Draw camera viewport on minimap
    let view_x = game_state.minimap_rect.x + game_state.camera_x * game_state.minimap_rect.w / game_state.map_width;
    let view_y = game_state.minimap_rect.y + game_state.camera_y * game_state.minimap_rect.h / game_state.map_height;
    let view_w = screen_width() * game_state.minimap_rect.w / game_state.map_width;
    let view_h = screen_height() * game_state.minimap_rect.h / game_state.map_height;
    
    draw_rectangle_lines(view_x, view_y, view_w, view_h, 1.0, WHITE);
    
    // Draw action buttons if units are selected
    if !game_state.selected_units.is_empty() {
        draw_unit_info(game_state, resource_manager, audio_manager);
    }
    
    // Draw messages (chat or game notifications)
    let mut y = screen_height() - 150.0;
    for message in game_state.messages.iter().rev().take(5) {
        draw_text(message, 10.0, y, 16.0, WHITE);
        y -= 20.0;
    }
    
    // Draw game time
    let minutes = (game_state.game_time / 60.0) as i32;
    let seconds = (game_state.game_time % 60.0) as i32;
    let time_text = format!("Time: {:02}:{:02}", minutes, seconds);
    draw_text(&time_text, screen_width() - 120.0, bar_height - 15.0, 16.0, WHITE);
    
    // Draw back to menu button
    if draw_button(10.0, bar_height + 10.0, 120.0, 30.0, "Main Menu") {
        audio_manager.play_ui_click(resource_manager, game_state);
        game_state.current_screen = GameScreen::MainMenu;
    }
    
    // Draw online/offline status
    let status_text = match game_state.game_mode {
        GameMode::Online => {
            if network_client.is_connected() {
                "Online"
            } else {
                "Connecting..."
            }
        },
        GameMode::Offline => "Offline"
    };
    draw_text(status_text, screen_width() - 100.0, bar_height + 20.0, 16.0, WHITE);
    
    // Show enhanced controls with zoom information
    let controls_text = "WASD: Camera | Mouse: Select | H: Home | +/-: Zoom | 1-9: Quick Zoom";
    draw_text(controls_text, 10.0, screen_height() - 30.0, 14.0, LIGHTGRAY);
    
    // Draw zoom level indicator
    let zoom_indicator = format!("Zoom Level: {}/50", game_state.zoom_system.current_level);
    draw_text(&zoom_indicator, 10.0, screen_height() - 50.0, 16.0, WHITE);
    
    // Replace egui_macroquad with macroquad native UI
    root_ui().window(hash!(), vec2(10.0, 120.0), vec2(200.0, 150.0), |ui| {
        ui.label(None, "Commands");
        if !game_state.selected_units.is_empty() {
            if ui.button(None, "Build Barracks") {
                audio_manager.play_ui_click(resource_manager, game_state);
                game_state.current_command = Some(Command::Build { 
                    building_type: BuildingType::Barracks,
                    x: 0.0,
                    y: 0.0 
                });
            }
            if ui.button(None, "Gather Resources") {
                audio_manager.play_ui_click(resource_manager, game_state);
                game_state.current_command = Some(Command::Gather { resource_id: 0 });
            }
        }
    });

    // Zoom Control Panel
    root_ui().window(hash!(), vec2(220.0, 120.0), vec2(200.0, 200.0), |ui| {
        ui.label(None, &format!("Zoom Level: {}/50", game_state.zoom_system.current_level));
        
        ui.horizontal(|ui| {
            if ui.button(None, "Zoom In (+)") && game_state.zoom_system.current_level > 1 {
                game_state.zoom_system.zoom_in();
            }
            if ui.button(None, "Zoom Out (-)") && game_state.zoom_system.current_level < 50 {
                game_state.zoom_system.zoom_out();
            }
            if ui.button(None, "Home (H)") {
                let home_pos = game_state.zoom_system.go_home();
                game_state.camera_x = home_pos.x;
                game_state.camera_y = home_pos.y;
            }
        });
        
        ui.separator();
        ui.label(None, "Quick Zoom:");
        ui.horizontal(|ui| {
            if ui.button(None, "1: Unit") { game_state.zoom_system.set_zoom_level(1); }
            if ui.button(None, "2: Village") { game_state.zoom_system.set_zoom_level(5); }
            if ui.button(None, "3: City") { game_state.zoom_system.set_zoom_level(10); }
            if ui.button(None, "4: Region") { game_state.zoom_system.set_zoom_level(15); }
        });
        ui.horizontal(|ui| {
            if ui.button(None, "5: Planet") { game_state.zoom_system.set_zoom_level(20); }
            if ui.button(None, "6: System") { game_state.zoom_system.set_zoom_level(25); }
            if ui.button(None, "7: Cluster") { game_state.zoom_system.set_zoom_level(30); }
            if ui.button(None, "8: Galaxy") { game_state.zoom_system.set_zoom_level(35); }
        });
        
        ui.separator();
        ui.label(None, &game_state.zoom_system.get_zoom_description());
    });

    // Unit Training Panel
    root_ui().window(hash!(), vec2(430.0, 120.0), vec2(180.0, 120.0), |ui| {
        ui.label(None, "Train Units");
        if ui.button(None, "Train Worker (50)") && game_state.can_afford(0, &UnitType::Worker) {
            audio_manager.play_build_sound(resource_manager, game_state);
            
            // Find HQ position
            if let Some(headquarters) = game_state.units.iter().find(|u| 
                u.unit_type == UnitType::Headquarters && u.player_id == game_state.current_player_id) {
                let hq_pos = (headquarters.x, headquarters.y);
                let spawn_pos = (hq_pos.0 + 50.0, hq_pos.1 + 50.0);
                
                // Create worker and deduct cost
                game_state.spawn_unit(UnitType::Worker, spawn_pos.0, spawn_pos.1, game_state.current_player_id);
                game_state.deduct_cost(game_state.current_player_id as usize, &UnitType::Worker);
            }
        }
    });

    // Combat Panel
    root_ui().window(hash!(), vec2(620.0, 120.0), vec2(150.0, 100.0), |ui| {
        ui.label(None, "Combat");
        if !game_state.selected_units.is_empty() {
            if ui.button(None, "Attack Move") {
                audio_manager.play_ui_click(resource_manager, game_state);
                game_state.current_command = Some(Command::Attack { target_id: 0 });
            }
        }
    });
}

fn draw_unit_info(
    game_state: &mut GameState, 
    resource_manager: &ResourceManager,
    audio_manager: &mut AudioManager
) {
    let panel_height = 150.0;
    let panel_y = screen_height() - panel_height;
    
    // Draw panel background
    draw_rectangle(0.0, panel_y, screen_width() - game_state.minimap_rect.w - 10.0, panel_height, Color::new(0.1, 0.1, 0.3, 0.8));
    
    // Find first selected unit to show info
    if let Some(selected_id) = game_state.selected_units.first() {
        if let Some(unit) = game_state.units.iter().find(|u| u.id == *selected_id) {
            // Draw unit name and info
            let unit_type_str = match unit.unit_type {
                UnitType::Worker => "Worker",
                UnitType::Fighter => "Fighter",
                UnitType::Ranger => "Ranger",
                UnitType::Tank => "Tank",
                UnitType::Building => "Building",
                UnitType::Headquarters => "Headquarters",
            };
            
            draw_text(&format!("{} (ID: {})", unit_type_str, unit.id), 20.0, panel_y + 30.0, 20.0, WHITE);
            draw_text(&format!("Health: {}/{}", unit.health, unit.max_health), 20.0, panel_y + 60.0, 16.0, WHITE);
            
            // Draw action buttons based on unit type
            let button_width = 120.0;
            let button_height = 40.0;
            let button_spacing = 10.0;
            let button_start_x = 20.0;
            let button_y = panel_y + 80.0;
            
            match unit.unit_type {
                UnitType::Worker => {
                    if draw_button(button_start_x, button_y, button_width, button_height, "Build") {
                        audio_manager.play_ui_click(resource_manager, game_state);
                        game_state.current_command = Some(Command::Build {
                            building_type: BuildingType::Barracks,
                            x: 0.0,
                            y: 0.0
                        });
                    }
                    
                    if draw_button(button_start_x + button_width + button_spacing, button_y, button_width, button_height, "Gather") {
                        audio_manager.play_ui_click(resource_manager, game_state);
                        game_state.current_command = Some(Command::Gather { resource_id: 0 });
                    }
                },
                UnitType::Headquarters => {
                    let player = &game_state.players[game_state.current_player_id];
                    
                    // Highlight button if can afford, gray out if cannot
                    let can_afford_worker = player.minerals >= 50;
                    if draw_button_colored(
                        button_start_x, 
                        button_y, 
                        button_width, 
                        button_height, 
                        "Train Worker (50)", 
                        if can_afford_worker { SKYBLUE } else { GRAY }
                    ) && can_afford_worker {
                        audio_manager.play_build_sound(resource_manager, game_state);
                        
                        // Find position near HQ
                        let hq_pos = (unit.x, unit.y);
                        let spawn_pos = (hq_pos.0 + 50.0, hq_pos.1 + 50.0);
                        
                        // Create worker and deduct cost
                        game_state.spawn_unit(UnitType::Worker, spawn_pos.0, spawn_pos.1, game_state.current_player_id);
                        game_state.deduct_cost(game_state.current_player_id as usize, &UnitType::Worker);
                    }
                },
                _ => {
                    if draw_button(button_start_x, button_y, button_width, button_height, "Attack") {
                        audio_manager.play_ui_click(resource_manager, game_state);
                        game_state.current_command = Some(Command::Attack { target_id: 0 });
                    }
                    
                    if draw_button(button_start_x + button_width + button_spacing, button_y, button_width, button_height, "Stop") {
                        audio_manager.play_ui_click(resource_manager, game_state);
                        game_state.current_command = Some(Command::Stop);
                    }
                }
            }
            
            // If multiple units selected, show count
            if game_state.selected_units.len() > 1 {
                draw_text(
                    &format!("Selected: {} units", game_state.selected_units.len()),
                    screen_width() - game_state.minimap_rect.w - 200.0,
                    panel_y + 30.0,
                    20.0,
                    WHITE
                );
            }
        }
    }
}

fn draw_button(x: f32, y: f32, width: f32, height: f32, text: &str) -> bool {
    draw_button_colored(x, y, width, height, text, SKYBLUE)
}

fn draw_button_colored(x: f32, y: f32, width: f32, height: f32, text: &str, color: Color) -> bool {
    let rect = Rect::new(x, y, width, height);
    let mouse_pos = mouse_position();
    let hover = rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
    
    // Draw button
    draw_rectangle(x, y, width, height, if hover { color } else { color.darker(0.2) });
    draw_rectangle_lines(x, y, width, height, 2.0, WHITE);
    
    // Draw text
    let font_size = 16.0;
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + (width - text_size.width) / 2.0;
    let text_y = y + (height + text_size.height) / 2.0;
    draw_text(text, text_x, text_y, font_size, WHITE);
    
    hover && is_mouse_button_released(MouseButton::Left)
}

// Helper method for color darkening
trait ColorExt {
    fn darker(&self, amount: f32) -> Self;
}

impl ColorExt for Color {
    fn darker(&self, amount: f32) -> Self {
        Color::new(
            (self.r * (1.0 - amount)).max(0.0),
            (self.g * (1.0 - amount)).max(0.0),
            (self.b * (1.0 - amount)).max(0.0),
            self.a
        )
    }
}
