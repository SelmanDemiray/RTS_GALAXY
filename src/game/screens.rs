#[derive(Debug, Clone, PartialEq)]
pub enum GameScreen {
    MainMenu,
    Playing,
    Settings,
    Credits,
    #[allow(dead_code)]
    Quit,
}
