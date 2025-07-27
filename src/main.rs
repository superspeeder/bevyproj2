pub mod utils;
pub mod game;
mod editor_tools;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins)
        .add_plugins(game::plugin)
        .run();
}
