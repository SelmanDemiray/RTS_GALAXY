mod game;
mod network;
mod ui;
mod ai;
mod entity;
mod resources;
mod audio;

use macroquad::prelude::*;
use crate::game::{GameMode, GameScreen, GameState};
use crate::ai::AIController;
use crate::ui::menu::MenuSystem;
use crate::resources::ResourceManager;
use crate::network::NetworkClient;
use crate::audio::AudioManager;

#[macroquad::main("Fantasy RTS")]
async fn main() {
    let mut game_state = GameState::new();
    let mut network_client = NetworkClient::new();
    let mut ai_controller = AIController::new();
    let mut menu_system = MenuSystem::new();
    let mut resource_manager = ResourceManager::new();
    let mut audio_manager = AudioManager::new();
    
    // Load game resources
    resource_manager.load_resources().await;
    menu_system.initialize(&resource_manager);
    
    // Start at main menu
    game_state.current_screen = GameScreen::MainMenu;
    
    // Track previous screen dimensions to detect resizes
    let mut prev_width = screen_width();
    let mut prev_height = screen_height();
    
    // Wait for resources to load completely
    while !resource_manager.is_loading_complete() {
        clear_background(Color::new(0.1, 0.1, 0.2, 1.0));
        
        // Display loading progress
        let progress = resource_manager.get_loading_progress();
        let bar_width = screen_width() * 0.7;
        let bar_height = 30.0;
        let bar_x = (screen_width() - bar_width) / 2.0;
        let bar_y = screen_height() / 2.0;
        
        // Draw loading bar background
        draw_rectangle(bar_x, bar_y, bar_width, bar_height, GRAY);
        // Draw loading progress
        draw_rectangle(bar_x, bar_y, bar_width * progress, bar_height, GREEN);
        // Draw percentage text
        let text = format!("Loading: {:.0}%", progress * 100.0);
        let font_size = 24.0;
        let text_dims = measure_text(&text, None, font_size as u16, 1.0);
        draw_text(
            &text,
            (screen_width() - text_dims.width) / 2.0,
            bar_y - 20.0,
            font_size,
            WHITE
        );
        
        next_frame().await;
    }
    
    // Start background music once loaded
    audio_manager.play_music("main_theme", &resource_manager, &game_state);
    
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
        
        // Update audio volumes if they've changed
        audio_manager.update_volumes(&resource_manager, &game_state);
        
        clear_background(Color::new(0.1, 0.1, 0.2, 1.0));
        
        match game_state.current_screen {
            GameScreen::MainMenu | GameScreen::Settings | GameScreen::Credits => {
                menu_system.update(&mut game_state, &resource_manager, &mut audio_manager);
                menu_system.draw(&mut game_state, &resource_manager);
            },
            GameScreen::Playing => {
                // Play game music if different from menu music
                if game_state.current_screen == GameScreen::Playing && 
                   audio_manager.get_current_music() != Some("gameplay") {
                    audio_manager.play_music("gameplay", &resource_manager, &game_state);
                }
                
                // Game update logic
                let previous_selected = game_state.selected_units.clone();
                game_state.update();
                
                // Play selection sound if selection changed
                if previous_selected != game_state.selected_units && !game_state.selected_units.is_empty() {
                    audio_manager.play_selection_sound(&resource_manager, &game_state);
                }
                
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
                ui::game_ui::draw_ui(&mut game_state, &mut network_client, &resource_manager, &mut audio_manager);
            },
        }
        
        next_frame().await;
    }
}
