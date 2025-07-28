pub mod game;
pub mod layers;
pub mod utils;

#[cfg(feature = "editor_tools")]
mod editor_tools;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

#[cfg(feature = "editor_tools")]
fn editor_plugin(app: &mut App) {
    app.add_plugins(editor_tools::EditorPlugin::default());
}

#[cfg(not(feature = "editor_tools"))]
fn editor_plugin(app: &mut App) {}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins((editor_plugin, game::plugin))
        .run();
}
