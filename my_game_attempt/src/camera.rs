use bevy::prelude::*;

pub fn spawn_camera() {}
pub fn move_camera() {}
pub fn camera_follow_player(cfp: Res<CameraFollowPlayer>) {}

#[derive(Resource)]
pub struct CameraFollowPlayer(pub bool);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, move_camera);
    }
}
