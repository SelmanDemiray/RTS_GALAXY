use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use crate::audio::AudioManager;
use crate::resources::ResourceManager;
use super::system::MenuSystem;

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub text: String,
    pub action: MenuAction,
    pub rect: Rect,
    pub is_hovered: bool,
    pub is_enabled: bool,
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    StartGame,
    Settings,
    Credits,
    Quit,
    JoinMultiplayer,
    HostGame,
}

pub struct MainMenu {
    items: Vec<MenuItem>,
    background_alpha: f32,
    title_scale: f32,
    selection_index: usize,
}

impl MainMenu {
    pub fn new() -> Self {
        let center_x = screen_width() / 2.0;
        let start_y = screen_height() / 2.0 - 50.0;
        let button_height = 40.0;
        let button_width = 200.0;
        let spacing = 50.0;

        let items = vec![
            MenuItem {
                text: "Start Game".to_string(),
                action: MenuAction::StartGame,
                rect: Rect::new(center_x - button_width/2.0, start_y, button_width, button_height),
                is_hovered: false,
                is_enabled: true,
            },
            MenuItem {
                text: "Join Multiplayer".to_string(),
                action: MenuAction::JoinMultiplayer,
                rect: Rect::new(center_x - button_width/2.0, start_y + spacing, button_width, button_height),
                is_hovered: false,
                is_enabled: true,
            },
            MenuItem {
                text: "Host Game".to_string(),
                action: MenuAction::HostGame,
                rect: Rect::new(center_x - button_width/2.0, start_y + spacing * 2.0, button_width, button_height),
                is_hovered: false,
                is_enabled: true,
            },
            MenuItem {
                text: "Settings".to_string(),
                action: MenuAction::Settings,
                rect: Rect::new(center_x - button_width/2.0, start_y + spacing * 3.0, button_width, button_height),
                is_hovered: false,
                is_enabled: true,
            },
            MenuItem {
                text: "Credits".to_string(),
                action: MenuAction::Credits,
                rect: Rect::new(center_x - button_width/2.0, start_y + spacing * 4.0, button_width, button_height),
                is_hovered: false,
                is_enabled: true,
            },
            MenuItem {
                text: "Quit".to_string(),
                action: MenuAction::Quit,
                rect: Rect::new(center_x - button_width/2.0, start_y + spacing * 5.0, button_width, button_height),
                is_hovered: false,
                is_enabled: true,
            },
        ];

        Self {
            items,
            background_alpha: 0.8,
            title_scale: 1.0,
            selection_index: 0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Animate title scale
        self.title_scale = 1.0 + (get_time() as f32 * 2.0).sin() * 0.05;
        
        // Update menu positions if screen size changed
        self.update_positions();
        
        // Handle mouse hover
        let mouse_pos = Vec2::from(mouse_position());
        for (i, item) in self.items.iter_mut().enumerate() {
            let was_hovered = item.is_hovered;
            item.is_hovered = item.rect.contains(mouse_pos) && item.is_enabled;
            
            // Play hover sound when mouse enters button
            if item.is_hovered && !was_hovered {
                // Audio would be played here in a full implementation
                self.selection_index = i;
            }
        }
        
        // Handle keyboard navigation
        if is_key_pressed(KeyCode::Up) {
            self.selection_index = if self.selection_index > 0 { 
                self.selection_index - 1 
            } else { 
                self.items.len() - 1 
            };
        }
        
        if is_key_pressed(KeyCode::Down) {
            self.selection_index = (self.selection_index + 1) % self.items.len();
        }
    }

    fn update_positions(&mut self) {
        let center_x = screen_width() / 2.0;
        let start_y = screen_height() / 2.0 - 50.0;
        let button_width = 200.0;
        let button_height = 40.0;
        let spacing = 50.0;

        for (i, item) in self.items.iter_mut().enumerate() {
            item.rect.x = center_x - button_width / 2.0;
            item.rect.y = start_y + spacing * i as f32;
            item.rect.w = button_width;
            item.rect.h = button_height;
        }
    }

    pub fn handle_input(
        &mut self, 
        game_state: &mut GameState, 
        audio_manager: &AudioManager, 
        resource_manager: &ResourceManager
    ) -> bool {
        let mut action_taken = false;
        
        // Handle mouse clicks
        if is_mouse_button_pressed(MouseButton::Left) {
            for item in &self.items {
                if item.is_hovered && item.is_enabled {
                    action_taken = self.execute_action(&item.action, game_state, audio_manager, resource_manager);
                    break;
                }
            }
        }
        
        // Handle keyboard selection
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            if let Some(item) = self.items.get(self.selection_index) {
                if item.is_enabled {
                    action_taken = self.execute_action(&item.action, game_state, audio_manager, resource_manager);
                }
            }
        }
        
        action_taken
    }

    fn execute_action(
        &self, 
        action: &MenuAction, 
        game_state: &mut GameState, 
        audio_manager: &AudioManager, 
        resource_manager: &ResourceManager
    ) -> bool {
        audio_manager.play_ui_click(resource_manager, game_state);
        
        match action {
            MenuAction::StartGame => {
                game_state.current_screen = GameScreen::Playing;
                true
            },
            MenuAction::Settings => {
                game_state.current_screen = GameScreen::Settings;
                true
            },
            MenuAction::Credits => {
                game_state.current_screen = GameScreen::Credits;
                true
            },
            MenuAction::Quit => {
                game_state.request_quit();
                true
            },
            MenuAction::JoinMultiplayer => {
                println!("Join Multiplayer selected - not yet implemented");
                false
            },
            MenuAction::HostGame => {
                println!("Host Game selected - not yet implemented");
                false
            },
        }
    }

    pub fn draw(&self, menu_system: &MenuSystem) {
        // Draw background
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), 
                      Color::new(0.0, 0.05, 0.1, self.background_alpha));
        
        // Draw animated background elements
        self.draw_background_effects();
        
        // Draw title
        let title_text = "RTS GALAXY";
        let title_size = 64.0 * self.title_scale;
        let title_width = measure_text(title_text, None, title_size as u16, 1.0).width;
        let title_x = (screen_width() - title_width) / 2.0;
        let title_y = screen_height() / 2.0 - 200.0;
        
        // Title glow effect
        draw_text(title_text, title_x + 2.0, title_y + 2.0, title_size, Color::new(0.0, 0.5, 1.0, 0.3));
        draw_text(title_text, title_x, title_y, title_size, WHITE);
        
        // Draw subtitle
        let subtitle = "Advanced Real-Time Strategy";
        let subtitle_size = 20.0;
        let subtitle_width = measure_text(subtitle, None, subtitle_size as u16, 1.0).width;
        let subtitle_x = (screen_width() - subtitle_width) / 2.0;
        draw_text(subtitle, subtitle_x, title_y + 80.0, subtitle_size, LIGHTGRAY);
        
        // Draw menu items
        for (i, item) in self.items.iter().enumerate() {
            self.draw_menu_item(item, i == self.selection_index, menu_system);
        }
        
        // Draw version and additional info
        draw_text("v0.1.0 - Alpha", 10.0, screen_height() - 40.0, 16.0, GRAY);
        draw_text("Built with Rust & macroquad", 10.0, screen_height() - 20.0, 14.0, GRAY);
        
        // Draw controls hint
        let controls_text = "Use arrow keys to navigate, Enter to select";
        let controls_size = 14.0;
        let controls_width = measure_text(controls_text, None, controls_size as u16, 1.0).width;
        let controls_x = (screen_width() - controls_width) / 2.0;
        draw_text(controls_text, controls_x, screen_height() - 60.0, controls_size, DARKGRAY);
    }

    fn draw_background_effects(&self) {
        let time = get_time() as f32;
        
        // Draw moving stars
        for i in 0..50 {
            let star_seed = i as f32 * 123.456;
            let x = ((time * 10.0 + star_seed).sin() * 0.5 + 0.5) * screen_width();
            let y = ((time * 15.0 + star_seed * 2.0).cos() * 0.5 + 0.5) * screen_height();
            let alpha = ((time * 3.0 + star_seed).sin() * 0.5 + 0.5) * 0.8;
            
            draw_circle(x, y, 1.0, Color::new(1.0, 1.0, 1.0, alpha));
        }
        
        // Draw grid lines
        let grid_spacing = 50.0;
        let grid_alpha = 0.1;
        let grid_color = Color::new(0.0, 0.5, 1.0, grid_alpha);
        
        // Vertical lines
        let mut x = 0.0;
        while x < screen_width() {
            draw_line(x, 0.0, x, screen_height(), 1.0, grid_color);
            x += grid_spacing;
        }
        
        // Horizontal lines
        let mut y = 0.0;
        while y < screen_height() {
            draw_line(0.0, y, screen_width(), y, 1.0, grid_color);
            y += grid_spacing;
        }
    }

    fn draw_menu_item(&self, item: &MenuItem, is_selected: bool, _menu_system: &MenuSystem) {
        let base_color = if item.is_enabled {
            if item.is_hovered || is_selected {
                Color::new(0.2, 0.7, 1.0, 0.8)
            } else {
                Color::new(0.1, 0.1, 0.3, 0.6)
            }
        } else {
            Color::new(0.3, 0.3, 0.3, 0.4)
        };
        
        let border_color = if item.is_hovered || is_selected {
            Color::new(0.0, 0.8, 1.0, 1.0)
        } else {
            Color::new(0.5, 0.5, 0.5, 0.6)
        };
        
        // Draw button background
        draw_rectangle(item.rect.x, item.rect.y, item.rect.w, item.rect.h, base_color);
        
        // Draw button border
        draw_rectangle_lines(item.rect.x, item.rect.y, item.rect.w, item.rect.h, 2.0, border_color);
        
        // Draw button text
        let text_color = if item.is_enabled {
            if item.is_hovered || is_selected { WHITE } else { LIGHTGRAY }
        } else {
            GRAY
        };
        
        let text_size = 20.0;
        let text_dimensions = measure_text(&item.text, None, text_size as u16, 1.0);
        let text_x = item.rect.x + (item.rect.w - text_dimensions.width) / 2.0;
        let text_y = item.rect.y + (item.rect.h + text_dimensions.height) / 2.0;
        
        draw_text(&item.text, text_x, text_y, text_size, text_color);
        
        // Draw selection indicator
        if is_selected {
            let indicator_x = item.rect.x - 20.0;
            let indicator_y = item.rect.y + item.rect.h / 2.0;
            draw_triangle(
                Vec2::new(indicator_x, indicator_y - 5.0),
                Vec2::new(indicator_x, indicator_y + 5.0),
                Vec2::new(indicator_x + 10.0, indicator_y),
                WHITE
            );
        }
    }
}

pub fn draw_main_menu(
    menu_system: &mut MenuSystem,
    game_state: &mut GameState,
    audio_manager: &AudioManager,
    resource_manager: &ResourceManager
) {
    // Initialize main menu if not already done
    if menu_system.main_menu.is_none() {
        menu_system.main_menu = Some(MainMenu::new());
    }
    
    if let Some(main_menu) = &mut menu_system.main_menu {
        let dt = get_frame_time();
        main_menu.update(dt);
        main_menu.handle_input(game_state, audio_manager, resource_manager);
        main_menu.draw(menu_system);
    }
}
