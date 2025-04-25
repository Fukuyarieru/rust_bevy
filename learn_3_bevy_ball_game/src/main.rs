use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_SCALE: f32 = 30.0;
pub const ENEMY_SCALE: f32 = 45.0;
pub const LOG_MOVEMENT: bool = false;
pub const NUMBER_OF_ENEMIES: usize = 4;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_players)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Startup, spawn_enemies)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

pub fn spawn_players(
    mut commands: Commands,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    assest_server: Res<AssetServer>,
) {
    let window = windows_query.single().unwrap();

    commands.spawn((
        Sprite {
            image: assest_server.load("sprites/ball_blue_large.png"),
            custom_size: Some(Vec2 {
                x: PLAYER_SCALE,
                y: PLAYER_SCALE,
            }),
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, windows_query: Query<&Window, With<PrimaryWindow>>) {
    let window = windows_query.single().unwrap();

    commands.spawn((
        Camera2d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    assest_server: Res<AssetServer>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows_query.single().unwrap();

    (0..NUMBER_OF_ENEMIES).into_iter().for_each(|_| {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: assest_server.load("sprites/ball_red_large.png"),
                custom_size: Some(Vec2 {
                    x: ENEMY_SCALE,
                    y: ENEMY_SCALE,
                }),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 0.0),
            Enemy {},
        ));
    });
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut directions = Vec3::new(0.0, 0.0, 0.0);

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            if LOG_MOVEMENT {
                println!("W/UP");
            }
            directions += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            if LOG_MOVEMENT {
                println!("A/LEFT");
            }
            directions += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            if LOG_MOVEMENT {
                println!("S/DOWN");
            }
            directions += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            if LOG_MOVEMENT {
                println!("D/RIGHT");
            }
            directions += Vec3::new(1.0, 0.0, 0.0);
        }
        if directions.length() > 0.0 {
            directions = directions.normalize();
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            if LOG_MOVEMENT {
                println!("SHIFT");
            }
            directions = directions + directions;
        }

        transform.translation += directions * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        let window = windows_query.single().unwrap();

        let half_player_size = PLAYER_SCALE / 2.0;

        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        }
        if translation.y < y_min {
            translation.y = y_min;
        }
        if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}
