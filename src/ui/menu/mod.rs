use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use crate::resources::ResourceManager;
use crate::audio::AudioManager;

pub mod main_menu;
pub mod settings;
pub mod credits;
pub mod system;

pub use system::MenuSystem;
pub use main_menu::draw_main_menu;
pub use settings::draw_settings;
pub use credits::draw_credits;
