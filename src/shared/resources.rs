use bevy::prelude::*;

// Resources

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

pub struct UiTextures {
    pub heart_full: Handle<Image>,
    pub heart_empty: Handle<Image>,
    pub ui_font: Handle<Font>,
}

// Game states

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Gameplay,
    Paused,
    GameOver,
}
