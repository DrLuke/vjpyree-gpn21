use bevy::prelude::*;
use std::ops::Range;

use crate::feedback_shader::FeedbackShaderMaterial;
use bevy_egui::{egui, EguiContexts};

#[derive(Resource)]
pub struct FeedbackControlsAutomation {
    pub col_r: bool,
    pub col_g: bool,
    pub col_b: bool,
    pub col_w: bool,
    pub col_r_range: Range<f32>,
    pub col_g_range: Range<f32>,
    pub col_b_range: Range<f32>,
    pub col_w_range: Range<f32>,
    pub rand: [bool; 8],
    pub rand_range: [Range<f32>; 8],
    pub pt1: [f32; 8],
    pub beatpt1: f32,
    pub beataccumpt1: f32,
    pub rand_pal: bool,
}

impl FromWorld for FeedbackControlsAutomation {
    fn from_world(world: &mut World) -> Self {
        Self {
            col_r: false,
            col_g: false,
            col_b: false,
            col_w: false,
            col_r_range: 0. ..1.,
            col_g_range: 0. ..1.,
            col_b_range: 0. ..1.,
            col_w_range: -0.05 ..0.05,
            rand: [false; 8],
            rand_range: [0. ..1., 0. ..1., 0. ..1., 0. ..1., 0. ..1., 0. ..1., 0. ..1., 0. ..1.],
            pt1: [0.3; 8],
            beatpt1: 0.3,
            beataccumpt1: 0.,
            rand_pal: false,
        }
    }
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut mat_query: Query<&Handle<FeedbackShaderMaterial>>,
    mut materials: ResMut<Assets<FeedbackShaderMaterial>>,
    mut fb_controls_automation: ResMut<FeedbackControlsAutomation>,
) {
    let mat_handle = mat_query.get_single_mut().unwrap();
    let mut mat = materials.get_mut(mat_handle).unwrap();

    egui::Window::new("Feedback Shader").show(contexts.ctx_mut(), |ui| {

        ui.label("Color Rotation");

        egui::Grid::new("col_rot")
            .num_columns(5)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("");
                ui.label("Val");
                ui.label("Rand");
                ui.label("Range");
                ui.end_row();

                ui.label("X");
                ui.add(egui::DragValue::new(&mut mat.col_rot.x).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.col_r, "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.col_r_range.start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.col_r_range.end).speed(0.01).max_decimals(2));
                ui.end_row();

                ui.label("Y");
                ui.add(egui::DragValue::new(&mut mat.col_rot.y).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.col_g, "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.col_g_range.start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.col_g_range.end).speed(0.01).max_decimals(2));
                ui.end_row();

                ui.label("Z");
                ui.add(egui::DragValue::new(&mut mat.col_rot.z).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.col_b, "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.col_b_range.start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.col_b_range.end).speed(0.01).max_decimals(2));
                ui.end_row();

                ui.label("W");
                ui.add(egui::DragValue::new(&mut mat.col_rot.w).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.col_w, "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.col_w_range.start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.col_w_range.end).speed(0.01).max_decimals(2));
                ui.end_row();
            });
        ui.separator();
        egui::Grid::new("rand")
            .num_columns(5)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("");
                ui.label("Val");
                ui.label("Beat");
                ui.label("Min");
                ui.label("Max");
                ui.label("T1");
                ui.end_row();

                // 0
                ui.label("PT1 bounce");
                ui.add(egui::DragValue::new(&mut mat.rand.p0).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.rand[0], "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[0].start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[0].end).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.pt1[0]).speed(0.01).max_decimals(2).clamp_range(0. ..=f32::INFINITY));
                ui.end_row();

                // 1
                ui.label("Accum rot");
                ui.add(egui::DragValue::new(&mut mat.rand.p1).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.rand[1], "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[1].start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[1].end).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.pt1[1]).speed(0.01).max_decimals(2).clamp_range(0. ..=f32::INFINITY));
                ui.end_row();

                // 2
                ui.label("Chip PT1 scale");
                ui.add(egui::DragValue::new(&mut mat.rand.p2).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.rand[2], "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[2].start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[2].end).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.pt1[2]).speed(0.01).max_decimals(2).clamp_range(0. ..=f32::INFINITY));
                ui.end_row();

                ui.label("3");
                ui.add(egui::DragValue::new(&mut mat.rand.p3).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.rand[3], "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[3].start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[3].end).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.pt1[3]).speed(0.01).max_decimals(2).clamp_range(0. ..=f32::INFINITY));
                ui.end_row();

                // 4
                ui.label("Rot");
                ui.add(egui::DragValue::new(&mut mat.rand.p4).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.rand[4], "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[4].start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[4].end).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.pt1[4]).speed(0.01).max_decimals(2).clamp_range(0. ..=f32::INFINITY));
                ui.end_row();

                // 5
                ui.label("HSV Rot Mult");
                ui.add(egui::DragValue::new(&mut mat.rand.p5).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.rand[5], "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[5].start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[5].end).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.pt1[5]).speed(0.01).max_decimals(2).clamp_range(0. ..=f32::INFINITY));
                ui.end_row();

                // 6
                ui.label("FB Rot");
                ui.add(egui::DragValue::new(&mut mat.rand.p6).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.rand[6], "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[6].start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[6].end).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.pt1[6]).speed(0.01).max_decimals(2).clamp_range(0. ..=f32::INFINITY));
                ui.end_row();
                
                ui.label("7");
                ui.add(egui::DragValue::new(&mut mat.rand.p7).speed(0.01).max_decimals(2));
                ui.add(egui::Checkbox::new(&mut fb_controls_automation.rand[7], "Rand"));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[7].start).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.rand_range[7].end).speed(0.01).max_decimals(2));
                ui.add(egui::DragValue::new(&mut fb_controls_automation.pt1[7]).speed(0.01).max_decimals(2).clamp_range(0. ..=f32::INFINITY));
                ui.end_row();

            });

        ui.separator();

        egui::Grid::new("beatpt1")
            .num_columns(5)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("");
                ui.label("PT1");
                ui.label("Val");
                ui.end_row();

                ui.label("Beat");
                ui.add(egui::DragValue::new(&mut fb_controls_automation.beatpt1).speed(0.01).max_decimals(2).clamp_range(0. ..=f32::INFINITY));
                ui.label(format!("{:.2}", mat.beat_stuff.beat));
                ui.label(format!("{:.2}", mat.beat_stuff.beatpt1));
                ui.label(format!("{:.2}", mat.beat_stuff.beataccum));
                ui.label(format!("{:.2}", mat.beat_stuff.beataccumpt1));
                ui.end_row();
        });

        ui.separator();
        ui.horizontal(|ui|{
            if ui.button("Rainbow").clicked() { mat.settings.palette = 0. }
            if ui.button("Reddish").clicked() { mat.settings.palette = 1. }
            if ui.button("Red/Green").clicked() { mat.settings.palette = 2. }
            if ui.button("Hot/Cold").clicked() { mat.settings.palette = 3. }
            if ui.button("Straw/Blue").clicked() { mat.settings.palette = 4. }
            if ui.button("Freestyle").clicked() { mat.settings.palette = 5. }
            ui.checkbox(&mut fb_controls_automation.rand_pal, "Rand");
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("None").clicked() { mat.settings.mirror_x = 0. }
            if ui.button("Mirror X").clicked() { mat.settings.mirror_x = 1. }
        });

        ui.separator();

        ui.label("UV scale");
        ui.horizontal(|ui| {
            if ui.button("YEET^(-1)").clicked() { mat.settings.uv_scale = 0.95 }
            if ui.button("Slow In").clicked() { mat.settings.uv_scale = 0.99 }
            if ui.button("No").clicked() { mat.settings.uv_scale = 1. }
            if ui.button("Slow Out").clicked() { mat.settings.uv_scale = 1.01 }
            if ui.button("YEET").clicked() { mat.settings.uv_scale = 1.05 }
        });
    });
}
