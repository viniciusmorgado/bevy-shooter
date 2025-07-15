pub mod window;
use crate::game::window::WindowsSettingsPlugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowsSettingsPlugin);
    }
}
