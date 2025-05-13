use egui_macroquad::egui;
use crate::game::{GameState, NetworkMessage};
use crate::network::NetworkClient;
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
            
            // Send movement command to network - no longer async
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
    
    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("Game Controls")
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.heading("RTS Game");
                ui.separator();
                
                let connection_status = if network_client.is_connected() {
                    "Connected"
                } else {
                    "Disconnected"
                };
                
                ui.label(format!("Network: {}", connection_status));
                
                if ui.button("Connect").clicked() {
                    let _ = network_client.connect("127.0.0.1:8080");
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
            });
    });
    
    egui_macroquad::draw();
}
