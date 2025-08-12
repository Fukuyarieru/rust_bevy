mod app_states;
mod enemy;
mod events;
mod game;
mod imports;
mod main_menu;
mod player;
mod score;
mod settings;
mod star;

use crate::imports::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((GamePlugin, MainMenuPlugin, StatesPlugin))
        .run();
}
