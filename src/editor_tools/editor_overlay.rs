use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiContexts, PrimaryEguiContext};

use crate::editor_tools::uitools::EditorToolsExt;

#[derive(Hash, PartialEq, Eq, Debug, Default, Clone, Copy, States)]
pub enum EditorLayer {
    #[default]
    None,
    UI,
    Game,
}

pub(super) fn editor_overlay(
    mut contexts: EguiContexts,
    active_overlay: Res<State<EditorLayer>>,
    mut next_active_overlay: ResMut<NextState<EditorLayer>>,
) -> Result {
    info!("Egui");

    egui::Window::new("Editor").show(contexts.ctx_mut()?, |ui| {
        ui.horizontal(|ui| {
            ui.label("Active Editor");
            ui.selectable_value_state(
                active_overlay.as_ref(),
                EditorLayer::None,
                next_active_overlay.as_mut(),
                "None",
            );
            ui.selectable_value_state(
                active_overlay.as_ref(),
                EditorLayer::UI,
                next_active_overlay.as_mut(),
                "UI",
            );
            ui.selectable_value_state(
                active_overlay.as_ref(),
                EditorLayer::Game,
                next_active_overlay.as_mut(),
                "Game",
            );
        })
    });

    Ok(())
}
