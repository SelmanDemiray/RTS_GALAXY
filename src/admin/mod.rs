pub mod ui;
pub mod asset_inspector;

pub use ui::AdminPanel;

use macroquad::prelude::*;

use crate::game::state::GameState;

use crate::resources::manager::ResourceManager;

pub struct AdminSystem {
    asset_inspector: asset_inspector::AssetInspector,
    ui: ui::AdminUI,
    visible: bool,
    focused: bool,
}

impl AdminSystem {
    pub fn new() -> Self {
        Self {
            asset_inspector: asset_inspector::AssetInspector::new(),
            ui: ui::AdminUI::new(),
            visible: false,
            focused: false,
        }
    }
    
    pub fn update(&mut self, game_state: &GameState) {
        if self.visible {
            self.asset_inspector.update(game_state, &Default::default());
            self.ui.update(game_state);
        }
    }
    
    pub fn render(&mut self, game_state: &GameState) {
        if self.visible {
            self.ui.render(game_state);
            self.asset_inspector.render();
        }
    }
    
    pub fn draw_overlay(&mut self) {
        if self.visible && !self.focused {
            // Draw minimal overlay when not focused
            draw_text("Press F1 to toggle Admin Panel", 10.0, 30.0, 20.0, WHITE);
        }
    }
    
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    pub fn is_focused(&self) -> bool {
        self.focused
    }
}
