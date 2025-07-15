mod game;
use crate::game::GamePlugin;
use bevy::prelude::*;

fn main() {
    App::new().add_plugins((GamePlugin, DefaultPlugins)).run();
}
