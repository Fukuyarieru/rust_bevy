use std::ops::Range;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_SCALE: f32 = 30.0;
pub const ENEMY_SCALE: f32 = 25.0;
pub const LOG_MOVEMENT: bool = false;
pub const PLAY_ENEMY_BOUNCE_SOUND: bool = true;
pub const NUMBER_OF_ENEMIES: usize = 10;
pub const ENEMY_SPEED_VARIATY: Range<f32> = (25.0)..(500.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_players)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub speed: f32,
}

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
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                speed: rand::random_range(ENEMY_SPEED_VARIATY),
            },
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

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transfrom, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transfrom.translation += direction * enemy.speed * time.delta_secs();
    }
}
pub fn update_enemy_direction(
    enemy_query: Query<(&Transform, &mut Enemy)>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    assest_server: Res<AssetServer>,
) {
    if let Ok(window) = windows_query.single() {
        let half_enemy_scale = ENEMY_SCALE / 2.0;

        let x_min = half_enemy_scale;
        let x_max = window.width() - half_enemy_scale;
        let y_min = half_enemy_scale;
        let y_max = window.height() - half_enemy_scale;

        for (transfrom, mut enemy) in enemy_query {
            if transfrom.translation.x <= x_min || transfrom.translation.x >= x_max {
                if PLAY_ENEMY_BOUNCE_SOUND {
                    commands.spawn(AudioPlayer::new(assest_server.load("audio/pluck_001.ogg")));
                }
                enemy.direction.x *= -1.0;
            }
            if transfrom.translation.y <= y_min || transfrom.translation.y >= y_max {
                if PLAY_ENEMY_BOUNCE_SOUND {
                    commands.spawn(AudioPlayer::new(assest_server.load("audio/pluck_001.ogg")));
                }
                enemy.direction.y *= -1.0;
            }
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    windw_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = windw_query.single() {
        let half_enemy_scale = ENEMY_SCALE / 2.0;

        let x_min = half_enemy_scale;
        let x_max = window.width() - half_enemy_scale;
        let y_min = half_enemy_scale;
        let y_max = window.height() - half_enemy_scale;

        for mut transform in enemy_query.iter_mut() {
            if transform.translation.x > x_max {
                transform.translation.x = x_max;
            } else if transform.translation.x < x_min {
                transform.translation.x = x_min;
            }
            if transform.translation.y > y_max {
                transform.translation.y = y_max;
            } else if transform.translation.y < y_min {
                transform.translation.y = y_min;
            }
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    assest_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        for enemy_tranform in enemy_query {
            let distance = player_transform
                .translation
                .distance(enemy_tranform.translation);

            let player_radius = PLAYER_SCALE / 2.0;
            let enemy_radius = ENEMY_SCALE / 2.0;

            if distance < player_radius + enemy_radius {
                commands.spawn(AudioPlayer::new(
                    assest_server.load("audio/explosionCrunch_000.ogg"),
                ));
                commands.entity(player_entity).despawn();
            }
        }
    }
}
