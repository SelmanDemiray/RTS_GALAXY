use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameMode {
    Offline,
    Online,
}

// Re-export GameScreen from screens module
pub use super::screens::GameScreen;
