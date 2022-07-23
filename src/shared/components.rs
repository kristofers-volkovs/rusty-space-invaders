use bevy::prelude::*;

// Gameplay components

#[derive(Component)]
pub struct GameRunning;

#[derive(Component)]
pub struct GameplayTeardown;

#[derive(Component)]
pub struct ResetGameplay;

#[derive(Component)]
pub struct SpawnPlayer;

// Ui components

#[derive(Component)]
pub struct ExitGameButton;
