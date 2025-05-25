mod enemy;
mod events;
mod player;
mod score;
mod settings;
mod star;

use enemy::*;
use events::*;
use player::*;
use score::*;
use star::*;

use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

// pub const ENEMY_SCALE: f32 = 45.0;
// pub const PLAY_ENEMY_BOUNCE_SOUND: bool = true;
// pub const NUMBER_OF_ENEMIES: usize = 10;
// pub const ENEMY_SPEED_VARIATY: Range<f32> = (25.0)..(500.0);

// pub const ENEMY_SPAWN_TIMER: f32 = 5.0;
// pub const ENEMY_SPAWN_OVER_TIME: usize = 3;

fn main() {
    App::new()
        .add_plugins((
            GamePlugin,
            DefaultPlugins,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
            EnemyPlugin,
        ))
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (exit_game, handle_game_over));
    }
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
