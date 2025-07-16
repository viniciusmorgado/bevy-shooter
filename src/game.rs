pub mod camera_controller;
pub mod cursor;
pub mod level;
pub mod player;
pub mod window;
use crate::game::{level::LevelPlugin, player::PlayerPlugin, window::WindowsSettingsPlugin};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowsSettingsPlugin)
            .add_plugins(LevelPlugin)
            .add_plugins(PlayerPlugin);
    }
}
