use macroquad::prelude::*;
use crate::game::state::{GameState, GameScreen};

pub struct MenuSystem {
    pub current_menu: GameScreen,
    pub is_visible: bool,
    pub selected_button: usize,
}

impl MenuSystem {
    pub fn new() -> Self {
        Self {
            current_menu: GameScreen::MainMenu,
            is_visible: true,
            selected_button: 0,
        }
    }

    pub fn reset(&mut self) {
        self.selected_button = 0;
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn toggle(&mut self) {
        self.is_visible = !self.is_visible;
    }

    pub fn set_menu(&mut self, menu: GameScreen) {
        self.current_menu = menu;
    }

    pub fn is_current_menu(&self, menu: GameScreen) -> bool {
        self.current_menu == menu
    }

    pub fn is_button_selected(&self, button_index: usize) -> bool {
        match self.current_menu {
            GameScreen::Playing => self.selected_button == 0,
            GameScreen::Settings => self.selected_button == 1,
            GameScreen::Credits => self.selected_button == 2,
            GameScreen::Quit => self.selected_button == 3,
            _ => false,
        }
    }

    pub fn handle_input(&mut self, game_state: &mut GameState) -> bool {
        if !self.is_visible {
            return false;
        }

        let mut input_handled = false;

        // Handle keyboard navigation
        if is_key_pressed(KeyCode::Up) {
            self.selected_button = if self.selected_button == 0 { 3 } else { self.selected_button - 1 };
            input_handled = true;
        } else if is_key_pressed(KeyCode::Down) {
            self.selected_button = (self.selected_button + 1) % 4;
            input_handled = true;
        } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            match self.selected_button {
                0 => game_state.current_screen = GameScreen::Playing,
                1 => game_state.current_screen = GameScreen::Settings,
                2 => game_state.current_screen = GameScreen::Credits,
                3 => game_state.request_quit(),
                _ => {}
            }
            input_handled = true;
        }

        // Handle escape key to return to main menu from sub-menus
        if is_key_pressed(KeyCode::Escape) {
            match game_state.current_screen {
                GameScreen::Settings => {
                    game_state.current_screen = GameScreen::MainMenu;
                    self.selected_button = 1; // Return to Settings button
                    input_handled = true;
                }
                GameScreen::Credits => {
                    game_state.current_screen = GameScreen::MainMenu;
                    self.selected_button = 2; // Return to Credits button
                    input_handled = true;
                }
                _ => {}
            }
        }

        input_handled
    }

    pub fn render(&self, game_state: &GameState) {
        if !self.is_visible {
            return;
        }

        match game_state.current_screen {
            GameScreen::MainMenu => {
                // Main menu will be drawn by main_menu module
            }
            GameScreen::Settings => {
                super::settings::draw(self, game_state);
            }
            GameScreen::Credits => {
                super::credits::draw(self, game_state);
            }
            _ => {}
        }
    }
}
