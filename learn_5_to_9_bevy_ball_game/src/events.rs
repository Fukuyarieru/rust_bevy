use bevy::prelude::*;

#[derive(Event)]
pub(super) struct GameOver {
    pub score: u32,
}
