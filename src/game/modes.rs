#[derive(Clone, Debug, PartialEq, Copy)]
pub enum GameMode {
    Offline,
    Online,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum GameScreen {
    MainMenu,
    Playing,
    Settings,
    Credits,
}
