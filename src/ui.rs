use egui_macroquad::egui;
use crate::game::{GameState, NetworkMessage, GameMode};
use crate::network::{NetworkClient, ConnectionStatus};
use macroquad::input::{is_mouse_button_pressed, MouseButton};
use macroquad::prelude::mouse_position;

pub fn draw_ui(game_state: &mut GameState, network_client: &mut NetworkClient) {
    // Process mouse input for game interaction
    if is_mouse_button_pressed(MouseButton::Left) {
        let (x, y) = mouse_position();
        game_state.select_unit_at(x, y);
    }
    
    if is_mouse_button_pressed(MouseButton::Right) {
        if game_state.selected_unit.is_some() {
            let (x, y) = mouse_position();
            game_state.move_selected_unit(x, y);
            
            // Send movement command to network only if online
            if game_state.game_mode == GameMode::Online && network_client.is_connected() {
                if let Some(unit_id) = game_state.selected_unit {
                    let action = NetworkMessage::PlayerAction {
                        unit_id,
                        target_x: x,
                        target_y: y,
                    };
                    
                    let _ = network_client.send(&action);
                }
            }
        }
    }
    
    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("Game Controls")
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.heading("RTS Game");
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
                if let Some(unit_id) = game_state.selected_unit {
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
                
                ui.separator();
                
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
            });
    });
    
    egui_macroquad::draw();
}
