use macroquad::prelude::*;
use crate::game::GameState;
use super::system::MenuSystem;

pub fn draw(_menu: &MenuSystem, _game_state: &mut GameState) {
    let screen_center_x = screen_width() / 2.0;
    
    draw_text(
        "Credits",
        screen_center_x - 70.0,
        120.0,
        48.0,
        WHITE
    );
    
    draw_text(
        "Game created by: You!",
        screen_center_x - 150.0,
        200.0,
        24.0,
        WHITE
    );
    
    // Add more credits content
    let y_start = 250.0;
    let line_spacing = 30.0;
    
    draw_text("Programming Team", screen_center_x - 100.0, y_start, 20.0, GOLD);
    draw_text("Lead Developer - Your Name", screen_center_x - 150.0, y_start + line_spacing, 18.0, WHITE);
    draw_text("AI Programmer - GitHub Copilot", screen_center_x - 150.0, y_start + line_spacing*2.0, 18.0, WHITE);
    
    draw_text("Art Team", screen_center_x - 60.0, y_start + line_spacing*3.5, 20.0, GOLD);
    draw_text("Game Artist - Fantasy RTS Art Team", screen_center_x - 150.0, y_start + line_spacing*4.5, 18.0, WHITE);
    
    draw_text("Special Thanks", screen_center_x - 80.0, y_start + line_spacing*6.0, 20.0, GOLD);
    draw_text("All the open source projects that made this possible", screen_center_x - 240.0, y_start + line_spacing*7.0, 18.0, WHITE);
    draw_text("macroquad, egui, and Rust community", screen_center_x - 190.0, y_start + line_spacing*8.0, 18.0, WHITE);
    draw_text("Asset creators and open source art contributors", screen_center_x - 220.0, y_start + line_spacing*9.0, 18.0, WHITE);
    
    draw_text(
        "Press ESC to return to menu",
        screen_center_x - 150.0,
        screen_height() - 50.0,
        20.0,
        LIGHTGRAY
    );
}
