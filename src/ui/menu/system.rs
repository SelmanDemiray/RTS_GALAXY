use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use crate::audio::AudioManager;
use crate::resources::ResourceManager;

pub struct MenuSystem {
    pub selected_button: i32,
    pub button_hover_time: f32,
}

impl MenuSystem {
    pub fn new() -> Self {
        Self {
            selected_button: 0,
            button_hover_time: 0.0,
        }
    }
    
    pub fn initialize(&mut self, _resource_manager: &ResourceManager) {
        self.selected_button = 0;
    }

    // Add missing helper methods for modular menu system
    pub fn get_title_font_size(&self) -> f32 {
        72.0
    }

    pub fn get_title_y(&self) -> f32 {
        screen_height() * 0.3
    }

    pub fn get_first_button_y(&self) -> f32 {
        screen_height() * 0.5
    }

    pub fn get_button_spacing(&self) -> f32 {
        60.0
    }

    pub fn draw_button(&self, text: &str, y: f32, target_screen: GameScreen, _game_state: &GameState) {
        let screen_width = screen_width();
        let button_width = measure_text(text, None, 32, 1.0).width;
        let x = (screen_width - button_width) / 2.0;
        
        // Determine if this button should be highlighted
        let is_selected = match target_screen {
            GameScreen::Playing => self.selected_button == 0,
            GameScreen::Settings => self.selected_button == 1,
            GameScreen::Credits => self.selected_button == 2,
            GameScreen::Quit => self.selected_button == 3,
            GameScreen::MainMenu => true, // For back buttons
        };
        
        let color = if is_selected { YELLOW } else { WHITE };
        draw_text(text, x, y, 32.0, color);
        
        if is_selected {
            draw_text(">", x - 30.0, y, 32.0, YELLOW);
            draw_text("<", x + button_width + 10.0, y, 32.0, YELLOW);
        }
    }

    pub fn update(&mut self, game_state: &mut GameState, _resource_manager: &ResourceManager, _audio_manager: &mut AudioManager) {
        match game_state.current_screen {
            GameScreen::MainMenu => self.update_main_menu(game_state),
            GameScreen::Settings => self.update_settings(game_state),
            GameScreen::Credits => self.update_credits(game_state),
            _ => {}
        }
    }

    fn update_main_menu(&mut self, game_state: &mut GameState) {
        // Handle keyboard navigation
        if is_key_pressed(KeyCode::Up) {
            self.selected_button = if self.selected_button == 0 { 3 } else { self.selected_button - 1 };
        }
        if is_key_pressed(KeyCode::Down) {
            self.selected_button = (self.selected_button + 1) % 4;
        }
        
        if is_key_pressed(KeyCode::Enter) {
            match self.selected_button {
                0 => game_state.current_screen = GameScreen::Playing,
                1 => game_state.current_screen = GameScreen::Settings,
                2 => game_state.current_screen = GameScreen::Credits,
                3 => game_state.request_quit(),
                _ => {}
            }
        }
    }

    fn update_settings(&mut self, game_state: &mut GameState) {
        if is_key_pressed(KeyCode::Escape) {
            game_state.current_screen = GameScreen::MainMenu;
            self.selected_button = 1; // Return to Settings button
        }
    }

    fn update_credits(&mut self, game_state: &mut GameState) {
        if is_key_pressed(KeyCode::Escape) {
            game_state.current_screen = GameScreen::MainMenu;
            self.selected_button = 2; // Return to Credits button
        }
    }

    pub fn draw(&self, game_state: &GameState, _resource_manager: &ResourceManager) { 
        match game_state.current_screen {
            GameScreen::MainMenu => {
                clear_background(Color::new(0.1, 0.1, 0.15, 1.0));
                super::main_menu::draw(self, game_state);
            },
            GameScreen::Settings => {
                clear_background(Color::new(0.1, 0.1, 0.15, 1.0));
                super::settings::draw(self, game_state);
            },
            GameScreen::Credits => self.draw_credits(),
            _ => {}
        }
    }

    pub fn draw_main_menu(
        &mut self,
        game_state: &mut GameState,
        audio_manager: &AudioManager,
        resource_manager: &ResourceManager
    ) {
        super::main_menu::draw_main_menu(self, game_state, audio_manager, resource_manager);
    }
    
    pub fn draw_settings(
        &mut self,
        game_state: &mut GameState,
        audio_manager: &AudioManager,
        resource_manager: &ResourceManager
    ) {
        super::settings::draw_settings(self, game_state, audio_manager, resource_manager);
    }
    
    pub fn draw_credits(
        &mut self,
        game_state: &mut GameState,
        audio_manager: &AudioManager,
        resource_manager: &ResourceManager
    ) {
        super::credits::draw_credits(self, game_state, audio_manager, resource_manager);
    }
    
    pub fn draw_button(&self, x: f32, y: f32, width: f32, height: f32, text: &str, is_hovered: bool) -> bool {
        let color = if is_hovered {
            LIGHTGRAY
        } else {
            GRAY
        };
        
        draw_rectangle(x, y, width, height, color);
        draw_rectangle_lines(x, y, width, height, 2.0, WHITE);
        
        let text_size = 24.0;
        let text_dims = measure_text(text, None, text_size as u16, 1.0);
        let text_x = x + (width - text_dims.width) / 2.0;
        let text_y = y + (height + text_dims.height) / 2.0;
        
        draw_text(text, text_x, text_y, text_size, WHITE);
        
        // Check if clicked
        if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
            return true;
        }
        
        false
    }
    
    pub fn is_point_in_rect(&self, px: f32, py: f32, x: f32, y: f32, width: f32, height: f32) -> bool {
        px >= x && px <= x + width && py >= y && py <= y + height
    }
    
    fn draw_main_menu(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        // Draw background
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));
        
        // Draw title
        let title = "Fantasy RTS";
        let title_size = 72.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(title, (screen_width - title_width) / 2.0, screen_height * 0.3, title_size, GOLD);
        
        // Draw menu buttons
        let buttons = ["Start Game", "Settings", "Credits", "Quit"];
        let button_y_start = screen_height * 0.5;
        let button_spacing = 60.0;
        
        for (i, button_text) in buttons.iter().enumerate() {
            let y = button_y_start + i as f32 * button_spacing;
            let button_width = measure_text(button_text, None, 32, 1.0).width;
            let x = (screen_width - button_width) / 2.0;
            
            let color = if i == self.selected_button { YELLOW } else { WHITE };
            draw_text(button_text, x, y, 32.0, color);
            
            if i == self.selected_button {
                draw_text(">", x - 30.0, y, 32.0, YELLOW);
                draw_text("<", x + button_width + 10.0, y, 32.0, YELLOW);
            }
        }
        
        // Draw instructions
        let instructions = "Use UP/DOWN arrows and ENTER to navigate";
        let inst_width = measure_text(instructions, None, 20, 1.0).width;
        draw_text(instructions, (screen_width - inst_width) / 2.0, screen_height * 0.9, 20.0, GRAY);
    }

    fn draw_settings(&self, game_state: &GameState) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));
        
        // Draw title
        let title = "Settings";
        let title_size = 48.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(title, (screen_width - title_width) / 2.0, screen_height * 0.2, title_size, GOLD);
        
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
        
        // Instructions
        draw_text("Press ESC to return to main menu", 100.0, screen_height * 0.8, 20.0, GRAY);
    }

    fn draw_credits(&self) {
        let screen_width = screen_width();
        let _screen_height = screen_height();
        
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));
        
        // Draw title
        let title = "Credits";
        let title_size = 48.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(title, (screen_width - title_width) / 2.0, 150.0, title_size, GOLD);
        
        // Draw credits
        let credits = [
            "Fantasy RTS Game",
            "",
            "Built with Rust and Macroquad",
            "",
            "Game Design: AI Assistant",
            "Programming: AI Assistant",
            "Graphics: Macroquad",
            "",
            "Special Thanks:",
            "- Rust Community",
            "- Macroquad Framework",
            "- Open Source Contributors",
        ];
        
        let start_y = 250.0;
        for (i, line) in credits.iter().enumerate() {
            let line_width = measure_text(line, None, 24, 1.0).width;
            let x = (screen_width - line_width) / 2.0;
            let y = start_y + i as f32 * 30.0;
            draw_text(line, x, y, 24.0, WHITE);
        }
        
        // Instructions
        let instructions = "Press ESC to return to main menu";
        let inst_width = measure_text(instructions, None, 20, 1.0).width;
        draw_text(instructions, (screen_width - inst_width) / 2.0, screen_height() * 0.9, 20.0, GRAY);
    }
}
