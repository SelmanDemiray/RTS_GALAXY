mod game;
mod network;
mod ui;
mod ai;

use macroquad::prelude::*;
use crate::game::GameMode;
use crate::ai::AIController;

#[macroquad::main("RTS Game")]
async fn main() {
    let mut game_state = game::GameState::new();
    let mut network_client = network::NetworkClient::new();
    let mut ai_controller = AIController::new();
    
    // Start in offline mode by default, don't try to connect automatically
    game_state.set_game_mode(GameMode::Offline);
    
    loop {
        clear_background(WHITE);
        
        // Game update logic
        game_state.update();
        
        // Process AI in offline mode
        if game_state.game_mode == GameMode::Offline {
            ai_controller.update(&mut game_state);
        }
        
        // Handle networking only if in online mode
        if game_state.game_mode == GameMode::Online && network_client.is_connected() {
            if let Some(msg) = network_client.receive() {
                game_state.handle_network_message(msg);
            }
        }
        
        // Draw game elements
        game_state.draw();
        
        // Draw UI
        ui::draw_ui(&mut game_state, &mut network_client);
        
        next_frame().await;
    }
}
