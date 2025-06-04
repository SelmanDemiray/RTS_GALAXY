use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameScreen {
    MainMenu,
    Playing,
    Settings,
    Credits,
    #[allow(dead_code)]
    Quit,
}
