use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use super::system::MenuSystem;

pub fn draw(menu: &MenuSystem, game_state: &mut GameState) {
    let screen_center_x = screen_width() / 2.0;
    
    // Draw title
    let title = "Settings";
    let title_size = measure_text(title, None, menu.get_title_font_size() as u16, 1.0);
    draw_text(
        title,
        screen_center_x - title_size.width / 2.0,
        menu.get_title_y(),
        menu.get_title_font_size(),
        WHITE
    );
    
    // Draw settings content
    let y_start = 200.0;
    let option_spacing = 60.0;
    
    // Game difficulty option
    let difficulty_label = "Game Difficulty:";
    let _label_size = measure_text(difficulty_label, None, 24, 1.0);
    draw_text(
        difficulty_label,
        screen_center_x - 200.0,
        y_start,
        24.0,
        WHITE
    );
    
    let difficulty_options = ["Easy", "Normal", "Hard"];
    let button_width = 80.0;
    let button_height = 30.0;
    let button_spacing = 20.0;
    let first_button_x = screen_center_x - (button_width * 1.5 + button_spacing);
    
    let mouse_pos = mouse_position();
    
    for (i, &diff) in difficulty_options.iter().enumerate() {
        let opt_x = first_button_x + i as f32 * (button_width + button_spacing);
        let opt_rect = Rect::new(opt_x, y_start + 15.0, button_width, button_height);
        let mouse_over = opt_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
        
        // Determine button color based on selection and hover state
        let color = if game_state.game_difficulty == i {
            // Selected difficulty
            if mouse_over {
                Color::new(0.5, 0.8, 0.5, 1.0) // Lighter green when selected and hovered
            } else {
                Color::new(0.3, 0.7, 0.3, 1.0) // Green when selected
            }
        } else {
            // Not selected
            if mouse_over {
                Color::new(0.4, 0.4, 0.8, 1.0) // Blue when hovered
            } else {
                Color::new(0.2, 0.2, 0.6, 1.0) // Dark blue by default
            }
        };
        
        draw_rectangle(opt_rect.x, opt_rect.y, opt_rect.w, opt_rect.h, color);
        draw_rectangle_lines(opt_rect.x, opt_rect.y, opt_rect.w, opt_rect.h, 1.0, WHITE);
        
        // Draw button text
        let text_size = measure_text(diff, None, 18, 1.0);
        draw_text(
            diff,
            opt_rect.x + (opt_rect.w - text_size.width) / 2.0,
            opt_rect.y + (opt_rect.h + text_size.height) / 2.0,
            18.0,
            WHITE
        );
        
        // Handle click - now actually updates game_state
        if mouse_over && is_mouse_button_pressed(MouseButton::Left) {
            game_state.game_difficulty = i;
        }
    }
    
    // Sound volume slider
    draw_text("Sound Volume:", screen_center_x - 200.0, y_start + option_spacing, 24.0, WHITE);
    
    // Draw mute toggle button for sound
    let mute_btn_size = 30.0;
    let mute_btn_x = screen_center_x + 180.0;
    let mute_btn_y = y_start + option_spacing + 15.0;
    let mute_btn_rect = Rect::new(mute_btn_x, mute_btn_y, mute_btn_size, mute_btn_size);
    
    // Change button color based on mute state
    let mute_btn_color = if game_state.sound_muted {
        Color::new(0.8, 0.2, 0.2, 1.0) // Red when muted
    } else {
        Color::new(0.2, 0.7, 0.2, 1.0) // Green when unmuted
    };
    
    let is_mute_btn_hovered = mute_btn_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
    
    // Draw the mute button with hover effect
    draw_rectangle(mute_btn_rect.x, mute_btn_rect.y, mute_btn_rect.w, mute_btn_rect.h, mute_btn_color);
    draw_rectangle_lines(mute_btn_rect.x, mute_btn_rect.y, mute_btn_rect.w, mute_btn_rect.h, 
                         if is_mute_btn_hovered { 2.0 } else { 1.0 }, WHITE);
    
    // Draw mute/unmute icon
    let icon_text = if game_state.sound_muted { "M" } else { "S" };
    let icon_size = measure_text(icon_text, None, 18, 1.0);
    draw_text(
        icon_text,
        mute_btn_rect.x + (mute_btn_rect.w - icon_size.width) / 2.0,
        mute_btn_rect.y + (mute_btn_rect.h + icon_size.height) / 2.0,
        18.0,
        WHITE
    );
    
    // Handle mute button click
    if is_mute_btn_hovered && is_mouse_button_pressed(MouseButton::Left) {
        game_state.sound_muted = !game_state.sound_muted;
    }
    
    // Draw slider track
    let slider_width = 300.0;
    let slider_height = 10.0;
    let slider_x = screen_center_x - 150.0;
    let slider_y = y_start + option_spacing + 30.0;
    
    draw_rectangle(slider_x, slider_y, slider_width, slider_height, DARKGRAY);
    
    // Draw slider fill - now uses actual sound volume from game state
    draw_rectangle(slider_x, slider_y, slider_width * game_state.sound_volume, slider_height, 
                  if game_state.sound_muted { GRAY } else { GREEN });
    
    // Draw slider handle
    let handle_radius = 15.0;
    let handle_x = slider_x + slider_width * game_state.sound_volume;
    let handle_y = slider_y + slider_height / 2.0;
    
    draw_circle(handle_x, handle_y, handle_radius, WHITE);
    
    // Handle slider interaction
    let handle_rect = Rect::new(
        handle_x - handle_radius, 
        handle_y - handle_radius,
        handle_radius * 2.0,
        handle_radius * 2.0
    );
    
    let is_handle_hovered = handle_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
    
    if is_handle_hovered {
        draw_circle_lines(handle_x, handle_y, handle_radius + 2.0, 2.0, GOLD);
        
        // Handle dragging
        if is_mouse_button_down(MouseButton::Left) {
            let new_pos = (mouse_pos.0 - slider_x).clamp(0.0, slider_width);
            game_state.sound_volume = new_pos / slider_width;
        }
    }
    
    // Music volume slider
    draw_text("Music Volume:", screen_center_x - 200.0, y_start + option_spacing * 2.0, 24.0, WHITE);
    
    // Draw mute toggle button for music
    let music_mute_btn_y = y_start + option_spacing * 2.0 + 15.0;
    let music_mute_btn_rect = Rect::new(mute_btn_x, music_mute_btn_y, mute_btn_size, mute_btn_size);
    
    // Change button color based on mute state
    let music_mute_btn_color = if game_state.music_muted {
        Color::new(0.8, 0.2, 0.2, 1.0) // Red when muted
    } else {
        Color::new(0.2, 0.2, 0.8, 1.0) // Blue when unmuted
    };
    
    let is_music_mute_btn_hovered = music_mute_btn_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
    
    // Draw the mute button with hover effect
    draw_rectangle(music_mute_btn_rect.x, music_mute_btn_rect.y, music_mute_btn_rect.w, music_mute_btn_rect.h, music_mute_btn_color);
    draw_rectangle_lines(music_mute_btn_rect.x, music_mute_btn_rect.y, music_mute_btn_rect.w, music_mute_btn_rect.h, 
                        if is_music_mute_btn_hovered { 2.0 } else { 1.0 }, WHITE);
    
    // Draw mute/unmute icon
    let music_icon_text = if game_state.music_muted { "M" } else { "♪" };
    let music_icon_size = measure_text(music_icon_text, None, 18, 1.0);
    draw_text(
        music_icon_text,
        music_mute_btn_rect.x + (music_mute_btn_rect.w - music_icon_size.width) / 2.0,
        music_mute_btn_rect.y + (music_mute_btn_rect.h + music_icon_size.height) / 2.0,
        18.0,
        WHITE
    );
    
    // Handle music mute button click
    if is_music_mute_btn_hovered && is_mouse_button_pressed(MouseButton::Left) {
        game_state.music_muted = !game_state.music_muted;
    }
    
    // Draw slider track
    let music_slider_y = y_start + option_spacing * 2.0 + 30.0;
    
    draw_rectangle(slider_x, music_slider_y, slider_width, slider_height, DARKGRAY);
    
    // Draw slider fill - now uses actual music volume from game state
    draw_rectangle(slider_x, music_slider_y, slider_width * game_state.music_volume, slider_height, 
                  if game_state.music_muted { GRAY } else { BLUE });
    
    // Draw slider handle
    let music_handle_x = slider_x + slider_width * game_state.music_volume;
    let music_handle_y = music_slider_y + slider_height / 2.0;
    
    draw_circle(music_handle_x, music_handle_y, handle_radius, WHITE);
    
    // Handle music slider interaction
    let music_handle_rect = Rect::new(
        music_handle_x - handle_radius, 
        music_handle_y - handle_radius,
        handle_radius * 2.0,
        handle_radius * 2.0
    );
    
    let is_music_handle_hovered = music_handle_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
    
    if is_music_handle_hovered {
        draw_circle_lines(music_handle_x, music_handle_y, handle_radius + 2.0, 2.0, GOLD);
        
        // Handle dragging
        if is_mouse_button_down(MouseButton::Left) {
            let new_pos = (mouse_pos.0 - slider_x).clamp(0.0, slider_width);
            game_state.music_volume = new_pos / slider_width;
        }
    }
    
    // Draw back button using the standard menu button system - moved to bottom of settings
    let back_button_y = y_start + option_spacing * 3.0 + 20.0;
    menu.draw_button("Back", back_button_y, GameScreen::MainMenu, game_state);
    
    // Additional settings options
    draw_text(
        "Press ESC to return to menu",
        screen_center_x - 150.0,
        screen_height() - 40.0,
        18.0,
        LIGHTGRAY
    );
}
