pub mod camera_controller;
pub mod crosshair;
pub mod cursor;
pub mod level;
pub mod player;
pub mod ui;
pub mod window;
use crate::game::{
    level::LevelPlugin, player::PlayerPlugin, ui::UiPlugin, window::WindowsSettingsPlugin,
};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, LevelPlugin, WindowsSettingsPlugin, UiPlugin));
    }
}
