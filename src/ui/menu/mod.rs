use macroquad::prelude::*;
use crate::game::state::GameScreen; // Fix import path

pub mod main_menu;
pub mod settings;
pub mod credits;
pub mod system;

pub use system::MenuSystem;
