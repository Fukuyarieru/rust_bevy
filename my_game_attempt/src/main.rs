mod camera;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_obsteceles, spawn_player))
        .add_systems(Update, player_movement)
        .run();
}



#[derive(Component)]
pub struct Object;



pub fn spawn_obsteceles() {}

