use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_players)
        .add_systems(Startup, spawn_camera)
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
