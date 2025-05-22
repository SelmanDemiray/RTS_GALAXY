use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use super::system::MenuSystem;

pub fn draw(_menu: &MenuSystem, game_state: &mut GameState) {
    let screen_center_x = screen_width() / 2.0;
    
    draw_text(
        "Settings",
        screen_center_x - 80.0,
        120.0,
        48.0,
        WHITE
    );
    
    // Draw back button
    let back_rect = Rect::new(20.0, 20.0, 120.0, 40.0);
    let mouse_pos = mouse_position();
    let back_hover = back_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
    
    draw_rectangle(back_rect.x, back_rect.y, back_rect.w, back_rect.h, 
                  if back_hover { DARKBLUE } else { BLUE });
    draw_text("Back", back_rect.x + 40.0, back_rect.y + 25.0, 20.0, WHITE);
    
    if back_hover && is_mouse_button_pressed(MouseButton::Left) {
        game_state.current_screen = GameScreen::MainMenu;
    }
    
    // Add game settings options
    let y_start = 200.0;
    let option_spacing = 50.0;
    
    // Game difficulty option
    draw_text("Game Difficulty:", screen_center_x - 150.0, y_start, 24.0, WHITE);
    
    let difficulty_options = ["Easy", "Normal", "Hard"];
    for (i, &diff) in difficulty_options.iter().enumerate() {
        let opt_rect = Rect::new(screen_center_x - 50.0 + i as f32 * 100.0, y_start + 30.0, 80.0, 30.0);
        let mouse_over = opt_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
        
        draw_rectangle(opt_rect.x, opt_rect.y, opt_rect.w, opt_rect.h,
                     if mouse_over { DARKBLUE } else { BLUE });
        
        let text_size = measure_text(diff, None, 20, 1.0);
        draw_text(
            diff,
            opt_rect.x + opt_rect.w/2.0 - text_size.width/2.0,
            opt_rect.y + opt_rect.h/2.0 + text_size.height/2.0,
            20.0,
            WHITE
        );
    }
    
    // Add sound volume slider
    draw_text("Sound Volume:", screen_center_x - 150.0, y_start + option_spacing, 24.0, WHITE);
    draw_rectangle(screen_center_x - 150.0, y_start + option_spacing + 30.0, 300.0, 10.0, DARKGRAY);
    draw_rectangle(screen_center_x - 150.0, y_start + option_spacing + 30.0, 200.0, 10.0, GREEN);
    draw_circle(screen_center_x - 150.0 + 200.0, y_start + option_spacing + 35.0, 15.0, WHITE);
}
