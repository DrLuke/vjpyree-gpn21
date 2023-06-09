use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};


pub struct BeatControls;

impl Plugin for BeatControls {
    fn build(&self, app: &mut App) {

    }
}

#[derive(Component)]
pub struct TraktorBeat();

pub fn traktor_beat_system(

) {

}


pub fn beat_ui(
    mut contexts: EguiContexts,
) {
    egui::Window::new("Beat").show(contexts.ctx_mut(), |ui| {

    });
}