use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode, WindowResolution},
};

use crate::game::cursor::CursorPlugin;

pub struct WindowsSettingsPlugin;
impl Plugin for WindowsSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CursorPlugin)
            .add_systems(PreStartup, init_window);
    }
}

fn init_window(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.single_mut() {
        window.resolution = WindowResolution::new(1920., 1080.);
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
    }
}
