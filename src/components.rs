use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle {
    pub side: Side,
}

#[derive(Component)]
pub struct Puck;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct PauseText;

#[derive(Component)]
pub struct StartScreenUI;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct CountdownText;

#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct PauseScreenUI;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct ResumeButton;

#[derive(PartialEq)]
pub enum Side {
    Left,
    Right,
}