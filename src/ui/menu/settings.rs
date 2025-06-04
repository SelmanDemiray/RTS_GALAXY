use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use crate::audio::AudioManager;
use crate::resources::ResourceManager;
use super::system::MenuSystem;

pub fn draw(menu: &MenuSystem, game_state: &GameState) {
    let screen_width = screen_width();
    let screen_height = screen_height();
    
    clear_background(Color::new(0.1, 0.1, 0.15, 1.0));
    
    // Draw title
    let title = "Settings";
    let title_size = menu.get_title_font_size();
    let title_width = measure_text(title, None, title_size as u16, 1.0).width;
    draw_text(
        title,
        (screen_width - title_width) / 2.0,
        menu.get_title_y(),
        menu.get_title_font_size(),
        GOLD
    );
    
    // Draw volume controls
    let y_start = screen_height * 0.4;
    let line_height = 40.0;
    
    // Sound Volume
    let sound_text = format!("Sound Volume: {:.0}%", game_state.sound_volume * 100.0);
    draw_text(&sound_text, 100.0, y_start, 24.0, WHITE);
    
    // Music Volume  
    let music_text = format!("Music Volume: {:.0}%", game_state.music_volume * 100.0);
    draw_text(&music_text, 100.0, y_start + line_height, 24.0, WHITE);
    
    // Mute toggles
    let sound_mute_text = format!("Sound Muted: {}", if game_state.sound_muted { "Yes" } else { "No" });
    draw_text(&sound_mute_text, 100.0, y_start + line_height * 2.0, 24.0, WHITE);
    
    let music_mute_text = format!("Music Muted: {}", if game_state.music_muted { "Yes" } else { "No" });
    draw_text(&music_mute_text, 100.0, y_start + line_height * 3.0, 24.0, WHITE);
    
    // Back button
    let back_button_y = screen_height * 0.8;
    menu.draw_button("Back", back_button_y, GameScreen::MainMenu, game_state);
    
    // Instructions
    draw_text("Press ESC to return to main menu", 100.0, screen_height * 0.9, 20.0, GRAY);
}

pub fn draw_settings(
    menu_system: &mut MenuSystem,
    game_state: &mut GameState,
    audio_manager: &AudioManager,
    resource_manager: &ResourceManager
) {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;
    
    // Draw title
    let title = "SETTINGS";
    let title_size = 36.0;
    let title_dims = measure_text(title, None, title_size as u16, 1.0);
    draw_text(title, center_x - title_dims.width / 2.0, center_y - 200.0, title_size, WHITE);
    
    let (mouse_x, mouse_y) = mouse_position();
    
    // Sound volume slider
    draw_text("Sound Volume:", center_x - 150.0, center_y - 100.0, 24.0, WHITE);
    let sound_slider_x = center_x - 100.0;
    let sound_slider_y = center_y - 70.0;
    let slider_width = 200.0;
    let slider_height = 20.0;
    
    // Draw slider background
    draw_rectangle(sound_slider_x, sound_slider_y, slider_width, slider_height, DARKGRAY);
    
    // Draw slider fill
    let sound_fill_width = slider_width * game_state.sound_volume;
    draw_rectangle(sound_slider_x, sound_slider_y, sound_fill_width, slider_height, BLUE);
    
    // Handle sound slider interaction
    if is_mouse_button_down(MouseButton::Left) &&
       mouse_x >= sound_slider_x && mouse_x <= sound_slider_x + slider_width &&
       mouse_y >= sound_slider_y && mouse_y <= sound_slider_y + slider_height {
        game_state.sound_volume = ((mouse_x - sound_slider_x) / slider_width).clamp(0.0, 1.0);
    }
    
    // Music volume slider
    draw_text("Music Volume:", center_x - 150.0, center_y - 20.0, 24.0, WHITE);
    let music_slider_x = center_x - 100.0;
    let music_slider_y = center_y + 10.0;
    
    // Draw slider background
    draw_rectangle(music_slider_x, music_slider_y, slider_width, slider_height, DARKGRAY);
    
    // Draw slider fill
    let music_fill_width = slider_width * game_state.music_volume;
    draw_rectangle(music_slider_x, music_slider_y, music_fill_width, slider_height, BLUE);
    
    // Handle music slider interaction
    if is_mouse_button_down(MouseButton::Left) &&
       mouse_x >= music_slider_x && mouse_x <= music_slider_x + slider_width &&
       mouse_y >= music_slider_y && mouse_y <= music_slider_y + slider_height {
        game_state.music_volume = ((mouse_x - music_slider_x) / slider_width).clamp(0.0, 1.0);
    }
    
    // Mute checkboxes
    let checkbox_size = 20.0;
    
    // Sound mute checkbox
    let sound_checkbox_x = center_x + 120.0;
    let sound_checkbox_y = sound_slider_y;
    draw_rectangle(sound_checkbox_x, sound_checkbox_y, checkbox_size, checkbox_size, WHITE);
    if game_state.sound_muted {
        draw_rectangle(sound_checkbox_x + 2.0, sound_checkbox_y + 2.0, checkbox_size - 4.0, checkbox_size - 4.0, RED);
    }
    draw_text("Mute", sound_checkbox_x + 30.0, sound_checkbox_y + 15.0, 16.0, WHITE);
    
    // Handle sound mute click
    if is_mouse_button_pressed(MouseButton::Left) &&
       mouse_x >= sound_checkbox_x && mouse_x <= sound_checkbox_x + checkbox_size &&
       mouse_y >= sound_checkbox_y && mouse_y <= sound_checkbox_y + checkbox_size {
        game_state.sound_muted = !game_state.sound_muted;
        audio_manager.play_ui_click(resource_manager, game_state);
    }
    
    // Music mute checkbox
    let music_checkbox_x = center_x + 120.0;
    let music_checkbox_y = music_slider_y;
    draw_rectangle(music_checkbox_x, music_checkbox_y, checkbox_size, checkbox_size, WHITE);
    if game_state.music_muted {
        draw_rectangle(music_checkbox_x + 2.0, music_checkbox_y + 2.0, checkbox_size - 4.0, checkbox_size - 4.0, RED);
    }
    draw_text("Mute", music_checkbox_x + 30.0, music_checkbox_y + 15.0, 16.0, WHITE);
    
    // Handle music mute click
    if is_mouse_button_pressed(MouseButton::Left) &&
       mouse_x >= music_checkbox_x && mouse_x <= music_checkbox_x + checkbox_size &&
       mouse_y >= music_checkbox_y && mouse_y <= music_checkbox_y + checkbox_size {
        game_state.music_muted = !game_state.music_muted;
        audio_manager.play_ui_click(resource_manager, game_state);
    }
    
    // Back button
    let button_width = 150.0;
    let button_height = 40.0;
    let back_y = center_y + 150.0;
    let back_hovered = menu_system.is_point_in_rect(mouse_x, mouse_y, center_x - button_width / 2.0, back_y, button_width, button_height);
    if menu_system.draw_button(center_x - button_width / 2.0, back_y, button_width, button_height, "BACK", back_hovered) {
        game_state.current_screen = GameScreen::MainMenu;
        audio_manager.play_ui_click(resource_manager, game_state);
    }
}
