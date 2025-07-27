use bevy::prelude::StateSet;
use bevy::prelude::{States, SubStates};
use super::GameState;

#[derive(SubStates, Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::MenuScreen)]
pub enum MenuScreen {
    #[default]
    Title,
}