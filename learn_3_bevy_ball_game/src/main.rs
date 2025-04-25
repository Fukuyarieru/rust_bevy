use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_players)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_players(
    mut commands: Commands,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    assest_server: Res<AssetServer>,
) {
    let window = windows_query.single().unwrap();

    commands.spawn((
        Sprite {
            image: assest_server.load("sprites/ball_blue_large.png"),
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

pub const PLAYER_SPEED: f32 = 500.0;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut directions = Vec3::new(0.0, 0.0, 0.0);

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            directions += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            directions += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            directions += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            directions += Vec3::new(1.0, 0.0, 0.0);
        }
        if directions.length() > 0.0 {
            directions = directions.normalize();
        }
        transform.translation += directions * PLAYER_SPEED * time.delta_secs();
    }
}
