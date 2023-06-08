use bevy::prelude::*;

use crate::rd::wipes::{WipeEvent, WipeShape};
use crate::rd::RDShaderMaterial;
use bevy_egui::{egui, EguiContexts};
use rand::random;

pub fn ui_system(
    mut contexts: EguiContexts,
    mut mat_query: Query<&Handle<RDShaderMaterial>>,
    mut materials: ResMut<Assets<RDShaderMaterial>>,
    mut event_writer: EventWriter<WipeEvent>,
    mut local_event: Local<WipeEvent>,
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
                ui.add(
                    egui::DragValue::new(&mut mat.da)
                        .speed(0.01)
                        .max_decimals(2),
                );
                ui.end_row();

                ui.label("Db");
                ui.add(
                    egui::DragValue::new(&mut mat.db)
                        .speed(0.01)
                        .max_decimals(2),
                );
                ui.end_row();

                ui.label("Feed");
                ui.add(
                    egui::DragValue::new(&mut mat.feed)
                        .speed(0.001)
                        .max_decimals(4),
                );
                ui.end_row();

                ui.label("Kill");
                ui.add(
                    egui::DragValue::new(&mut mat.kill)
                        .speed(0.001)
                        .max_decimals(4),
                );
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

        ui.label("Wipes");

        /*let ev = WipeEvent {
            shape: WipeShape::Circle,
            duration: 0.4,
            steps: 7,
            start_size: 15.0,
            end_size: 35.0,
        };*/

        egui::Grid::new("wipe params")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Duration");
                ui.add(egui::Slider::new(&mut local_event.duration, 0.0..=1.0));
                ui.end_row();

                ui.label("Steps");
                ui.add(egui::Slider::new(&mut local_event.steps, 1..=10));
                ui.end_row();

                ui.label("Start size");
                ui.add(egui::Slider::new(&mut local_event.start_size, 1.0..=100.0));
                ui.end_row();

                ui.label("End size");
                ui.add(egui::Slider::new(&mut local_event.end_size, 1.0..=100.0));
                ui.end_row();
            });

        ui.horizontal(|ui| {
            if ui.button("Circle").clicked() {
                event_writer.send(WipeEvent {
                    shape: WipeShape::Circle,
                    ..*local_event
                })
            }
            if ui.button("Octagon").clicked() {
                event_writer.send(WipeEvent {
                    shape: WipeShape::Octagon,
                    ..*local_event
                })
            }
            if ui.button("Cross").clicked() {
                event_writer.send(WipeEvent {
                    shape: WipeShape::Cross,
                    ..*local_event
                })
            }
            if ui.button("Square").clicked() {
                event_writer.send(WipeEvent {
                    shape: WipeShape::Square,
                    ..*local_event
                })
            }
            if ui.button("Hexagram").clicked() {
                event_writer.send(WipeEvent {
                    shape: WipeShape::Hexagram,
                    ..*local_event
                })
            }
        });
        if ui.button("Random Shape").clicked() {
            event_writer.send(WipeEvent {
                shape: random(),
                ..*local_event
            })
        }
    });
}
