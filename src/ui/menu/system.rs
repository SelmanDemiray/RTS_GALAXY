use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use crate::resources::ResourceManager;
use super::{main_menu, settings, credits};

pub struct MenuSystem {
    title_font_size: f32,
    button_font_size: f32,
    button_width: f32,
    button_height: f32,
    #[allow(dead_code)]
    button_padding: f32,  // Keeping this field for future use
    title_y: f32,
    first_button_y: f32,
    button_spacing: f32,
}

impl MenuSystem {
    pub fn new() -> Self {
        Self {
            title_font_size: 48.0,
            button_font_size: 24.0,
            button_width: 200.0,
            button_height: 50.0,
            button_padding: 10.0,
            title_y: 120.0,
            first_button_y: 240.0,
            button_spacing: 70.0,
        }
    }
    
    pub fn initialize(&mut self, _resource_manager: &ResourceManager) {
        // Any initialization needed when resources are loaded
    }
    
    pub fn update(&mut self, game_state: &mut GameState) {
        if is_key_pressed(KeyCode::Escape) && game_state.current_screen != GameScreen::MainMenu {
            game_state.current_screen = GameScreen::MainMenu;
        }
    }
    
    pub fn draw(&self, game_state: &mut GameState, _resource_manager: &ResourceManager) {
        match game_state.current_screen {
            GameScreen::MainMenu => main_menu::draw(self, game_state),
            GameScreen::Settings => self.draw_settings(game_state),
            GameScreen::Credits => self.draw_credits(game_state),
            _ => {} // Do nothing for other screens
        }
    }
    
    pub fn draw_button(&self, text: &str, y: f32, screen: GameScreen, game_state: &mut GameState) -> bool {
        let screen_center_x = screen_width() / 2.0;
        let button_x = screen_center_x - self.button_width / 2.0;
        
        // Draw button background
        let rect = Rect::new(button_x, y, self.button_width, self.button_height);
        let mouse_pos = mouse_position();
        let mouse_over = rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
        
        let color = if mouse_over {
            Color::new(0.4, 0.4, 0.8, 1.0)
        } else {
            Color::new(0.2, 0.2, 0.6, 1.0)
        };
        
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, WHITE);
        
        // Draw button text
        let text_size = measure_text(text, None, self.button_font_size as u16, 1.0);
        draw_text(
            text,
            screen_center_x - text_size.width / 2.0,
            y + self.button_height / 2.0 + text_size.height / 2.0,
            self.button_font_size,
            WHITE
        );
        
        // Handle click
        let clicked = mouse_over && is_mouse_button_pressed(MouseButton::Left);
        if clicked {
            if text == "Quit" {
                std::process::exit(0);
            } else {
                // Directly update the game state screen
                game_state.current_screen = screen;
            }
        }
        
        clicked
    }
    
    pub fn draw_settings(&self, game_state: &mut GameState) {
        settings::draw(self, game_state);
    }
    
    pub fn draw_credits(&self, game_state: &mut GameState) {
        credits::draw(self, game_state);
    }
    
    // Getter methods for menu components to access properties
    pub fn get_title_font_size(&self) -> f32 { self.title_font_size }
    #[allow(dead_code)]
    pub fn get_button_font_size(&self) -> f32 { self.button_font_size }
    #[allow(dead_code)]
    pub fn get_button_width(&self) -> f32 { self.button_width }
    #[allow(dead_code)]
    pub fn get_button_height(&self) -> f32 { self.button_height }
    pub fn get_title_y(&self) -> f32 { self.title_y }
    pub fn get_first_button_y(&self) -> f32 { self.first_button_y }
    pub fn get_button_spacing(&self) -> f32 { self.button_spacing }
}
