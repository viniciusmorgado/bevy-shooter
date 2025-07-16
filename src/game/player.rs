use crate::game::camera_controller::{self, update_camera_controller};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera_controller);
        app.add_systems(Startup, init_player);
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(mut commands: Commands) {
    let fov: f32 = 103.0_f32.to_radians();
    commands.spawn((
        Player {},
        Transform::from_translation(Vec3::new(0., 10., 0.)),
        Projection::Perspective(PerspectiveProjection {
            fov: fov,
            ..default()
        }),
        Camera3d { ..default() },
        camera_controller::CameraController {
            sensitivity: 0.035,
            rotation: Vec2::ZERO,
            rotation_lock: 88.0,
        },
    ));
}
