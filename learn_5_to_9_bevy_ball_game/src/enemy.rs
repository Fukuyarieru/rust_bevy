use crate::imports::*;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub speed: f32,
}

#[derive(Resource)]
pub struct EnemySpawnTime {
    pub timer: Timer,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTime>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(
                Update,
                (
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                    enemy_movement,
                    confine_enemy_movement,
                    enemy_hit_player,
                    update_enemy_direction,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

impl Default for EnemySpawnTime {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIMER, TimerMode::Repeating),
        }
    }
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
