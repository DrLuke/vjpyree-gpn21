use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts};
use crate::rd::RDShaderMaterial;

pub fn ui_system(
    mut contexts: EguiContexts,
    mut mat_query: Query<&Handle<RDShaderMaterial>>,
    mut materials: ResMut<Assets<RDShaderMaterial>>,
) {

    let mat_handle = mat_query.get_single_mut().unwrap();
    let mut mat = materials.get_mut(mat_handle).unwrap();

    egui::Window::new("RD Shader").show(contexts.ctx_mut(), |ui| {
        egui::Grid::new("rd params")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {

                ui.label("Da");
                ui.add(egui::DragValue::new(&mut mat.da).speed(0.01).max_decimals(2));
                ui.end_row();

                ui.label("Db");
                ui.add(egui::DragValue::new(&mut mat.db).speed(0.01).max_decimals(2));
                ui.end_row();

                ui.label("Feed");
                ui.add(egui::DragValue::new(&mut mat.feed).speed(0.01).max_decimals(4));
                ui.end_row();

                ui.label("Kill");
                ui.add(egui::DragValue::new(&mut mat.kill).speed(0.01).max_decimals(4));
                ui.end_row();
            });

        ui.separator();

        ui.label("Presets");
        if ui.button("Rings").clicked() {
            mat.da = 1.;
            mat.db = 0.3;
            mat.feed = 0.0287;
            mat.kill = 0.078;
        }

        ui.separator();




    });
}