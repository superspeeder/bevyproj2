use bevy::state::state::{FreelyMutableState, NextState, State, States};
use bevy_egui::egui::{self, WidgetText};

pub trait EditorToolsExt {
    fn selectable_value_state<T: States + FreelyMutableState + Clone + Eq>(&mut self, current_value: &State<T>, selected_value: T, next_value: &mut NextState<T>, label: impl Into<WidgetText>) -> egui::Response;
}

impl EditorToolsExt for egui::Ui {
    fn selectable_value_state<T: States + FreelyMutableState + Clone + Eq>(&mut self, current_value: &State<T>, selected_value: T, next_value: &mut NextState<T>, label: impl Into<WidgetText>) -> egui::Response {
        let response = self.selectable_label(current_value.get() == &selected_value, label);
        if response.clicked() {
            next_value.set(selected_value);
        }
        return response;
    }
}
