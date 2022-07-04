use bevy::prelude::*;

// Asset Constants

pub const PLAYER_HEART_FULL: &str = "hud_heartFull.png";
pub const PLAYER_HEART_EMPTY: &str = "hud_heartEmpty.png";

pub const GAME_FONT: &str = "MinimalPixel v2.ttf";

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
