pub mod camera_controller;
pub mod crosshair;
pub mod cursor;
pub mod level;
pub mod player;
pub mod tracer;
pub mod ui;
pub mod window;

use crate::game::{
    level::LevelPlugin, player::PlayerPlugin, ui::UiPlugin, window::WindowsSettingsPlugin,
};
use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            PlayerPlugin,
            LevelPlugin,
            WindowsSettingsPlugin,
            UiPlugin,
        ));
    }
}
