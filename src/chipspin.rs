use std::f32::consts::{PI, TAU};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AddressMode, Extent3d, AsBindGroup, SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::texture::{DEFAULT_IMAGE_HANDLE, ImageSampler};
use bevy::sprite::Anchor;
use bevy::utils::tracing::event;
use bevy_egui::{egui, EguiContexts};
use bevy_pyree::beat::BeatEvent;
use rand::random;
use crate::{RenderLayers, ShaderRef};
use crate::AlphaMode::Blend;
use crate::beat_controls::BeatMute;
use crate::chipspin::ChipSpinState::Fixed;
use crate::shape::Quad;


pub struct ChipSpin;

impl Plugin for ChipSpin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(gltf_render_layer_system)

            .add_system(chip_spin_ui.before(spin_dip_system))
            .add_system(spin_dip_system)

            .init_resource::<ChipSpinTexture>()
            .init_resource::<ChipSpinStateResource>()


        ;
    }
}

#[derive(Component)]
struct DipModel;

#[derive(Resource)]
pub struct ChipSpinTexture {
    pub texture: Handle<Image>
}

impl FromWorld for ChipSpinTexture {
    fn from_world(world: &mut World) -> Self {
        Self {
            texture: DEFAULT_IMAGE_HANDLE.typed::<Image>()
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    assets: Res<AssetServer>,
    mut chip_spin_texture: ResMut<ChipSpinTexture>,
) {
    // Text on the chip must first be rendered to a texture before it can be rendered in 3d space
    let size = Extent3d { width: 1024, height: 1024, ..default() };
    let mut dip_text_image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba32Float,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[]
        },
        sampler_descriptor: ImageSampler::Descriptor(SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            ..Default::default()
        }),
        ..default()
    };
    dip_text_image.resize(size);

    let dip_text_image_handle = images.add(dip_text_image);
    commands.spawn(Camera2dBundle{
        camera: Camera {
            target: RenderTarget::Image(dip_text_image_handle.clone()),
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgba(0., 0., 0., 0.)),
            ..default()
        },
        ..default()
    }).insert(RenderLayers::layer(8));

    let font = assets.load("fonts/Roboto-Bold.ttf");
    let text_style1 = TextStyle {
        font: font.clone(),
        font_size: 190.0,
        color: Color::WHITE,
    };

    commands.spawn(Text2dBundle {
        text_anchor: Anchor::TopLeft,
        text: Text::from_section("GPN 42/2", text_style1).with_alignment(TextAlignment::Left),
        transform: Transform::from_scale(Vec3::splat(1.))
            .with_translation(Vec3::new(-480.,500., 0.)),
        ..default()
    }).insert(RenderLayers::layer(8));

    let text_style2 = TextStyle {
        font: font.clone(),
        font_size: 100.0,
        color: Color::WHITE,
    };
    commands.spawn(Text2dBundle {
        text_anchor: Anchor::BottomRight,
        text: Text::from_section("Project Poltergeist", text_style2).with_alignment(TextAlignment::Right),
        transform: Transform::from_scale(Vec3::splat(1.))
            .with_translation(Vec3::new(315.,-50., 0.)),
        ..default()
    }).insert(RenderLayers::layer(8));


    // 3D scene for rendering chip to texture
    let size = Extent3d { width: 1024, height: 1024, ..default() };
    let mut dip_render_target_image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba32Float,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[]
        },
        sampler_descriptor: ImageSampler::Descriptor(SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            ..Default::default()
        }),
        ..default()
    };
    dip_render_target_image.resize(size);

    let dip_render_target_handle = images.add(dip_render_target_image);

    chip_spin_texture.texture = dip_render_target_handle.clone();

    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize(1.),
            near: 2000.,
            far: -2000.,
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(0.0, 500.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            order: -1000,
            target: RenderTarget::Image(dip_render_target_handle.clone()),
            ..default()
        },
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::rgba(0., 0., 0., 0.)),
            ..default()
        },
        tonemapping: Tonemapping::None,
        ..default()
    })
        .insert(RenderLayers::layer(16))
    ;

    let my_gltf = assets.load("models/gpn21-dip.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(0., 0., 0.)
            .with_scale(Vec3::splat(100.))
            .with_rotation(Quat::from_rotation_x(-PI/2.)),
        ..Default::default()
    })
        .insert(RenderLayers::layer(16))
        .insert(DipModel)
        .with_children(|child_builder| {
            child_builder
                .spawn(MaterialMeshBundle{
                    mesh: meshes.add(Quad::new(Vec2::splat(1.)).into()),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(dip_text_image_handle.clone()),
                        alpha_mode: AlphaMode::Blend,
                        ..default()
                    }),
                    transform: Transform::from_scale(Vec3::splat(4.))
                        .with_translation(Vec3::new(-0.85, 1.67, -0.5))
                        .with_rotation(Quat::from_rotation_x(PI/2.)*Quat::from_rotation_z(-PI/2.)),
                    ..default()
                }).insert(RenderLayers::layer(10));
        })
    ;

    // Lights
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            illuminance: 7000.,
            color: Color::rgb(0.6, 0.6, 1.0),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(f32::to_radians(-10.)) *
                Quat::from_rotation_z(f32::to_radians(20.)),
            ..default()
        },
        ..default()
    }).insert(RenderLayers::layer(16));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            illuminance: 4000.,
            color: Color::ORANGE_RED,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(f32::to_radians(-80.)) *
                Quat::from_rotation_z(f32::to_radians(-40.)),
            ..default()
        },
        ..default()
    }).insert(RenderLayers::layer(16));
}

fn gltf_render_layer_system(
    mut commands: Commands,
    mut query: Query<(&RenderLayers, &Children), (With<Handle<Scene>>, Changed<Children>, With<RenderLayers>)>,
    child_query: Query<&Children>
) {
    for (&render_layers, children) in query.iter_mut() {
        if render_layers.intersects(&RenderLayers::layer(16)) {
            for child in children.iter() {
                let _ = recursive_render_layer_insert(&mut commands, *child, &child_query);
            }
        }
    }
}

fn recursive_render_layer_insert(
    commands: &mut Commands,
    child: Entity,
    child_query: &Query<&Children>,
) -> Result<(), ()> {
    commands.entity(child).insert(RenderLayers::layer(16));
    for child in child_query.get(child).map_err(drop)?.iter() {
        let _ = recursive_render_layer_insert(commands, *child, child_query);
    }
    Ok(())
}

// Chip spin system
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChipSpinState {
    Default, // Default position
    Fixed, // Set fixed rotation
    Rate, // Rotate with rate
}

#[derive(Resource)]
pub struct ChipSpinStateResource {
    pub state: ChipSpinState,
    pub fixed: (f32, f32, f32),
    pub rate: (f32, f32, f32),
    pub visible: bool,
    pub jump: bool,
    pub pt1_strength: f32,
    pub pt1_t: f32,
    pub rand: bool,
    pub rand_range: f32,
}

impl FromWorld for ChipSpinState {
    fn from_world(world: &mut World) -> Self { Self::Rate }
}

impl FromWorld for ChipSpinStateResource {
    fn from_world(world: &mut World) -> Self {
        Self {
            state: ChipSpinState::Rate,
            fixed: (0., 0., 0.),
            rate: (0.25, 0., 0.),
            visible: true,
            jump: false,
            pt1_strength: 1.,
            pt1_t: 0.2,
            rand: false,
            rand_range: 2.,
        }
    }
}

fn spin_dip_system(
    mut query: Query<&mut Transform, With<DipModel>>,
    time: Res<Time>,
    mut csr: ResMut<ChipSpinStateResource>,
    mut event_listener: EventReader<BeatEvent>,
    beat_mute: Res<BeatMute>,
) {
    let mut transform = query.single_mut();

    for event in &mut event_listener {
        if beat_mute.mute { continue; }
        if csr.rand {
            match csr.state {
                Fixed => {
                    csr.fixed.0 = random::<f32>() * csr.rand_range * TAU;
                    csr.fixed.1 = random::<f32>() * csr.rand_range * TAU;
                    csr.fixed.2 = random::<f32>() * csr.rand_range * TAU;
                },
                ChipSpinState::Rate => {
                    csr.rate.0 = random::<f32>() * csr.rand_range;
                    csr.rate.1 = random::<f32>() * csr.rand_range;
                    csr.rate.2 = random::<f32>() * csr.rand_range;
                },
                _ => {}
            }
        }
    }

    match csr.state {
        ChipSpinState::Default => {
            *transform = transform.with_rotation(Quat::from_euler(EulerRot::XYZ, -2., -1.55, 0.));
        },
        ChipSpinState::Fixed => {
            *transform = transform.with_rotation(Quat::from_euler(EulerRot::XYZ, csr.fixed.0, csr.fixed.1, csr.fixed.2));
        },
        ChipSpinState::Rate => {
            transform.rotate(
                Quat::from_rotation_x(csr.rate.0 * time.delta_seconds()) *
                    Quat::from_rotation_y(csr.rate.1 * time.delta_seconds()) *
                    Quat::from_rotation_z(csr.rate.2 * time.delta_seconds())
            );
        },
    }
}

fn chip_spin_ui(
    mut contexts: EguiContexts,
    mut csr: ResMut<ChipSpinStateResource>,
    mut query: Query<&mut Transform, With<DipModel>>,
) {
    let mut transform = query.single_mut();

    egui::Window::new("Chip Spin").show(contexts.ctx_mut(), |ui| {
        egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Rot Mode");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut csr.state, ChipSpinState::Default, "Default");
                        ui.selectable_value(&mut csr.state, ChipSpinState::Fixed, "Fixed");
                        ui.selectable_value(&mut csr.state, ChipSpinState::Rate, "Rate");
                    });
                    ui.end_row();

                    ui.label("Fixed");
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut csr.fixed.0).speed(0.01).max_decimals(2));
                        ui.add(egui::DragValue::new(&mut csr.fixed.1).speed(0.01).max_decimals(2));
                        ui.add(egui::DragValue::new(&mut csr.fixed.2).speed(0.01).max_decimals(2));
                    });
                    ui.end_row();

                    ui.label("Rate");
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut csr.rate.0).speed(0.01).max_decimals(2));
                        ui.add(egui::DragValue::new(&mut csr.rate.1).speed(0.01).max_decimals(2));
                        ui.add(egui::DragValue::new(&mut csr.rate.2).speed(0.01).max_decimals(2));
                    });
                    ui.end_row();

                    ui.label("Fixed from current");
                    if ui.button("Set").clicked() {
                        csr.fixed = transform.rotation.to_euler(EulerRot::XYZ);
                        csr.state = Fixed;
                    }
                });
        ui.separator();
        ui.checkbox(&mut csr.visible, "Show");
        ui.checkbox(&mut csr.jump, "Audio React");
        ui.add(egui::DragValue::new(&mut csr.pt1_strength).speed(0.01));
        ui.add(egui::DragValue::new(&mut csr.pt1_t).speed(0.01).clamp_range(0. ..=1000.));
        ui.checkbox(&mut csr.rand, "Rand");
        ui.add(egui::DragValue::new(&mut csr.rand_range).speed(0.01));
    });
}