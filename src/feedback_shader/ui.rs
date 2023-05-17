use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts};
use crate::feedback_shader::FeedbackShaderMaterial;

pub fn ui_system(
    mut contexts: EguiContexts,
    mut mat_query: Query<&Handle<FeedbackShaderMaterial>>,
    mut materials: ResMut<Assets<FeedbackShaderMaterial>>,
) {

    let mat_handle = mat_query.get_single_mut().unwrap();
    let mut mat = materials.get_mut(mat_handle).unwrap();

    egui::Window::new("Feedback Shader").show(contexts.ctx_mut(), |ui| {
        egui::Grid::new("my_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {

                ui.label("Axis");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut mat.col_rot.x).speed(0.01).max_decimals(2));
                    ui.add(egui::DragValue::new(&mut mat.col_rot.y).speed(0.01).max_decimals(2));
                    ui.add(egui::DragValue::new(&mut mat.col_rot.z).speed(0.01).max_decimals(2));
                });
                ui.end_row();

                ui.label("Amount");
                ui.add(egui::DragValue::new(&mut mat.col_rot.w).speed(0.01).max_decimals(2));
                ui.end_row();
            });
    });
}