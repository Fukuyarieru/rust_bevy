use std::ops::Range;

// Player settings
pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_SCALE: f32 = 30.0;
pub const LOG_PLAYER_MOVEMENT: bool = false;
pub const LOG_COLLECTING_STARS: bool = true;

// Star settings
pub const STAR_SCALE: f32 = 30.0;
pub const NUMBER_OF_STARS_AT_STARTUP: usize = 10;
pub const STAR_SPAWN_TIME: f32 = 3.0;
pub const AMOUNT_OF_STARS_PER_SPAWN: usize = 2;

// Enemy settings
pub const ENEMY_SCALE: f32 = 45.0;
pub const PLAY_ENEMY_BOUNCE_SOUND: bool = true;
pub const NUMBER_OF_ENEMIES: usize = 10;
pub const ENEMY_SPEED_VARIATY: Range<f32> = (25.0)..(500.0);
pub const ENEMY_SPAWN_TIMER: f32 = 5.0;
pub const ENEMY_SPAWN_OVER_TIME: usize = 3;
