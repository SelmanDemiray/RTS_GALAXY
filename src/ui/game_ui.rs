use egui_macroquad::egui;
use crate::game::{GameState, GameMode};
use crate::network::{NetworkClient, ConnectionStatus};
use macroquad::input::{is_mouse_button_pressed, MouseButton};
use macroquad::prelude::{mouse_position, draw_text_ex, TextParams, screen_width, screen_height, get_frame_time};
use crate::resources::ResourceManager;
use crate::network::NetworkMessage;

// Global static to track the collapse timer
static mut PANEL_COLLAPSE_TIMER: f32 = 0.0;
// How long to keep the panel open after mouse leaves (in seconds)
const PANEL_COLLAPSE_DELAY: f32 = 0.5;
// Initial panel state - start collapsed
static mut PANEL_EXPANDED: bool = false;
static mut PANEL_HAS_FOCUS: bool = false;
// Previous screen dimensions to detect resize
static mut PREV_SCREEN_WIDTH: f32 = 0.0;
static mut PREV_SCREEN_HEIGHT: f32 = 0.0;

pub fn draw_ui(game_state: &mut GameState, network_client: &mut NetworkClient, resource_manager: &ResourceManager) {
    // Process mouse input for game interaction
    if is_mouse_button_pressed(MouseButton::Left) {
        let (x, y) = mouse_position();
        game_state.select_unit_at(x, y);
    }
    
    if is_mouse_button_pressed(MouseButton::Right) {
        if !game_state.selected_units.is_empty() {
            let (x, y) = mouse_position();
            game_state.move_selected_unit(x, y);
            
            // Send movement command to network only if online
            if game_state.game_mode == GameMode::Online && network_client.is_connected() {
                if let Some(unit_id) = game_state.selected_units.first() {
                    let action = NetworkMessage::PlayerAction {
                        unit_id: *unit_id,
                        target_x: x,
                        target_y: y,
                    };
                    
                    let _ = network_client.send(&action);
                }
            }
        }
    }
    
    // Check for screen resize
    unsafe {
        let current_width = screen_width();
        let current_height = screen_height();
        
        if PREV_SCREEN_WIDTH != current_width || PREV_SCREEN_HEIGHT != current_height {
            // Screen has been resized, update minimap position
            if PREV_SCREEN_WIDTH > 0.0 {  // Skip first frame
                game_state.minimap_rect.x = current_width - 210.0;
                game_state.minimap_rect.y = current_height - 210.0;
            }
            
            PREV_SCREEN_WIDTH = current_width;
            PREV_SCREEN_HEIGHT = current_height;
        }
    }
    
    egui_macroquad::ui(|egui_ctx| {
        // Track if mouse is on the left edge of the screen
        let mouse_pos = egui_ctx.input(|i| i.pointer.hover_pos()).unwrap_or(egui::Pos2::new(0.0, 0.0));
        let hover_zone_width = 15.0; // Reduced width of hover zone for more precise control
        let is_hovering_edge = mouse_pos.x < hover_zone_width;
        
        unsafe {
            // Get current frame time for timer
            let delta_time = get_frame_time();
            
            // Auto-expand on hover, stay expanded if it has focus
            if is_hovering_edge || PANEL_HAS_FOCUS {
                PANEL_EXPANDED = true;
                // Reset collapse timer when mouse is in hover zone or panel has focus
                PANEL_COLLAPSE_TIMER = 0.0;
            } else {
                // Start timer for collapse
                PANEL_COLLAPSE_TIMER += delta_time;
                // Only collapse if timer has exceeded the delay
                if PANEL_COLLAPSE_TIMER > PANEL_COLLAPSE_DELAY {
                    PANEL_EXPANDED = false;
                }
            }
            
            if PANEL_EXPANDED {
                // Only show the panel when expanded
                egui::SidePanel::left("game_controls_panel")
                    .resizable(false)
                    .default_width(250.0)
                    .show(egui_ctx, |ui| {
                        // Update focus state based on interaction
                        PANEL_HAS_FOCUS = ui.ui_contains_pointer();
                        
                        ui.heading("Game Controls");
                        ui.separator();
                        
                        // Game Mode Section
                        ui.heading("Game Mode");
                        
                        let current_mode = if game_state.game_mode == GameMode::Online { "Online" } else { "Offline" };
                        ui.label(format!("Current Mode: {}", current_mode));
                        
                        ui.horizontal(|ui| {
                            if ui.button("Play Offline").clicked() {
                                game_state.set_game_mode(GameMode::Offline);
                                network_client.disconnect();
                            }
                            
                            if game_state.game_mode == GameMode::Offline {
                                if ui.button("Connect to World").clicked() {
                                    game_state.world_address = "127.0.0.1:8080".to_string();
                                    game_state.game_mode = GameMode::Online;
                                }
                            }
                        });
                        
                        // Connection section - only show when in online mode
                        if game_state.game_mode == GameMode::Online {
                            ui.separator();
                            ui.heading("Network Connection");
                            
                            let connection_status = match &network_client.status {
                                ConnectionStatus::Connected => "Connected".to_string(),
                                ConnectionStatus::Connecting => "Connecting...".to_string(),
                                ConnectionStatus::Disconnected => "Disconnected".to_string(),
                                ConnectionStatus::Failed(error) => format!("Failed: {}", error),
                            };
                            
                            ui.label(format!("Status: {}", connection_status));
                            
                            ui.horizontal(|ui| {
                                let mut address = game_state.world_address.clone();
                                ui.label("Server:");
                                if ui.text_edit_singleline(&mut address).changed() {
                                    game_state.world_address = address;
                                }
                                
                                if !network_client.is_connected() {
                                    if ui.button("Connect").clicked() {
                                        if game_state.world_address.is_empty() {
                                            game_state.world_address = "127.0.0.1:8080".to_string();
                                        }
                                        let _ = network_client.connect(&game_state.world_address);
                                    }
                                } else {
                                    if ui.button("Disconnect").clicked() {
                                        network_client.disconnect();
                                    }
                                }
                            });
                            
                            if let Some(error) = &network_client.last_error {
                                ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                            }
                        }
                        
                        ui.separator();
                        
                        ui.heading("Selected Unit");
                        if let Some(&unit_id) = game_state.selected_units.first() {
                            if let Some(unit) = game_state.units.iter().find(|u| u.id == unit_id) {
                                ui.label(format!("Unit ID: {}", unit.id));
                                ui.label(format!("Position: ({:.1}, {:.1})", unit.x, unit.y));
                                ui.label(format!("Health: {}", unit.health));
                            }
                        } else {
                            ui.label("No unit selected");
                        }
                        
                        ui.separator();
                        
                        ui.heading("Chat Messages");
                        egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                            for message in &game_state.messages {
                                ui.label(message);
                            }
                        });
                        
                        // Only allow chat if in online mode
                        if game_state.game_mode == GameMode::Online && network_client.is_connected() {
                            let mut chat_input = String::new();
                            ui.horizontal(|ui| {
                                let response = ui.text_edit_singleline(&mut chat_input);
                                
                                if ui.button("Send").clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                    if !chat_input.is_empty() {
                                        let message = NetworkMessage::ChatMessage(chat_input.clone());
                                        let _ = network_client.send(&message);
                                        game_state.messages.push(format!("You: {}", chat_input));
                                        chat_input.clear();
                                    }
                                }
                            });
                        } else {
                            ui.label("Chat available when connected to a world");
                        }
                        
                        // Add collapse button option
                        ui.separator();
                        if ui.button("← Collapse Menu").clicked() {
                            PANEL_EXPANDED = false;
                            PANEL_COLLAPSE_TIMER = PANEL_COLLAPSE_DELAY + 1.0; // Force immediate collapse
                        }
                    });
            } else {
                // When collapsed, just show a small button to expand at the edge
                egui::Area::new("expand_button")
                    .fixed_pos(egui::pos2(0.0, 50.0))
                    .show(egui_ctx, |ui| {
                        if ui.button("►").clicked() {
                            PANEL_EXPANDED = true;
                        }
                    });
            }
        }
    });
    
    // Render any additional UI elements using the resource manager
    if let Some(font) = resource_manager.get_font("default") {
        let params = TextParams {
            font: *font,
            font_size: 20,
            ..Default::default()
        };
        
        // Draw game version info in the corner
        draw_text_ex("Fantasy RTS v0.1", screen_width() - 150.0, 20.0, params);
    }
    
    egui_macroquad::draw();
}
