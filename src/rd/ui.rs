use std::ops::Range;
use bevy::prelude::*;

use crate::rd::wipes::{WipeEvent, WipeShape};
use crate::rd::RDShaderMaterial;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::WidgetType::{ComboBox, DragValue};
use rand::random;
use crate::rd::wipes::WipeShape::Circle;

#[derive(Resource)]
pub struct WipeAutomationControls {
    pub on_beat: bool,
    pub randomize_shape: bool,
    pub shape: WipeShape,
    pub randomize_steps: bool,
    pub steps_range: Range<isize>,
    pub randomize_min: bool,
    pub min_range: Range<f32>,
    pub randomize_max: bool,
    pub max_range: Range<f32>,
    pub wipe_time: f32,
    pub beat_div: usize,
    pub beat_count: usize,
}

impl FromWorld for WipeAutomationControls {
    fn from_world(world: &mut World) -> Self {
        Self {
            on_beat: false,
            randomize_shape: true,
            shape: WipeShape::Circle,
            randomize_steps: true,
            steps_range: 5..15,
            randomize_min: true,
            randomize_max: true,
            min_range: 10. .. 40.,
            max_range: 20. .. 50.,
            wipe_time: 0.5,
            beat_div: 1,
            beat_count: 0,
        }
    }
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut mat_query: Query<&Handle<RDShaderMaterial>>,
    mut materials: ResMut<Assets<RDShaderMaterial>>,
    mut event_writer: EventWriter<WipeEvent>,
    mut local_event: Local<WipeEvent>,
    mut automation_controls: ResMut<WipeAutomationControls>,
    keys: Res<Input<KeyCode>>,
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
        if ui.button("Fuzzy").clicked() {
            mat.da = 1.;
            mat.db = 0.07;
            mat.feed = 0.037;
            mat.kill = 0.130;
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
                ui.add(egui::Slider::new(&mut local_event.duration, 0.0..=2.0));
                ui.end_row();

                ui.label("Steps");
                ui.add(egui::Slider::new(&mut local_event.steps, 1..=20));
                ui.end_row();

                ui.label("Start size");
                ui.add(egui::Slider::new(&mut local_event.start_size, 1.0..=100.0));
                ui.end_row();

                ui.label("End size");
                ui.add(egui::Slider::new(&mut local_event.end_size, 1.0..=100.0));
                ui.end_row();
            });

        ui.horizontal(|ui| {
            if ui.button("Circle").clicked() || keys.just_pressed(KeyCode::Key1) {
                event_writer.send(WipeEvent {
                    shape: WipeShape::Circle,
                    ..*local_event
                })
            }
            if ui.button("Octagon").clicked() || keys.just_pressed(KeyCode::Key2) {
                event_writer.send(WipeEvent {
                    shape: WipeShape::Octagon,
                    ..*local_event
                })
            }
            if ui.button("Cross").clicked() || keys.just_pressed(KeyCode::Key3) {
                event_writer.send(WipeEvent {
                    shape: WipeShape::Cross,
                    ..*local_event
                })
            }
            if ui.button("Square").clicked() || keys.just_pressed(KeyCode::Key4) {
                event_writer.send(WipeEvent {
                    shape: WipeShape::Square,
                    ..*local_event
                })
            }
            if ui.button("Hexagram").clicked() || keys.just_pressed(KeyCode::Key5) {
                event_writer.send(WipeEvent {
                    shape: WipeShape::Hexagram,
                    ..*local_event
                })
            }
        });
        if ui.button("Random Shape").clicked() || keys.just_pressed(KeyCode::Key6) {
            event_writer.send(WipeEvent {
                shape: random(),
                ..*local_event
            })
        }

        ui.separator();

        ui.checkbox(&mut automation_controls.on_beat, "On Beat");

        egui::Grid::new("wipe automation")
            .num_columns(3)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("");
                ui.label("Rand");
                ui.label("Min");
                ui.label("Max");
                ui.end_row();

                ui.label("Shape");
                ui.checkbox(&mut automation_controls.randomize_shape, "Rand");
                egui::ComboBox::from_label("Fixed Shape")
                    .selected_text(format!("{:?}", automation_controls.shape))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut automation_controls.shape, WipeShape::Circle, "Circle");
                        ui.selectable_value(&mut automation_controls.shape, WipeShape::Octagon, "Octagram");
                        ui.selectable_value(&mut automation_controls.shape, WipeShape::Cross, "Cross");
                        ui.selectable_value(&mut automation_controls.shape, WipeShape::Square, "Square");
                        ui.selectable_value(&mut automation_controls.shape, WipeShape::Hexagram, "Hexagram");
                    });
                ui.add(egui::DragValue::new(&mut automation_controls.wipe_time).speed(0.01).clamp_range(0. ..=1.));
                ui.end_row();

                ui.label("Steps");
                ui.checkbox(&mut automation_controls.randomize_steps, "Rand");
                ui.add(egui::DragValue::new(&mut automation_controls.steps_range.start).speed(1).clamp_range(0..=30));
                ui.add(egui::DragValue::new(&mut automation_controls.steps_range.end).speed(1).clamp_range(0..=30));
                ui.end_row();

                ui.label("Min");
                ui.checkbox(&mut automation_controls.randomize_min, "Rand");
                ui.add(egui::DragValue::new(&mut automation_controls.min_range.start).speed(1).clamp_range(0. ..=100.));
                ui.add(egui::DragValue::new(&mut automation_controls.min_range.end).speed(1).clamp_range(0. ..=100.));
                ui.end_row();

                ui.label("Max");
                ui.checkbox(&mut automation_controls.randomize_max, "Rand");
                ui.add(egui::DragValue::new(&mut automation_controls.max_range.start).speed(1).clamp_range(0. ..=100.));
                ui.add(egui::DragValue::new(&mut automation_controls.max_range.end).speed(1).clamp_range(0. ..=100.));
                ui.end_row();

                ui.label("Beat Div");
                ui.label(format!("{}", automation_controls.beat_count));
                ui.add(egui::DragValue::new(&mut automation_controls.beat_div).speed(1).clamp_range(0..=32));
                if ui.button("Skip").clicked() {
                    automation_controls.beat_count += 1;
                }

            });
    });
}
