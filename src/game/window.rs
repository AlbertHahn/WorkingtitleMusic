use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui};

use crate::AppState;
// use bevy_egui::EguiPlugin;


pub fn create_musician_window(state: Res<State<AppState>>, mut contexts: EguiContexts){
    if state.get() != &AppState::Game {
        return
    }

    egui::Window::new("Musicians Window").show(contexts.ctx_mut(), |ui| {
        ui.label("Musicians");
    });
}


fn run_if_in_game(state: State<AppState>)-> bool {
    match state.get() == &AppState::Game {
        true => true,
        false => true,
    }
}