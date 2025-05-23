// We're exporting the system module but using our own implementation
pub mod main_menu;
pub mod settings;
pub mod credits;
mod system;

use crate::game::{GameState, GameScreen, GameMode};
use crate::resources::ResourceManager;
use crate::audio::AudioManager;
use macroquad::prelude::*;

// Rename to make clear this is separate from the system module
pub struct MenuSystem {
    initialized: bool,
}

impl MenuSystem {
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }
    
    pub fn initialize(&mut self, _resource_manager: &ResourceManager) {
        self.initialized = true;
    }
    
    pub fn update(&mut self, game_state: &mut GameState, resource_manager: &ResourceManager, audio_manager: &mut AudioManager) {
        match game_state.current_screen {
            GameScreen::MainMenu => self.update_main_menu(game_state, resource_manager, audio_manager),
            GameScreen::Settings => self.update_settings(game_state, resource_manager, audio_manager),
            GameScreen::Credits => self.update_credits(game_state, resource_manager, audio_manager),
            _ => {}
        }
    }
    
    fn update_main_menu(&self, game_state: &mut GameState, resource_manager: &ResourceManager, audio_manager: &mut AudioManager) {
        // Handle main menu button clicks
        let button_width = 200.0;
        let button_height = 50.0;
        let center_x = screen_width() / 2.0 - button_width / 2.0;
        let start_y = screen_height() / 2.0 - 100.0;
        let spacing = 70.0;
        
        if self.button(center_x, start_y, button_width, button_height, "Start Game") {
            audio_manager.play_ui_click(resource_manager, game_state);
            game_state.current_screen = GameScreen::Playing;
            game_state.set_game_mode(GameMode::Offline);
        }
        
        if self.button(center_x, start_y + spacing, button_width, button_height, "Settings") {
            audio_manager.play_ui_click(resource_manager, game_state);
            game_state.current_screen = GameScreen::Settings;
        }
        
        if self.button(center_x, start_y + spacing * 2.0, button_width, button_height, "Credits") {
            audio_manager.play_ui_click(resource_manager, game_state);
            game_state.current_screen = GameScreen::Credits;
        }
        
        if self.button(center_x, start_y + spacing * 3.0, button_width, button_height, "Exit") {
            audio_manager.play_ui_click(resource_manager, game_state);
            std::process::exit(0);
        }
    }
    
    fn update_settings(&self, game_state: &mut GameState, resource_manager: &ResourceManager, audio_manager: &mut AudioManager) {
        // Settings screen
        let button_width = 200.0;
        let button_height = 50.0;
        let center_x = screen_width() / 2.0 - button_width / 2.0;
        let start_y = 100.0;
        let spacing = 70.0;
        
        // Sound volume slider
        let sound_label = format!("Sound Volume: {:.0}%", game_state.sound_volume * 100.0);
        draw_text(&sound_label, center_x, start_y, 20.0, WHITE);
        
        // Draw slider background
        draw_rectangle(center_x, start_y + 30.0, button_width, 20.0, GRAY);
        
        // Draw slider fill
        draw_rectangle(center_x, start_y + 30.0, button_width * game_state.sound_volume, 20.0, GREEN);
        
        // Handle slider interaction
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if mouse_pos.1 >= start_y + 30.0 && mouse_pos.1 <= start_y + 50.0 &&
               mouse_pos.0 >= center_x && mouse_pos.0 <= center_x + button_width {
                game_state.sound_volume = ((mouse_pos.0 - center_x) / button_width).clamp(0.0, 1.0);
            }
        }
        
        // Music volume slider
        let music_label = format!("Music Volume: {:.0}%", game_state.music_volume * 100.0);
        draw_text(&music_label, center_x, start_y + spacing, 20.0, WHITE);
        
        // Draw slider background
        draw_rectangle(center_x, start_y + spacing + 30.0, button_width, 20.0, GRAY);
        
        // Draw slider fill
        draw_rectangle(center_x, start_y + spacing + 30.0, button_width * game_state.music_volume, 20.0, GREEN);
        
        // Handle slider interaction
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if mouse_pos.1 >= start_y + spacing + 30.0 && mouse_pos.1 <= start_y + spacing + 50.0 &&
               mouse_pos.0 >= center_x && mouse_pos.0 <= center_x + button_width {
                game_state.music_volume = ((mouse_pos.0 - center_x) / button_width).clamp(0.0, 1.0);
            }
        }
        
        // Sound mute toggle
        let sound_mute_text = if game_state.sound_muted { "Unmute Sound" } else { "Mute Sound" };
        if self.button(center_x, start_y + spacing * 2.0, button_width, button_height, sound_mute_text) {
            audio_manager.play_ui_click(resource_manager, game_state);
            game_state.sound_muted = !game_state.sound_muted;
        }
        
        // Music mute toggle
        let music_mute_text = if game_state.music_muted { "Unmute Music" } else { "Mute Music" };
        if self.button(center_x, start_y + spacing * 3.0, button_width, button_height, music_mute_text) {
            audio_manager.play_ui_click(resource_manager, game_state);
            game_state.music_muted = !game_state.music_muted;
        }
        
        // Game difficulty selection
        let difficulty_y = start_y + spacing * 4.0;
        draw_text("Game Difficulty:", center_x, difficulty_y, 20.0, WHITE);
        
        let difficulties = ["Easy", "Normal", "Hard"];
        let difficulty_button_width = 120.0;
        let difficulty_button_height = 40.0;
        let difficulty_button_spacing = 10.0;
        let total_width = difficulties.len() as f32 * difficulty_button_width + (difficulties.len() - 1) as f32 * difficulty_button_spacing;
        let difficulty_start_x = (screen_width() - total_width) / 2.0;
        
        for (i, difficulty) in difficulties.iter().enumerate() {
            let button_x = difficulty_start_x + i as f32 * (difficulty_button_width + difficulty_button_spacing);
            let button_y = difficulty_y + 30.0;
            
            let is_selected = game_state.game_difficulty == i;
            let button_color = if is_selected { GREEN } else { SKYBLUE };
            
            if self.button_colored(button_x, button_y, difficulty_button_width, difficulty_button_height, difficulty, button_color) {
                audio_manager.play_ui_click(resource_manager, game_state);
                game_state.game_difficulty = i;
            }
        }
        
        // Back button - moved further down to avoid overlap
        if self.button(center_x, start_y + spacing * 5.5, button_width, button_height, "Back") {
            audio_manager.play_ui_click(resource_manager, game_state);
            game_state.current_screen = GameScreen::MainMenu;
        }
    }
    
    fn update_credits(&self, game_state: &mut GameState, resource_manager: &ResourceManager, audio_manager: &mut AudioManager) {
        // Credits screen
        let button_width = 200.0;
        let button_height = 50.0;
        let center_x = screen_width() / 2.0 - button_width / 2.0;
        let start_y = 100.0;
        
        draw_text("CREDITS", center_x + button_width / 4.0, start_y, 30.0, WHITE);
        draw_text("Programming: Your Name", center_x - 50.0, start_y + 60.0, 20.0, WHITE);
        draw_text("Art: Placeholder Art", center_x - 50.0, start_y + 90.0, 20.0, WHITE);
        draw_text("Sound: Free Resources", center_x - 50.0, start_y + 120.0, 20.0, WHITE);
        draw_text("Special Thanks:", center_x - 50.0, start_y + 160.0, 20.0, WHITE);
        draw_text("- Macroquad Team", center_x - 30.0, start_y + 190.0, 20.0, WHITE);
        draw_text("- Rust Community", center_x - 30.0, start_y + 220.0, 20.0, WHITE);
        
        if self.button(center_x, start_y + 300.0, button_width, button_height, "Back") {
            audio_manager.play_ui_click(resource_manager, game_state);
            game_state.current_screen = GameScreen::MainMenu;
        }
    }
    
    pub fn draw(&self, game_state: &mut GameState, _resource_manager: &ResourceManager) {
        match game_state.current_screen {
            GameScreen::MainMenu => {
                // Draw title
                let title = "Fantasy RTS Game";
                let title_size = 50.0;
                let title_width = measure_text(title, None, title_size as u16, 1.0).width;
                draw_text(title, screen_width() / 2.0 - title_width / 2.0, 100.0, title_size, WHITE);
            },
            GameScreen::Settings => {
                // Draw title
                let title = "Settings";
                let title_size = 40.0;
                let title_width = measure_text(title, None, title_size as u16, 1.0).width;
                draw_text(title, screen_width() / 2.0 - title_width / 2.0, 50.0, title_size, WHITE);
            },
            GameScreen::Credits => {
                // Title is drawn in the update method
            },
            _ => {}
        }
    }
    
    fn button(&self, x: f32, y: f32, width: f32, height: f32, text: &str) -> bool {
        button(x, y, width, height, text, SKYBLUE)
    }
    
    fn button_colored(&self, x: f32, y: f32, width: f32, height: f32, text: &str, color: Color) -> bool {
        button(x, y, width, height, text, color)
    }
}

fn button(x: f32, y: f32, width: f32, height: f32, text: &str, color: Color) -> bool {
    let rect = Rect::new(x, y, width, height);
    let mouse_pos = mouse_position();
    let hover = rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
    
    // Draw button
    draw_rectangle(x, y, width, height, if hover { color } else { color.darker(0.2) });
    draw_rectangle_lines(x, y, width, height, 2.0, WHITE);
    
    // Draw text
    let font_size = 20.0;
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + (width - text_size.width) / 2.0;
    let text_y = y + (height + text_size.height) / 2.0;
    draw_text(text, text_x, text_y, font_size, WHITE);
    
    hover && is_mouse_button_released(MouseButton::Left)
}

// Helper method for color darkening
trait ColorExt {
    fn darker(&self, amount: f32) -> Self;
}

impl ColorExt for Color {
    fn darker(&self, amount: f32) -> Self {
        Color::new(
            (self.r * (1.0 - amount)).max(0.0),
            (self.g * (1.0 - amount)).max(0.0),
            (self.b * (1.0 - amount)).max(0.0),
            self.a
        )
    }
}
