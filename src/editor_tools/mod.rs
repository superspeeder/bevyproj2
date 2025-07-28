#![cfg(feature = "editor_tools")]

use bevy::app::App;
use bevy::app::Update;
use bevy::ecs::event::EventReader;
use bevy::ecs::schedule::{IntoScheduleConfigs, SystemSet};
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::input::keyboard::KeyboardInput;
use bevy::log::info;
use bevy::prelude::Plugin;
use bevy::state::app::AppExtStates;
use bevy::state::condition::in_state;
use bevy::state::state::NextState;
use bevy::state::state::State;
use bevy::state::state::States;
use bevy_egui::EguiPlugin;
use bevy_egui::EguiPrimaryContextPass;

use crate::editor_tools::editor_overlay::{EditorLayer, editor_overlay};

mod editor_overlay;
mod uitools;

#[derive(Default)]
pub struct EditorPlugin {}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
pub struct EditorState(pub bool);

#[derive(SystemSet, Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct EditorUpdateSet;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(EditorState(false));
        app.init_state::<EditorLayer>();
        app.configure_sets(
            EguiPrimaryContextPass,
            EditorUpdateSet.run_if(in_state(EditorState(true))),
        );
        app.add_systems(Update, editor_state_update);
        app.add_systems(
            EguiPrimaryContextPass,
            editor_overlay.in_set(EditorUpdateSet),
        );
    }
}

fn editor_state_update(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<EditorState>>,
    mut next_state: ResMut<NextState<EditorState>>,
) {
    if keys.just_pressed(KeyCode::F2) {
        next_state.set(EditorState(!state.get().0));
        info!("Toggle debug");
    }
}
