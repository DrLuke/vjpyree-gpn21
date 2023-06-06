use std::ops::Range;
use bevy::prelude::*;
use bevy_pyree::beat::{BeatCounter, BeatEvent};
use rand::random;
use crate::feedback_shader::FeedbackShaderMaterial;
use crate::feedback_shader::ui::FeedbackControlsAutomation;


pub struct AutomationPlugin;

impl Plugin for AutomationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(fb_automation)
        ;
    }
}

fn scale_rand(r: f32, range: &Range<f32>) -> f32
{
    return r * (range.end - range.start) + range.start;
}

fn rand_param(b: &bool, p: &mut f32, range: &Range<f32>)
{
    if *b {
        *p = random::<f32>() * (range.end - range.start) + range.start;
    }
}

fn pt1_param(u: &mut f32, y: f32, pt1: f32, dt: f32)
{
    *u = *u + (y - *u) * (dt/(pt1+dt))
}

fn fb_automation(
    mut beat_event_listener: EventReader<BeatEvent>,
    controls: Res<FeedbackControlsAutomation>,
    mut materials: ResMut<Assets<FeedbackShaderMaterial>>,
    mut mat_query: Query<&Handle<FeedbackShaderMaterial>>,
    time: Res<Time>,
    beat_counter: Res<BeatCounter>,
) {
    let mat_handle = mat_query.get_single_mut().unwrap();
    let mut mat = materials.get_mut(mat_handle).unwrap();

    pt1_param(&mut mat.beat_stuff.beatpt1, 0., controls.beatpt1, time.delta_seconds());
    pt1_param(&mut mat.beat_stuff.beataccumpt1, 0., mat.beat_stuff.beataccum, time.delta_seconds());

    for beat_event in &mut beat_event_listener {
        if controls.col_r {
            mat.col_rot.x = scale_rand(random(), &controls.col_r_range);
        }
        if controls.col_g {
            mat.col_rot.y = scale_rand(random(), &controls.col_g_range);
        }
        if controls.col_b {
            mat.col_rot.z = scale_rand(random(), &controls.col_b_range);
        }
        if controls.col_w {
            mat.col_rot.w = scale_rand(random(), &controls.col_w_range);
        }

        rand_param(&controls.rand[0], &mut mat.rand.p0, &controls.rand_range[0]);
        rand_param(&controls.rand[1], &mut mat.rand.p1, &controls.rand_range[1]);
        rand_param(&controls.rand[2], &mut mat.rand.p2, &controls.rand_range[2]);
        rand_param(&controls.rand[3], &mut mat.rand.p3, &controls.rand_range[3]);
        rand_param(&controls.rand[4], &mut mat.rand.p4, &controls.rand_range[4]);
        rand_param(&controls.rand[5], &mut mat.rand.p5, &controls.rand_range[5]);
        rand_param(&controls.rand[6], &mut mat.rand.p6, &controls.rand_range[6]);
        rand_param(&controls.rand[7], &mut mat.rand.p7, &controls.rand_range[7]);

        mat.beat_stuff.beat = 1.;
        mat.beat_stuff.beatpt1 = 1.;
        mat.beat_stuff.beataccum = beat_counter.get_count() as f32;
    }

    pt1_param(&mut mat.randpt1.p0, mat.rand.p0, controls.pt1[0], time.delta_seconds());
    pt1_param(&mut mat.randpt1.p1, mat.rand.p1, controls.pt1[1], time.delta_seconds());
    pt1_param(&mut mat.randpt1.p2, mat.rand.p2, controls.pt1[2], time.delta_seconds());
    pt1_param(&mut mat.randpt1.p3, mat.rand.p3, controls.pt1[3], time.delta_seconds());
    pt1_param(&mut mat.randpt1.p4, mat.rand.p4, controls.pt1[4], time.delta_seconds());
    pt1_param(&mut mat.randpt1.p5, mat.rand.p5, controls.pt1[5], time.delta_seconds());
    pt1_param(&mut mat.randpt1.p6, mat.rand.p6, controls.pt1[6], time.delta_seconds());
    pt1_param(&mut mat.randpt1.p7, mat.rand.p7, controls.pt1[7], time.delta_seconds());
}