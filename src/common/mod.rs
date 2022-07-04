pub mod constants;

// Game states

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Loading,
    Gameplay,
    Paused,
    GameOver,
}

