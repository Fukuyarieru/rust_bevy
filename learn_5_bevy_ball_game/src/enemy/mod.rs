use bevy::app::Plugin;

pub mod components;
pub mod resources;
pub mod systems;

pub struct EnemyPlugin; 

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        
    }
}