use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    velocity: Vec2,
    rotation: u32,
}
pub fn spawn_player() {}

pub fn player_movement(
    players: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(schedule, systems)
    }
}