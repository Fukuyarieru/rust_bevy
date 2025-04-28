use bevy::prelude::*;
use super::super::ENEMY_SPAWN_TIMER;


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