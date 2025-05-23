mod game;
mod network;
mod ui;
mod ai;
mod entity;
mod resources;

use macroquad::prelude::*;
use crate::game::{GameMode, GameScreen, GameState};
use crate::ai::AIController;
use crate::ui::menu::MenuSystem;
use crate::resources::ResourceManager;
use crate::network::NetworkClient;

#[macroquad::main("Fantasy RTS")]
async fn main() {
    let mut game_state = GameState::new();
    let mut network_client = NetworkClient::new();
    let mut ai_controller = AIController::new();
    let mut menu_system = MenuSystem::new();
    let mut resource_manager = ResourceManager::new();
    
    // Load game resources
    resource_manager.load_resources().await;
    menu_system.initialize(&resource_manager);
    
    // Start at main menu
    game_state.current_screen = GameScreen::MainMenu;
    
    // Track previous screen dimensions to detect resizes
    let mut prev_width = screen_width();
    let mut prev_height = screen_height();
    
    loop {
        // Check for screen resize
        let current_width = screen_width();
        let current_height = screen_height();
        
        if prev_width != current_width || prev_height != current_height {
            // Screen has been resized
            game_state.handle_screen_resize();
            prev_width = current_width;
            prev_height = current_height;
        }
        
        clear_background(Color::new(0.1, 0.1, 0.2, 1.0));
        
        match game_state.current_screen {
            GameScreen::MainMenu | GameScreen::Settings | GameScreen::Credits => {
                menu_system.update(&mut game_state);
                menu_system.draw(&mut game_state, &resource_manager);
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
                ui::game_ui::draw_ui(&mut game_state, &mut network_client, &resource_manager);
            },
        }
        
        next_frame().await;
    }
}
