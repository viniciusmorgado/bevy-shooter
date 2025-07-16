use crate::game::crosshair;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, crosshair::spawn_crosshair);
    }
}
