use std::ops::Range;

use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use rand::random;

pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_SCALE: f32 = 30.0;
pub const ENEMY_SCALE: f32 = 45.0;
pub const LOG_MOVEMENT: bool = false;
pub const PLAY_ENEMY_BOUNCE_SOUND: bool = true;
pub const NUMBER_OF_ENEMIES: usize = 10;
pub const ENEMY_SPEED_VARIATY: Range<f32> = (25.0)..(500.0);
pub const STAR_SCALE: f32 = 30.0;
pub const LOG_COLLECTING_STARS: bool = true;
pub const NUMBER_OF_STARS_AT_STARTUP: usize = 10;
pub const STAR_SPAWN_TIME: f32 = 3.0;
pub const AMOUNT_OF_STARS_PER_SPAWN: usize = 2;
pub const ENEMY_SPAWN_TIMER: f32 = 5.0;
pub const ENEMY_SPAWN_OVER_TIME: usize = 3;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTime>()
        .init_resource::<HighScore>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_players)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, player_hit_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, (spawn_enemies_over_time, tick_enemy_spawn_timer))
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, update_high_scores)
        .add_systems(Update, high_scores_updated)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub speed: f32,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Debug)]
pub struct HighScore {
    scores: Vec<(String, u32)>,
}

impl Default for HighScore {
    fn default() -> Self {
        Self { scores: Vec::new() }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}
impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTime {
    pub timer: Timer,
}
impl Default for EnemySpawnTime {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIMER, TimerMode::Repeating),
        }
    }
}

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
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

pub fn spawn_stars(
    mut commands: Commands,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    assest_server: Res<AssetServer>,
) {
    let window = windows_query.single().unwrap();

    (0..NUMBER_OF_STARS_AT_STARTUP).into_iter().for_each(|_| {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: assest_server.load("sprites/star.png"),
                custom_size: Some(Vec2 {
                    x: STAR_SCALE,
                    y: STAR_SCALE,
                }),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 0.0),
            Star {},
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
    mut game_over_event_writter: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    assest_server: Res<AssetServer>,
    score: Res<Score>,
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

                game_over_event_writter.write(GameOver { score: score.value });
            }
        }
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

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    star_spawn_timer: Res<StarSpawnTimer>,
    assest_server: Res<AssetServer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = windows_query.single().unwrap();

        commands.spawn(AudioPlayer::new(assest_server.load("audio/select_001.ogg")));

        (0..AMOUNT_OF_STARS_PER_SPAWN).for_each(|_| {
            let random_x = random::<f32>() * window.width();
            let random_y = random::<f32>() * window.height();

            commands.spawn((
                Sprite {
                    image: assest_server.load("sprites/star.png"),
                    custom_size: Some(Vec2 {
                        x: STAR_SCALE,
                        y: STAR_SCALE,
                    }),
                    ..default()
                },
                Transform::from_xyz(random_x, random_y, 0.0),
                Star {},
            ));
        })
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTime>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}
pub fn spawn_enemies_over_time(
    mut commands: Commands,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    enemy_spawn_timer: Res<EnemySpawnTime>,
    assest_server: Res<AssetServer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = windows_query.single().unwrap();

        commands.spawn(AudioPlayer::new(assest_server.load("audio/drop_001.ogg")));
        (0..ENEMY_SPAWN_OVER_TIME).into_iter().for_each(|_| {
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
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.write(AppExit::Success);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Your final score is {}", event.score)
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScore>,
) {
    for event in game_over_event_reader.read() {
        high_scores.scores.push(("Player".to_string(), event.score))
    }
}
pub fn high_scores_updated(high_scores: Res<HighScore>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores)
    }
}
