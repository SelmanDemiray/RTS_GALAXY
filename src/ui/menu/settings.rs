use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
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
