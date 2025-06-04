use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use crate::resources::ResourceManager;
use crate::audio::AudioManager;

pub mod main_menu;
pub mod settings;
pub mod credits;
pub mod system;

pub use system::MenuSystem;
