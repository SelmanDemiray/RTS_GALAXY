use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use crate::audio::AudioManager;
use crate::resources::ResourceManager;
use super::system::MenuSystem;

pub fn draw_main_menu(
    menu_system: &mut MenuSystem,
    game_state: &mut GameState,
    audio_manager: &AudioManager,
    resource_manager: &ResourceManager
) {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;
    
    // Draw title
    let title = "GALAXY RTS";
    let title_size = 48.0;
    let title_dims = measure_text(title, None, title_size as u16, 1.0);
    draw_text(title, center_x - title_dims.width / 2.0, center_y - 150.0, title_size, WHITE);
    
    // Button dimensions
    let button_width = 200.0;
    let button_height = 50.0;
    let button_spacing = 60.0;
    
    let (mouse_x, mouse_y) = mouse_position();
    
    // Play button
    let play_y = center_y - 50.0;
    let play_hovered = menu_system.is_point_in_rect(mouse_x, mouse_y, center_x - button_width / 2.0, play_y, button_width, button_height);
    if menu_system.draw_button(center_x - button_width / 2.0, play_y, button_width, button_height, "PLAY", play_hovered) {
        game_state.current_screen = GameScreen::Playing;
        audio_manager.play_ui_click(resource_manager, game_state);
    }
    
    // Settings button
    let settings_y = center_y + 10.0;
    let settings_hovered = menu_system.is_point_in_rect(mouse_x, mouse_y, center_x - button_width / 2.0, settings_y, button_width, button_height);
    if menu_system.draw_button(center_x - button_width / 2.0, settings_y, button_width, button_height, "SETTINGS", settings_hovered) {
        game_state.current_screen = GameScreen::Settings;
        audio_manager.play_ui_click(resource_manager, game_state);
    }
    
    // Credits button
    let credits_y = center_y + 70.0;
    let credits_hovered = menu_system.is_point_in_rect(mouse_x, mouse_y, center_x - button_width / 2.0, credits_y, button_width, button_height);
    if menu_system.draw_button(center_x - button_width / 2.0, credits_y, button_width, button_height, "CREDITS", credits_hovered) {
        game_state.current_screen = GameScreen::Credits;
        audio_manager.play_ui_click(resource_manager, game_state);
    }
    
    // Quit button
    let quit_y = center_y + 130.0;
    let quit_hovered = menu_system.is_point_in_rect(mouse_x, mouse_y, center_x - button_width / 2.0, quit_y, button_width, button_height);
    if menu_system.draw_button(center_x - button_width / 2.0, quit_y, button_width, button_height, "QUIT", quit_hovered) {
        game_state.request_quit();
        audio_manager.play_ui_click(resource_manager, game_state);
    }
}
