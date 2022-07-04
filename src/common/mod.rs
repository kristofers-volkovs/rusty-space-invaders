pub mod constants;

// Game states

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Gameplay,
    Paused,
    GameOver,
}

