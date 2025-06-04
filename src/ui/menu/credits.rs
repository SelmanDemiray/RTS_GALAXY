use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use crate::audio::AudioManager;
use crate::resources::ResourceManager;
use super::system::MenuSystem;

pub fn draw_credits(
    menu_system: &mut MenuSystem,
    game_state: &mut GameState,
    audio_manager: &AudioManager,
    resource_manager: &ResourceManager
) {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;
    
    // Draw title
    let title = "CREDITS";
    let title_size = 36.0;
    let title_dims = measure_text(title, None, title_size as u16, 1.0);
    draw_text(title, center_x - title_dims.width / 2.0, center_y - 200.0, title_size, WHITE);
    
    // Credits content
    let credits_text = vec![
        "Galaxy RTS",
        "",
        "Game Development:",
        "  - AI Assistant (GitHub Copilot)",
        "",
        "Engine:",
        "  - Macroquad (Rust Game Engine)",
        "",
        "Libraries:",
        "  - Serde (Serialization)",
        "  - Tokio (Async Runtime)",
        "",
        "Special Thanks:",
        "  - Rust Community",
        "  - Open Source Contributors",
        "",
        "This is a demonstration RTS game",
        "featuring advanced zoom mechanics",
        "and galactic-scale gameplay.",
    ];
    
    let line_height = 24.0;
    let start_y = center_y - 100.0;
    
    for (i, line) in credits_text.iter().enumerate() {
        let y_pos = start_y + (i as f32) * line_height;
        let text_size = if line.is_empty() { 16.0 } else if line.starts_with("  ") { 18.0 } else { 20.0 };
        let text_dims = measure_text(line, None, text_size as u16, 1.0);
        draw_text(line, center_x - text_dims.width / 2.0, y_pos, text_size, WHITE);
    }
    
    // Back button
    let (mouse_x, mouse_y) = mouse_position();
    let button_width = 150.0;
    let button_height = 40.0;
    let back_y = center_y + 200.0;
    let back_hovered = menu_system.is_point_in_rect(mouse_x, mouse_y, center_x - button_width / 2.0, back_y, button_width, button_height);
    if menu_system.draw_button(center_x - button_width / 2.0, back_y, button_width, button_height, "BACK", back_hovered) {
        game_state.current_screen = GameScreen::MainMenu;
        audio_manager.play_ui_click(resource_manager, game_state);
    }
}
