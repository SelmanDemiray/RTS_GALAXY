use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use super::system::MenuSystem;

pub fn draw(menu: &MenuSystem, game_state: &GameState) {
    let screen_center_x = screen_width() / 2.0;

    // Draw title
    let title = "Fantasy RTS";
    let title_size = measure_text(title, None, menu.get_title_font_size() as u16, 1.0);
    draw_text(
        title,
        screen_center_x - title_size.width / 2.0,
        menu.get_title_y(),
        menu.get_title_font_size(),
        GOLD
    );

    // Draw buttons
    menu.draw_button("Play Game", menu.get_first_button_y(), GameScreen::Playing, game_state);
    menu.draw_button("Settings", menu.get_first_button_y() + menu.get_button_spacing(), GameScreen::Settings, game_state);
    menu.draw_button("Credits", menu.get_first_button_y() + menu.get_button_spacing() * 2.0, GameScreen::Credits, game_state);
    
    // Draw quit button manually since it doesn't map to a screen
    let quit_text = "Quit";
    let quit_y = menu.get_first_button_y() + menu.get_button_spacing() * 3.0;
    let button_width = measure_text(quit_text, None, 32, 1.0).width;
    let x = (screen_width() - button_width) / 2.0;
    let color = if menu.selected_button == 3 { YELLOW } else { WHITE };
    draw_text(quit_text, x, quit_y, 32.0, color);
    if menu.selected_button == 3 {
        draw_text(">", x - 30.0, quit_y, 32.0, YELLOW);
        draw_text("<", x + button_width + 10.0, quit_y, 32.0, YELLOW);
    }

    // Version info
    let version = "v0.1";
    draw_text(
        version,
        screen_width() - 50.0,
        screen_height() - 20.0,
        16.0,
        LIGHTGRAY
    );
}
