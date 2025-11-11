use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub left: u32,
    pub right: u32,
}

#[derive(Resource)]
pub struct PuckDelayTimer(pub Timer);

#[derive(Resource, Default, PartialEq)]
pub struct GamePaused(pub bool);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    StartScreen,
    Countdown,
    Playing,
}

#[derive(Resource)]
pub struct CountdownTimer(pub Timer, pub u32); // Timer and current count