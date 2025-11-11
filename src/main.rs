use bevy::prelude::*;

mod components;
mod resources;
mod constants;
mod systems;

use components::*;
use resources::*;
use constants::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Air Hockey".into(),
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(Score { left: 0, right: 0 })
        .insert_resource(PuckDelayTimer(Timer::from_seconds(2.0, TimerMode::Once)))
        .insert_resource(GamePaused(false))
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::StartScreen), setup_start_screen)
        .add_systems(Update, (
            handle_start_screen_input,
        ).run_if(in_state(GameState::StartScreen)))
        .add_systems(OnEnter(GameState::Countdown), (
            cleanup_start_screen,
            setup_countdown,
        ))
        .add_systems(Update, (
            update_countdown,
        ).run_if(in_state(GameState::Countdown)))
        .add_systems(OnEnter(GameState::Playing), (
            cleanup_countdown,
            setup_game,
        ))
        .add_systems(Update, (
            handle_pause_input,
            handle_pause_screen_input,
            move_paddles.run_if(resource_equals(GamePaused(false))),
            move_puck.run_if(resource_equals(GamePaused(false))),
            check_paddle_collision.run_if(resource_equals(GamePaused(false))),
            check_score.run_if(resource_equals(GamePaused(false))),
            update_score_display,
            update_pause_display,
        ).run_if(in_state(GameState::Playing)))
        .run();
}