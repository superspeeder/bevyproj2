mod menu;

use crate::game::menu::MenuScreen;
use bevy::app::App;
use bevy::prelude::{AppExtStates, States};

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    MenuScreen,
    InGame,
}

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>().add_sub_state::<MenuScreen>();
}
