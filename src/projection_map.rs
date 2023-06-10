use crate::fractal::FractalRenderTarget;
use crate::rd::RDRenderTarget;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dBindGroup, Mesh2dHandle};

pub struct ProjectionMapPlugin;

impl Plugin for ProjectionMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(ui_system)
            .add_system(chip_card_system);
    }
}

#[derive(Component)]
pub struct ChipSpinCard;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    fractal_rt: Res<FractalRenderTarget>,
    rd_rt: Res<RDRenderTarget>,
    feedback_rt: Res<FeedbackShaderRenderTarget>,
    chipsin: Res<ChipSpinTexture>,
) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 1920.,
                height: 1080.,
            },
            ..default()
        },
        ..default()
    });

    // Top rectangle
    commands.spawn(SpriteBundle {
        texture: feedback_rt.render_target.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(1920., 610.)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 235., 0.)),
        ..default()
    });

    // Parallelogram
    // 935, 340
    // <- 77
    let mut parallelogram = Mesh::new(PrimitiveTopology::TriangleList);
    let v_pos = vec![
        [0., 0., 0.],      // bottom right
        [-77., 340., 0.],  // top right
        [-935., 340., 0.], // top left
        [-858., 0., 0.],   // bottom left
    ];
    parallelogram.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    let v_uv = vec![[1., 1.], [1., 0.], [0., 0.], [0., 1.]];
    parallelogram.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_uv);

    parallelogram.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 4]);

    let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];
    parallelogram.set_indices(Some(Indices::U32(indices)));

    let mesh_handle = Mesh2dHandle(meshes.add(parallelogram));

    let mat_handle = color_materials.add(ColorMaterial {
        texture: Some(feedback_rt.render_target.clone()),
        ..default()
    });
    commands.spawn(ColorMesh2dBundle {
        mesh: mesh_handle.clone(),
        transform: Transform::from_xyz(0., -480., 0.),
        material: mat_handle.clone(),
        ..default()
    });
    commands.spawn(ColorMesh2dBundle {
        mesh: mesh_handle.clone(),
        transform: Transform::from_xyz(0., -480., 0.).with_scale(Vec3::new(-1., 1., 1.)),
        material: mat_handle.clone(),
        ..default()
    });

    /*let mat_handle = color_materials.add(ColorMaterial {
        texture: Some(fractal_rt.render_target.clone()),
        ..default()
    });
    commands.spawn(ColorMesh2dBundle {
        mesh: mesh_handle.clone(),
        transform: Transform::from_xyz(0., -480., 1.),
        material: mat_handle.clone(),
        ..default()
    });
    commands.spawn(ColorMesh2dBundle {
        mesh: mesh_handle.clone(),
        transform: Transform::from_xyz(0., -480., 1.).with_scale(Vec3::new(-1., 1., 1.)),
        material: mat_handle.clone(),
        ..default()
    });*/

    // CHIPS
    let mesh = meshes.add(shape::Quad::new(Vec2::splat(100.)).into());
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::splat(600.)).into())
            .into(),
        material: color_materials.add(ColorMaterial {
            texture: Some(chipsin.texture.clone()),
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(400., 250., 10.)),
        ..default()
    }, ChipSpinCard{}));

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::splat(600.)).into())
            .into(),
        material: color_materials.add(ColorMaterial {
            texture: Some(chipsin.texture.clone()),
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(-400., 250., 10.)),
        ..default()
    }, ChipSpinCard{}));
}

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowRef};

use crate::WindowResolution;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Shape;
use bevy_pyree::beat::BeatEvent;
use rand::random;
use crate::beat_controls::BeatMute;
use crate::chipspin::{ChipSpinStateResource, ChipSpinTexture};
use crate::feedback_shader::{FeedbackShaderMaterial, FeedbackShaderRenderTarget};
use crate::traktor_beat::TraktorBeat;


pub fn ui_system(mut contexts: EguiContexts, mut commands: Commands) {
    egui::Window::new("Projection Map").show(contexts.ctx_mut(), |ui| {
        if ui.button("Spawn Window").clicked() {
            let second_window = commands
                .spawn(Window {
                    title: "VJ Pyree output".to_owned(),
                    resolution: WindowResolution::new(1920.0, 1080.0)
                        .with_scale_factor_override(1.0),
                    present_mode: PresentMode::AutoVsync,
                    ..Default::default()
                })
                .id();

            commands.spawn(Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                    ..default()
                },
                camera: Camera {
                    target: RenderTarget::Window(WindowRef::Entity(second_window)),
                    ..default()
                },
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::Fixed {
                        width: 1920.,
                        height: 1080.,
                    },
                    ..default()
                },
                ..default()
            });
        }
    });
}

fn pt1_param(u: &mut f32, y: f32, pt1: f32, dt: f32)
{
    *u = *u + (y - *u) * (dt/(pt1+dt))
}

pub fn chip_card_system(
    chip_spin_state: Res<ChipSpinStateResource>,
    mut query: Query<&mut Visibility, With<ChipSpinCard>>,
    mut transform_query: Query<&mut Transform, With<ChipSpinCard>>,
    traktor: Res<TraktorBeat>,
    mut event_listener: EventReader<BeatEvent>,
    mut beat_mute: Res<BeatMute>,
    time: Res<Time>,
) {
    for mut vis in query.iter_mut() {
        *vis = match chip_spin_state.visible {
            true => Visibility::Visible,
            false => Visibility::Hidden,
        }
    }

    for mut transform in transform_query.iter_mut() {
        let mirror = match (transform.translation.x > 0.) {
            true => 1.,
            false => -1.,
        };
        pt1_param(&mut transform.scale.x, mirror, chip_spin_state.pt1_t, time.delta_seconds());
        pt1_param(&mut transform.scale.y, 1., chip_spin_state.pt1_t, time.delta_seconds());
    }

    for beat_event in &mut event_listener {
        if beat_mute.mute { continue; }
        if chip_spin_state.jump == false { continue; }

        for mut transform in transform_query.iter_mut() {
            let strength = (traktor.last_volume as f32 / 128.) * chip_spin_state.pt1_strength;
            let mirror = match (transform.translation.x > 0.) {
                true => 1.,
                false => -1.,
            };
            *transform = transform.with_scale(Vec3::new((1. + strength) * mirror, 1. + strength, 1.));
        }
    }

}