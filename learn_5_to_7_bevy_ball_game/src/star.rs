use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::settings::*;

#[derive(Component)]
pub struct Star {}

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_stars)
            .add_systems(Update, (tick_star_spawn_timer, spawn_stars_over_time));
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
