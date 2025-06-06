use macroquad::prelude::*;
use rts_galaxy::game::GameState;
use rts_galaxy::resources::ResourceManager;
use rts_galaxy::audio::AudioManager;
use rts_galaxy::ui::MenuSystem;
use rts_galaxy::ai::AIController;

#[macroquad::main("RTS Galaxy")]
async fn main() {
    let mut game_state = GameState::new();
    let mut resource_manager = ResourceManager::new();
    let mut audio_manager = AudioManager::new();
    let mut menu_system = MenuSystem::new();
    let mut ai_controller = AIController::new();

    // Load initial assets
    resource_manager.load_resources().await;

    loop {
        // Update game state
        game_state.update();
        ai_controller.update(&mut game_state);
        menu_system.update();

        // Clear screen
        clear_background(BLACK);

        // Render game
        game_state.render(&resource_manager);
        menu_system.render();

        // Handle input
        game_state.handle_input();

        // Exit if requested
        if !game_state.is_running {
            break;
        }

        next_frame().await
    }
}
