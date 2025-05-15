use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use crate::resources::ResourceManager;

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
    
    pub fn draw(&self, _resource_manager: &ResourceManager) {
        let screen_center_x = screen_width() / 2.0;
        
        // Draw title
        let title = "Fantasy RTS";
        let title_size = measure_text(title, None, self.title_font_size as u16, 1.0);
        draw_text(
            title,
            screen_center_x - title_size.width / 2.0,
            self.title_y,
            self.title_font_size,
            GOLD
        );
        
        // Draw buttons
        self.draw_button("Play Game", self.first_button_y, GameScreen::Playing);
        self.draw_button("Settings", self.first_button_y + self.button_spacing, GameScreen::Settings);
        self.draw_button("Credits", self.first_button_y + self.button_spacing * 2.0, GameScreen::Credits);
        self.draw_button("Quit", self.first_button_y + self.button_spacing * 3.0, GameScreen::MainMenu);
        
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
    
    fn draw_button(&self, text: &str, y: f32, screen: GameScreen) -> bool {
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
                unsafe {
                    let game_state_ptr = std::ptr::null_mut::<GameState>();
                    if !game_state_ptr.is_null() {
                        (*game_state_ptr).current_screen = screen;
                    }
                }
            }
        }
        
        clicked
    }
    
    pub fn draw_settings(&self, game_state: &mut GameState) {
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
    }
    
    pub fn draw_credits(&self) {
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
        
        draw_text(
            "Press ESC to return to menu",
            screen_center_x - 150.0,
            screen_height() - 50.0,
            20.0,
            LIGHTGRAY
        );
    }
}
