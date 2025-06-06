use macroquad::prelude::*;
use crate::game::state::{GameState, GameScreen}; // Fix import
use crate::resources::manager::ResourceManager;
use crate::audio::manager::AudioManager;
use super::system::MenuSystem;

pub struct SettingsMenu {
    selected_option: usize,
    options: Vec<SettingsOption>,
}

#[derive(Clone)]
pub enum SettingsOption {
    SoundVolume,
    MusicVolume,
    ToggleSound,
    ToggleMusic,
    Back,
}

impl SettingsMenu {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: vec![
                SettingsOption::SoundVolume,
                SettingsOption::MusicVolume,
                SettingsOption::ToggleSound,
                SettingsOption::ToggleMusic,
                SettingsOption::Back,
            ],
        }
    }
    
    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.selected_option = if self.selected_option > 0 {
                self.selected_option - 1
            } else {
                self.options.len() - 1
            };
        }
        
        if is_key_pressed(KeyCode::Down) {
            self.selected_option = (self.selected_option + 1) % self.options.len();
        }
    }
    
    pub fn handle_input(&mut self, game_state: &mut GameState) -> bool {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            match &self.options[self.selected_option] {
                SettingsOption::ToggleSound => {
                    game_state.sound_muted = !game_state.sound_muted;
                },
                SettingsOption::ToggleMusic => {
                    game_state.music_muted = !game_state.music_muted;
                },
                SettingsOption::Back => {
                    return true; // Signal to go back
                },
                _ => {}
            }
        }
        
        // Volume adjustment
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::Right) {
            let increment = if is_key_down(KeyCode::Right) { 0.05 } else { -0.05 };
            
            match &self.options[self.selected_option] {
                SettingsOption::SoundVolume => {
                    game_state.sound_volume = (game_state.sound_volume + increment).clamp(0.0, 1.0);
                },
                SettingsOption::MusicVolume => {
                    game_state.music_volume = (game_state.music_volume + increment).clamp(0.0, 1.0);
                },
                _ => {}
            }
        }
        
        false
    }
    
    pub fn draw(&self, game_state: &GameState) {
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        
        // Background
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), 
                      Color::new(0.0, 0.0, 0.1, 0.8));
        
        // Title
        let title = "SETTINGS";
        let title_size = 48.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(title, center_x - title_width / 2.0, center_y - 150.0, title_size, WHITE);
        
        // Options
        let option_spacing = 60.0;
        let start_y = center_y - 50.0;
        
        for (i, option) in self.options.iter().enumerate() {
            let y = start_y + i as f32 * option_spacing;
            let is_selected = i == self.selected_option;
            
            let color = if is_selected { YELLOW } else { WHITE };
            let size = if is_selected { 24.0 } else { 20.0 };
            
            match option {
                SettingsOption::SoundVolume => {
                    let text = format!("Sound Volume: {}%", (game_state.sound_volume * 100.0) as i32);
                    let text_width = measure_text(&text, None, size as u16, 1.0).width;
                    draw_text(&text, center_x - text_width / 2.0, y, size, color);
                    
                    if is_selected {
                        draw_text("← →", center_x + text_width / 2.0 + 20.0, y, 16.0, LIGHTGRAY);
                    }
                },
                SettingsOption::MusicVolume => {
                    let text = format!("Music Volume: {}%", (game_state.music_volume * 100.0) as i32);
                    let text_width = measure_text(&text, None, size as u16, 1.0).width;
                    draw_text(&text, center_x - text_width / 2.0, y, size, color);
                    
                    if is_selected {
                        draw_text("← →", center_x + text_width / 2.0 + 20.0, y, 16.0, LIGHTGRAY);
                    }
                },
                SettingsOption::ToggleSound => {
                    let status = if game_state.sound_muted { "OFF" } else { "ON" };
                    let text = format!("Sound Effects: {}", status);
                    let text_width = measure_text(&text, None, size as u16, 1.0).width;
                    draw_text(&text, center_x - text_width / 2.0, y, size, color);
                },
                SettingsOption::ToggleMusic => {
                    let status = if game_state.music_muted { "OFF" } else { "ON" };
                    let text = format!("Music: {}", status);
                    let text_width = measure_text(&text, None, size as u16, 1.0).width;
                    draw_text(&text, center_x - text_width / 2.0, y, size, color);
                },
                SettingsOption::Back => {
                    let text = "Back to Main Menu";
                    let text_width = measure_text(text, None, size as u16, 1.0).width;
                    draw_text(text, center_x - text_width / 2.0, y, size, color);
                },
            }
            
            // Selection indicator
            if is_selected {
                let indicator_x = center_x - 200.0;
                draw_triangle(
                    Vec2::new(indicator_x, y - 5.0),
                    Vec2::new(indicator_x, y + 5.0),
                    Vec2::new(indicator_x + 10.0, y),
                    YELLOW
                );
            }
        }
        
        // Instructions
        let instructions = vec![
            "Use arrow keys to navigate",
            "Enter to select",
            "← → to adjust volumes",
            "ESC to go back",
        ];
        
        let instructions_y = center_y + 200.0;
        for (i, instruction) in instructions.iter().enumerate() {
            let text_width = measure_text(instruction, None, 14, 1.0).width;
            draw_text(instruction, center_x - text_width / 2.0, 
                     instructions_y + i as f32 * 20.0, 14.0, GRAY);
        }
    }
}

// Add draw function
pub fn draw(
    menu_system: &MenuSystem,
    game_state: &GameState,
) {
    draw_settings_screen(menu_system, game_state);
}
