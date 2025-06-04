use macroquad::prelude::*;

mod game;
mod entity;
mod resources;
mod network;
mod ui;
mod audio;
mod ai;

use game::{GameState, GameScreen};
use resources::ResourceManager;
use audio::AudioManager;
use ai::AIController;
use ui::menu::MenuSystem;

#[macroquad::main("Galaxy RTS")]
async fn main() {
    // Initialize game systems
    let mut game_state = GameState::new();
    let mut resource_manager = ResourceManager::new();
    let mut audio_manager = AudioManager::new();
    let mut ai_controller = AIController::new();
    let mut menu_system = MenuSystem::new();
    
    // Load resources
    resource_manager.load_resources().await;
    
    loop {
        // Handle different game screens
        match game_state.current_screen {
            GameScreen::MainMenu => {
                handle_main_menu(&mut game_state, &mut menu_system, &audio_manager, &resource_manager);
            },
            GameScreen::Playing => {
                handle_gameplay(&mut game_state, &mut ai_controller, &audio_manager, &resource_manager);
            },
            GameScreen::Settings => {
                handle_settings(&mut game_state, &mut menu_system, &audio_manager, &resource_manager);
            },
            GameScreen::Credits => {
                handle_credits(&mut game_state, &mut menu_system, &audio_manager, &resource_manager);
            },
        }
        
        // Check for quit
        if game_state.should_quit {
            break;
        }
        
        next_frame().await;
    }
}

fn handle_main_menu(
    game_state: &mut GameState, 
    menu_system: &mut MenuSystem,
    audio_manager: &AudioManager,
    resource_manager: &ResourceManager
) {
    clear_background(BLACK);
    
    // Draw main menu
    menu_system.draw_main_menu(game_state, audio_manager, resource_manager);
}

fn handle_gameplay(
    game_state: &mut GameState,
    ai_controller: &mut AIController,
    audio_manager: &AudioManager,
    resource_manager: &ResourceManager
) {
    // Update game state
    game_state.update();
    
    // Update AI
    ai_controller.update(game_state);
    
    // Handle input
    handle_game_input(game_state, audio_manager, resource_manager);
    
    // Render game
    clear_background(DARKGREEN);
    game::rendering::draw_game(game_state, resource_manager);
    
    // Draw UI overlay
    draw_ui_overlay(game_state);
}

fn handle_settings(
    game_state: &mut GameState,
    menu_system: &mut MenuSystem,
    audio_manager: &AudioManager,
    resource_manager: &ResourceManager
) {
    clear_background(BLACK);
    menu_system.draw_settings(game_state, audio_manager, resource_manager);
}

fn handle_credits(
    game_state: &mut GameState,
    menu_system: &mut MenuSystem,
    audio_manager: &AudioManager,
    resource_manager: &ResourceManager
) {
    clear_background(BLACK);
    menu_system.draw_credits(game_state, audio_manager, resource_manager);
}

fn handle_game_input(
    game_state: &mut GameState,
    audio_manager: &AudioManager,
    resource_manager: &ResourceManager
) {
    // Handle escape to menu
    if is_key_pressed(KeyCode::Escape) {
        game_state.current_screen = GameScreen::MainMenu;
        audio_manager.play_ui_click(resource_manager, game_state);
    }
    
    // Handle unit selection
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let world_x = mouse_x + game_state.camera_x - screen_width() / 2.0;
        let world_y = mouse_y + game_state.camera_y - screen_height() / 2.0;
        
        select_unit_at(game_state, world_x, world_y, audio_manager, resource_manager);
    }
    
    // Handle unit movement
    if is_mouse_button_pressed(MouseButton::Right) && !game_state.selected_units.is_empty() {
        let (mouse_x, mouse_y) = mouse_position();
        let world_x = mouse_x + game_state.camera_x - screen_width() / 2.0;
        let world_y = mouse_y + game_state.camera_y - screen_height() / 2.0;
        
        move_selected_units(game_state, world_x, world_y, audio_manager, resource_manager);
    }
}

fn select_unit_at(
    game_state: &mut GameState,
    world_x: f32,
    world_y: f32,
    audio_manager: &AudioManager,
    resource_manager: &ResourceManager
) {
    game_state.selected_units.clear();
    
    for unit in &game_state.units {
        if unit.player_id == game_state.current_player_id {
            let distance = ((unit.x - world_x).powi(2) + (unit.y - world_y).powi(2)).sqrt();
            if distance < 30.0 {
                game_state.selected_units.push(unit.id);
                audio_manager.play_selection_sound(resource_manager, game_state);
                break;
            }
        }
    }
}

fn move_selected_units(
    game_state: &mut GameState,
    world_x: f32,
    world_y: f32,
    _audio_manager: &AudioManager,
    _resource_manager: &ResourceManager
) {
    for unit_id in &game_state.selected_units {
        for unit in &mut game_state.units {
            if unit.id == *unit_id {
                unit.target_x = Some(world_x);
                unit.target_y = Some(world_y);
            }
        }
    }
}

fn draw_ui_overlay(game_state: &GameState) {
    // Draw zoom level indicator
    let zoom_text = game_state.zoom_system.get_zoom_label();
    draw_text(&zoom_text, 10.0, screen_height() - 40.0, 20.0, WHITE);
    
    let zoom_desc = game_state.zoom_system.get_zoom_description();
    draw_text(&zoom_desc, 10.0, screen_height() - 20.0, 16.0, LIGHTGRAY);
    
    // Draw controls help
    draw_text("Controls: WASD/Arrows - Move Camera", 10.0, 90.0, 16.0, WHITE);
    draw_text("+/- or Mouse Wheel - Zoom", 10.0, 110.0, 16.0, WHITE);
    draw_text("H or Home - Return to HQ", 10.0, 130.0, 16.0, WHITE);
    draw_text("Left Click - Select Unit", 10.0, 150.0, 16.0, WHITE);
    draw_text("Right Click - Move Unit", 10.0, 170.0, 16.0, WHITE);
    draw_text("ESC - Main Menu", 10.0, 190.0, 16.0, WHITE);
}
