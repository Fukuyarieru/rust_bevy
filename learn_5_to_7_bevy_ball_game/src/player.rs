use bevy::{prelude::*, window::PrimaryWindow};

use crate::{score::Score, star::Star};

use crate::settings::*;

pub struct PlayerPlugin;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct MovementSystemSet;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct ConfinementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement),
        )
        .add_systems(Startup, spawn_players)
        .add_systems(
            Update,
            (
                player_movement.in_set(PlayerSystemSet::Movement),
                confine_player_movement.in_set(PlayerSystemSet::Confinement),
                player_hit_star,
            ),
        );
        // add_systems(Update, (player_movement,confine_player_movement).chain())
    }
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
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut directions = Vec3::new(0.0, 0.0, 0.0);

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            if LOG_PLAYER_MOVEMENT {
                println!("W/UP");
            }
            directions += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            if LOG_PLAYER_MOVEMENT {
                println!("A/LEFT");
            }
            directions += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            if LOG_PLAYER_MOVEMENT {
                println!("S/DOWN");
            }
            directions += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            if LOG_PLAYER_MOVEMENT {
                println!("D/RIGHT");
            }
            directions += Vec3::new(1.0, 0.0, 0.0);
        }
        if directions.length() > 0.0 {
            directions = directions.normalize();
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            if LOG_PLAYER_MOVEMENT {
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
pub fn player_hit_star(
    mut commands: Commands,
    assest_server: Res<AssetServer>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    player_query: Query<&Transform, With<Player>>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (star_entity, star_transform) in star_query {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            let player_radius = PLAYER_SCALE / 2.0;
            let star_radius = STAR_SCALE / 2.0;

            if distance < player_radius + star_radius {
                if LOG_COLLECTING_STARS {
                    println!("Player hit star");
                }
                score.value += 1;
                commands.spawn(AudioPlayer::new(
                    assest_server.load("audio/laserLarge_000.ogg"),
                ));
                commands.entity(star_entity).despawn();
            }
        }
    }
}
