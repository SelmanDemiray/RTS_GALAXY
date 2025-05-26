use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use crate::resources::ResourceManager;
use crate::audio::AudioManager;

pub mod system;
pub mod main_menu;
pub mod settings;

pub use system::MenuSystem;
