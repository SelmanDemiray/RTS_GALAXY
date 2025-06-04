mod game;
mod network;
mod ui;
mod ai;
mod entity;
mod resources;
mod audio;

use macroquad::prelude::*;
use crate::game::{GameMode, GameState};
use crate::game::screens::GameScreen;
use crate::ai::AIController;
use crate::ui::menu::system::MenuSystem;
use crate::resources::ResourceManager;
use crate::network::NetworkClient;
use crate::audio::AudioManager;
use std::panic::{self, AssertUnwindSafe};

#[macroquad::main("Fantasy RTS")]
async fn main() {
    // Initialize 3D camera
    let mut camera = Camera3D {
        position: vec3(0.0, 10.0, 10.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        fovy: 45.0,
        projection: Projection::Perspective,
        ..Default::default()
    };

    let mut game_state = GameState::new();
    let mut network_client = NetworkClient::new();
    let mut ai_controller = AIController::new();
    let mut menu_system = MenuSystem::new();
    let mut resource_manager = ResourceManager::new();
    let mut audio_manager = AudioManager::new();
    
    // Load game resources
    resource_manager.load_resources().await;
    
    // Initialize menu system only after resources are loaded
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
    
    loop {
        // Check if game should quit
        if game_state.should_quit {
            break;
        }
        
        // Wrap the frame processing in a catch_unwind to prevent crashes
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            // Check for screen resize
            let current_width = screen_width();
            let current_height = screen_height();
            
            if prev_width != current_width || prev_height != current_height {
                // Screen has been resized
                game_state.handle_screen_resize();
                prev_width = current_width;
                prev_height = current_height;
                
                // Log the resize event
                println!("Screen resized to: {}x{}", current_width, current_height);
            }
            
            // Update audio volumes if they've changed
            audio_manager.update_volumes(&resource_manager, &game_state);
            
            clear_background(Color::new(0.1, 0.1, 0.2, 1.0));
            
            match game_state.current_screen {
                GameScreen::MainMenu | GameScreen::Settings | GameScreen::Credits => {
                    menu_system.update(&mut game_state, &resource_manager, &mut audio_manager);
                    menu_system.draw(&game_state, &resource_manager);
                },
                GameScreen::Quit => {
                    // This will be handled by the quit check at the start of the loop
                    game_state.request_quit();
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
                    
                    // Update 3D camera based on game state and zoom system
                    let zoom_scale = game_state.zoom_system.get_current_scale() as f32;
                    let camera_height = 10.0 + (zoom_scale / 100.0).clamp(1.0, 1000.0);
                    let camera_distance = (zoom_scale / 50.0).clamp(5.0, 500.0);
                    
                    camera.position = vec3(
                        game_state.camera_x, 
                        camera_height, 
                        game_state.camera_y + camera_distance
                    );
                    camera.target = vec3(game_state.camera_x, 0.0, game_state.camera_y);
                    
                    // Begin 3D mode
                    set_camera(&camera);
                    
                    // Draw 3D terrain and entities with LOD based on zoom level
                    let grid_size = (zoom_scale / 10.0).clamp(1.0, 100.0) as i32;
                    let grid_spacing = (zoom_scale / 100.0).clamp(0.1, 50.0);
                    draw_grid(grid_size, grid_spacing, BLACK, GRAY);
                    
                    // LOD system: only draw detailed models at appropriate zoom levels
                    if game_state.zoom_system.current_level <= 15 {
                        // Draw 3D models for game entities at close zoom levels
                        for unit in &game_state.units {
                            if let Some(model) = resource_manager.get_model(&unit.unit_type.to_string().to_lowercase()) {
                                let position = vec3(unit.x, 0.0, unit.y);
                                let rotation = vec3(0.0, 0.0, 0.0);
                                let scale = (1.0 / zoom_scale as f32 * 1000.0).clamp(0.1, 2.0);
                                
                                // Only draw if unit would be visible at this scale
                                if scale > 0.01 {
                                    model.draw(position, rotation, scale);
                                }
                                
                                // Draw selection indicator if unit is selected
                                if game_state.selected_units.contains(&unit.id) {
                                    // Draw a circle or highlight for selected units
                                    draw_circle_3d(position, scale * 2.0, None, GREEN);
                                }
                            }
                        }
                    } else {
                        // At higher zoom levels, draw simplified representations
                        for unit in &game_state.units {
                            let position = vec3(unit.x, 0.0, unit.y);
                            let size = (zoom_scale / 1000.0).clamp(1.0, 100.0);
                            let color = if unit.player_id == game_state.current_player_id { BLUE } else { RED };
                            draw_cube(position, vec3(size, size * 0.5, size), None, color);
                        }
                    }
                    
                    // End 3D mode and switch back to 2D for UI
                    set_default_camera();
                    
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
                    
                    // Draw 2D UI elements
                    ui::game_ui::draw_ui(&mut game_state, &mut network_client, &resource_manager, &mut audio_manager);
                },
            }
        }));
        
        // If there was a panic during frame processing, log it but don't crash
        if let Err(e) = result {
            if let Some(err_msg) = e.downcast_ref::<String>() {
                eprintln!("Recovered from error: {}", err_msg);
            } else {
                eprintln!("Recovered from unknown error");
            }
        }
        
        next_frame().await;
    }
}
