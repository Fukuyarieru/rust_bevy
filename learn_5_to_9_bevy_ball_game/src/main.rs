// mod enemy;
// mod events;
// mod game;
// mod player;
// mod score;
// mod settings;
// mod star;
//
mod enemy;
mod events;
mod game;
mod player;
mod prelude;
mod score;
mod settings;
mod star;

use crate::prelude::*;
use bevy::prelude::*;

// use events::*;
// use game::*;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
