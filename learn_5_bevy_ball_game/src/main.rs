mod star;
mod score;
mod events;
mod player;
mod enemy;
mod settings;

use events::*;
use star::*;
use score::*;
use player::*;
use enemy::*;


use std::ops::Range;

use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};


pub const ENEMY_SCALE: f32 = 45.0;
pub const PLAY_ENEMY_BOUNCE_SOUND: bool = true;
pub const NUMBER_OF_ENEMIES: usize = 10;
pub const ENEMY_SPEED_VARIATY: Range<f32> = (25.0)..(500.0);

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
