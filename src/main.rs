mod game;
mod network;
mod ui;

use macroquad::prelude::*;

#[macroquad::main("RTS Game")]
async fn main() {
    let mut game_state = game::GameState::new();
    let mut network_client = network::NetworkClient::new();
    
    // Try to connect, but continue even if it fails
    // No longer async!
    let _ = network_client.connect("127.0.0.1:8080");
    
    loop {
        clear_background(WHITE);
        
        // Game update logic
        game_state.update();
        
        // Handle networking - no longer async!
        if let Some(msg) = network_client.receive() {
            game_state.handle_network_message(msg);
        }
        
        // Draw game elements
        game_state.draw();
        
        // Draw UI
        ui::draw_ui(&mut game_state, &mut network_client);
        
        next_frame().await;
    }
}
