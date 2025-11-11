use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;

// Setup camera - runs once at startup
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Start Screen Systems
pub fn setup_start_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Background
    commands.spawn((
        Sprite {
            image: asset_server.load("rink.png"),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
        StartScreenUI,
    ));

    // Start Screen UI
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        StartScreenUI,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("PONG HOCKEY"),
            TextFont {
                font_size: 72.0,
                ..default()
            },
            TextColor(Color::BLACK),
            Node {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            },
        ));
        
        // Play Button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.8)),
            PlayButton,
        )).with_children(|parent| {
            parent.spawn((
                Text::new("PLAY"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
        
        // Controls instruction
        parent.spawn((
            Text::new("Left Player: W/S\nRight Player: Arrow Keys\nESC: Pause"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::BLACK),
            Node {
                margin: UiRect::top(Val::Px(50.0)),
                ..default()
            },
        ));
    });
}

pub fn handle_start_screen_input(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Countdown);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.85).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.2, 0.2, 0.8).into();
            }
        }
    }
}

pub fn cleanup_start_screen(
    mut commands: Commands,
    query: Query<Entity, With<StartScreenUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// Countdown Systems
pub fn setup_countdown(mut commands: Commands) {
    commands.insert_resource(CountdownTimer(Timer::from_seconds(1.0, TimerMode::Repeating), 3));
    
    // Countdown UI
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        CountdownText,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("3"),
            TextFont {
                font_size: 128.0,
                ..default()
            },
            TextColor(Color::BLACK),
            CountdownText,
        ));
    });
}

pub fn update_countdown(
    commands: Commands,
    mut countdown_timer: ResMut<CountdownTimer>,
    mut query: Query<&mut Text, With<CountdownText>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    countdown_timer.0.tick(time.delta());
    
    if countdown_timer.0.just_finished() {
        countdown_timer.1 -= 1;
        
        if countdown_timer.1 == 0 {
            next_state.set(GameState::Playing);
        } else {
            // Update countdown display
            for mut text in &mut query {
                *text = Text::new(countdown_timer.1.to_string());
            }
        }
    }
}

pub fn cleanup_countdown(
    mut commands: Commands,
    query: Query<Entity, With<CountdownText>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<CountdownTimer>();
}

// Game Systems
pub fn setup_game(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn background
    commands.spawn((
        Sprite {
            image: asset_server.load("rink.png"),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
        GameUI,
    ));
    
    // left paddle (circular)
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(PADDLE_RADIUS))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))), // Red
        Transform::from_xyz(LEFT_PADDLE_X, 0.0, 0.0),
        Paddle { side: Side::Left },
        GameUI,
    ));

    // right paddle (circular)
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(PADDLE_RADIUS))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))), // Blue
        Transform::from_xyz(RIGHT_PADDLE_X, 0.0, 0.0),
        Paddle { side: Side::Right },
        GameUI,
    ));

    // puck
    let initial_direction = if rand::thread_rng().gen_bool(0.5) { 1.0 } else { -1.0 };
    commands.spawn((
        Sprite {
            image: asset_server.load("puck.png"),
            custom_size: Some(Vec2::new(PUCK_SIZE, PUCK_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Puck,
        Velocity(Vec2::new(PUCK_SPEED * initial_direction, 0.0)),
        GameUI,
    ));

    // UI Root - Full screen container
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            ..default()
        },
        GameUI,
    )).with_children(|parent| {
        // Score display
        parent.spawn((
            Text::new("0 - 0"),
            TextFont {
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::BLACK),
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            ScoreText,
        ));
        
        // Pause display
        parent.spawn((
            Text::new(""),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor(Color::BLACK),
            Node {
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
            PauseText,
        ));
    });
}

pub fn handle_pause_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paused: ResMut<GamePaused>,
    mut timer: ResMut<PuckDelayTimer>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        paused.0 = !paused.0;
    }
    
    // Only tick the timer when the game is not paused
    if !paused.0 {
        timer.0.tick(time.delta());
    }
}

pub fn move_paddles(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Paddle)>,
    time: Res<Time>,
) {
    for (mut transform, paddle) in &mut query {
        let mut direction = 0.0;

        // Controls
        if paddle.side == Side::Left {
            if keyboard_input.pressed(KeyCode::KeyW) {
                direction += 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                direction -= 1.0;
            }
        } else {
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                direction += 1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                direction -= 1.0;
            }
        }

        transform.translation.y += direction * PADDLE_SPEED * time.delta_secs();

        // Keep paddles inside window (using radius for circular paddles)
        let half_height = WINDOW_HEIGHT / 2.0 - PADDLE_RADIUS;
        transform.translation.y = transform.translation.y.clamp(-half_height, half_height);
    }
}

pub fn move_puck(
    mut query: Query<(&mut Transform, &mut Velocity), With<Puck>>,
    time: Res<Time>,
    timer: Res<PuckDelayTimer>,
) {
    // Only move puck if delay is finished
    if !timer.0.is_finished() {
        return;
    }

    for (mut transform, mut velocity) in &mut query {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();

        // Bounce off top and bottom with position correction
        let boundary = WINDOW_HEIGHT / 2.0 - PUCK_SIZE / 2.0;
        if transform.translation.y > boundary {
            // Hit top wall
            transform.translation.y = boundary; // Correct position
            velocity.0.y = -velocity.0.y.abs(); // Ensure velocity points downward
        } else if transform.translation.y < -boundary {
            // Hit bottom wall
            transform.translation.y = -boundary; // Correct position
            velocity.0.y = velocity.0.y.abs(); // Ensure velocity points upward
        }
    }
}

pub fn check_paddle_collision(
    mut puck_query: Query<(&Transform, &mut Velocity), With<Puck>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Puck>)>,
    timer: Res<PuckDelayTimer>,
) {
    // Only check collisions if the delay timer has finished
    if !timer.0.is_finished() {
        return;
    }

    if let Ok((puck_transform, mut puck_velocity)) = puck_query.single_mut() {
        for paddle_transform in &paddle_query {
            let paddle_pos = paddle_transform.translation;
            let puck_pos = puck_transform.translation;

            // Calculate distance between centers (circle-circle collision)
            let distance = puck_pos.xy().distance(paddle_pos.xy());
            let collision_distance = PADDLE_RADIUS + PUCK_SIZE / 2.0;

            if distance < collision_distance && distance > 0.0 {
                // Calculate collision normal (direction from paddle center to puck center)
                let collision_normal = (puck_pos.xy() - paddle_pos.xy()).normalize();
                
                // Calculate relative velocity in collision normal direction
                let velocity_along_normal = puck_velocity.0.dot(collision_normal);
                
                // Only resolve collision if objects are moving toward each other
                if velocity_along_normal < 0.0 {
                    // Reflect velocity along the collision normal (realistic circular collision)
                    puck_velocity.0 -= 2.0 * velocity_along_normal * collision_normal;
                    
                    // Maintain constant speed (like real air hockey)
                    let current_speed = puck_velocity.0.length();
                    if current_speed > 0.0 {
                        puck_velocity.0 = puck_velocity.0.normalize() * PUCK_SPEED;
                    }
                }
            }
        }
    }
}

pub fn check_score(
    mut puck_query: Query<&mut Transform, With<Puck>>,
    mut velocity_query: Query<&mut Velocity, With<Puck>>,
    mut paddle_query: Query<(&mut Transform, &Paddle), Without<Puck>>,
    mut score: ResMut<Score>,
    mut timer: ResMut<PuckDelayTimer>,
) {
    if let (Ok(mut puck_transform), Ok(mut velocity)) = (puck_query.single_mut(), velocity_query.single_mut()) {
        let mut scored = false;

        if puck_transform.translation.x < -WINDOW_WIDTH / 2.0 - 50.0 {
            score.right += 1;
            scored = true;
            velocity.0 = Vec2::new(PUCK_SPEED, 0.0); // Right player scored, shoot left
        } else if puck_transform.translation.x > WINDOW_WIDTH / 2.0 + 50.0 {
            score.left += 1;
            scored = true;
            velocity.0 = Vec2::new(-PUCK_SPEED, 0.0); // Left player scored, shoot right
        }

        if scored {
            // Reset puck position to center
            puck_transform.translation = Vec3::ZERO;

            // Reset paddles to center position
            for (mut paddle_transform, paddle) in &mut paddle_query {
                match paddle.side {
                    Side::Left => {
                        paddle_transform.translation = Vec3::new(LEFT_PADDLE_X, 0.0, 0.0);
                    }
                    Side::Right => {
                        paddle_transform.translation = Vec3::new(RIGHT_PADDLE_X, 0.0, 0.0);
                    }
                }
            }

            // Start 2 second timer
            timer.0 = Timer::from_seconds(2.0, TimerMode::Once);
        }
    }
}

pub fn update_score_display(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if score.is_changed() {
        for mut text in &mut query {
            *text = Text::new(format!("{} - {}", score.left, score.right));
        }
    }
}

pub fn update_pause_display(
    paused: Res<GamePaused>,
    mut commands: Commands,
    pause_ui_query: Query<Entity, With<PauseScreenUI>>,
    mut text_query: Query<&mut Text, With<PauseText>>,
) {
    if paused.is_changed() {
        if paused.0 {
            // Game just got paused - hide the regular pause text and show pause screen
            for mut text in &mut text_query {
                *text = Text::new("");
            }
            
            // Create pause screen UI
            commands.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                PauseScreenUI,
            )).with_children(|parent| {
                // Pause title
                parent.spawn((
                    Text::new("PAUSED"),
                    TextFont {
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect::bottom(Val::Px(40.0)),
                        ..default()
                    },
                ));
                
                // Resume Button
                parent.spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                    ResumeButton,
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("RESUME"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
                
                // Restart Button
                parent.spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                    RestartButton,
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("RESTART"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            });
        } else {
            // Game just got unpaused - remove pause screen
            for entity in &pause_ui_query {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn handle_pause_screen_input(
    mut resume_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    mut restart_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RestartButton>, Without<ResumeButton>),
    >,
    mut paused: ResMut<GamePaused>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    game_entities: Query<Entity, With<GameUI>>,
    pause_ui_query: Query<Entity, With<PauseScreenUI>>,
    mut score: ResMut<Score>,
    mut puck_timer: ResMut<PuckDelayTimer>,
) {
    // Handle Resume Button
    for (interaction, mut color) in &mut resume_button_query {
        match *interaction {
            Interaction::Pressed => {
                paused.0 = false;
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.65, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.2, 0.6, 0.2).into();
            }
        }
    }
    
    // Handle Restart Button
    for (interaction, mut color) in &mut restart_button_query {
        match *interaction {
            Interaction::Pressed => {
                // Reset score
                score.left = 0;
                score.right = 0;
                
                // Clean up game entities
                for entity in &game_entities {
                    commands.entity(entity).despawn();
                }
                
                // Clean up pause screen entities
                for entity in &pause_ui_query {
                    commands.entity(entity).despawn();
                }
                
                // Reset puck delay timer to add delay at start
                puck_timer.0 = Timer::from_seconds(2.0, TimerMode::Once);
                
                // Unpause and go to countdown
                paused.0 = false;
                next_state.set(GameState::Countdown);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.85, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.8, 0.2, 0.2).into();
            }
        }
    }
}