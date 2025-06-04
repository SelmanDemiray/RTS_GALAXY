use macroquad::prelude::*;

mod game;
mod entity;
mod ui;
mod resources;
mod network;
mod audio;
mod ai;

use game::{GameState, GameScreen};
use ui::menu::MenuSystem;
use ui::game_ui;
use resources::ResourceManager;
use audio::AudioManager;
use network::NetworkClient;
use ai::AIController;

#[macroquad::main("Galaxy RTS")]
async fn main() {
    let mut game_state = GameState::new();
    let mut menu_system = MenuSystem::new();
    let mut resource_manager = ResourceManager::new();
    let mut audio_manager = AudioManager::new();
    let mut network_client = NetworkClient::new();
    let mut ai_controller = AIController::new();

    // Load resources
    resource_manager.load_resources().await;
    menu_system.initialize(&resource_manager);

    loop {
        // Handle quit request
        if game_state.should_quit {
            break;
        }

        // Update based on current screen
        match game_state.current_screen {
            GameScreen::MainMenu | GameScreen::Settings | GameScreen::Credits => {
                menu_system.update(&mut game_state, &resource_manager, &mut audio_manager);
                menu_system.draw(&game_state, &resource_manager);
            }
            GameScreen::Playing => {
                game_state.update();
                ai_controller.update(&mut game_state);

                // Draw game
                clear_background(BLACK);
                game::rendering::draw_game(&game_state, &resource_manager);
                game_ui::draw_ui(&mut game_state, &mut network_client, &resource_manager, &mut audio_manager);
            }
        }

        next_frame().await;
    }
}
