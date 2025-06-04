pub mod state;
pub mod modes;
pub mod screens;
pub mod types;
pub mod commands;
pub mod resources;
pub mod rendering;
pub mod zoom; // Add zoom module

// Re-export commonly used types
pub use state::GameState;
pub use modes::{GameMode, GameScreen};
pub use zoom::ZoomSystem;
