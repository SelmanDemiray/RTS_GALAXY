use super::asset_inspector::{AssetInspector, AssetStatus, AssetType, AnimationStatus};
use macroquad::prelude::*;

#[cfg(feature = "admin")]
use egui_macroquad::egui;

#[cfg(feature = "admin")]
use crate::game::state::GameState;

#[cfg(feature = "admin")]
pub struct AdminPanel {
    show_debug_window: bool,
    show_asset_inspector: bool,
}

#[cfg(feature = "admin")]
impl AdminPanel {
    pub fn new() -> Self {
        Self {
            show_debug_window: false,
            show_asset_inspector: false,
        }
    }
    
    pub fn update(&mut self, game_state: &mut GameState) {
        egui_macroquad::ui(|egui_ctx| {
            // Main menu bar
            egui::TopBottomPanel::top("admin_menu").show(egui_ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Debug", |ui| {
                        if ui.button("Game State").clicked() {
                            self.show_debug_window = !self.show_debug_window;
                        }
                        if ui.button("Asset Inspector").clicked() {
                            self.show_asset_inspector = !self.show_asset_inspector;
                        }
                    });
                });
            });
            
            // Debug windows
            if self.show_debug_window {
                self.draw_debug_window(egui_ctx, game_state);
            }
            
            if self.show_asset_inspector {
                self.draw_asset_inspector(egui_ctx);
            }
        });
    }
    
    pub fn draw(&self) {
        egui_macroquad::draw();
    }
    
    fn draw_debug_window(&mut self, ctx: &egui::Context, game_state: &GameState) {
        egui::Window::new("Game Debug")
            .open(&mut self.show_debug_window)
            .show(ctx, |ui| {
                ui.label(format!("Units: {}", game_state.units.len()));
                ui.label(format!("Players: {}", game_state.players.len()));
                ui.label(format!("Current Screen: {:?}", game_state.current_screen));
                
                ui.separator();
                
                ui.label("Resources:");
                for (i, player) in game_state.players.iter().enumerate() {
                    ui.label(format!("Player {}: {} minerals, {} energy", 
                             i, player.minerals, player.energy));
                }
            });
    }
    
    fn draw_asset_inspector(&mut self, ctx: &egui::Context) {
        egui::Window::new("Asset Inspector")
            .open(&mut self.show_asset_inspector)
            .show(ctx, |ui| {
                ui.label("Asset Inspector");
                ui.label("TODO: Implement asset browsing and inspection");
            });
    }
}

pub struct AdminUI {
    pub is_visible: bool,
}

impl AdminUI {
    pub fn new() -> Self {
        Self {
            is_visible: false,
        }
    }

    pub fn toggle(&mut self) {
        self.is_visible = !self.is_visible;
    }

    pub fn update(&mut self, _game_state: &GameState) {
        // Admin UI update logic
    }

    pub fn render(&self, _game_state: &GameState) {
        if !self.is_visible {
            return;
        }
        
        // Admin UI rendering logic
        use macroquad::prelude::*;
        
        draw_rectangle(10.0, 10.0, 300.0, 200.0, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_text("Admin Panel", 20.0, 30.0, 20.0, WHITE);
        draw_text("Press F1 to toggle", 20.0, 50.0, 16.0, GRAY);
    }
}
