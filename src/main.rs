mod game;
mod network;
mod ui;
mod ai;
mod menu;
mod resources;

use macroquad::prelude::*;
use crate::game::{GameMode, GameScreen};
use crate::ai::AIController;
use crate::menu::MenuSystem;
use crate::resources::ResourceManager;

#[macroquad::main("Fantasy RTS")]
async fn main() {
    let mut game_state = game::GameState::new();
    let mut network_client = network::NetworkClient::new();
    let mut ai_controller = AIController::new();
    let mut menu_system = MenuSystem::new();
    let mut resource_manager = ResourceManager::new();
    
    // Load game resources
    resource_manager.load_resources().await;
    menu_system.initialize(&resource_manager);
    
    // Start at main menu
    game_state.current_screen = GameScreen::MainMenu;
    
    loop {
        clear_background(Color::new(0.1, 0.1, 0.2, 1.0));
        
        match game_state.current_screen {
            GameScreen::MainMenu => {
                menu_system.update(&mut game_state);
                menu_system.draw(&resource_manager);
            },
            GameScreen::Playing => {
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
                game_state.draw(&resource_manager);
                
                // Draw UI
                ui::draw_ui(&mut game_state, &mut network_client, &resource_manager);
            },
            GameScreen::Settings => {
                menu_system.draw_settings(&mut game_state);
            },
            GameScreen::Credits => {
                menu_system.draw_credits();
                if is_key_pressed(KeyCode::Escape) {
                    game_state.current_screen = GameScreen::MainMenu;
                }
            },
        }
        
        next_frame().await;
    }
}
