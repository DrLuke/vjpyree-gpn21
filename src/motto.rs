use std::f32::consts::PI;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AddressMode, Extent3d, AsBindGroup, SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::texture::ImageSampler;
use bevy::sprite::Anchor;
use crate::{RenderLayers, ShaderRef};
use crate::AlphaMode::Blend;
use crate::chipspin::ChipSpinTexture;
use crate::shape::Quad;


pub struct Motto;

impl Plugin for Motto{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(gltf_render_layer_system)
            .add_system(spin_dip_system)

            .add_plugin(MaterialPlugin::<ChipCardMaterial>::default())
            .add_asset::<ChipCardMaterial>()
            .register_asset_reflect::<ChipCardMaterial>()
        ;
    }
}

#[derive(Component)]
struct DipModel;

#[derive(AsBindGroup, TypeUuid, Clone, Reflect, FromReflect)]
#[uuid = "a74c9f74-53c4-4939-80a8-055656553555"]
pub struct ChipCardMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub chip: Handle<Image>,
    #[uniform(2)]
    pub glitch_offset: f32,
    #[uniform(3)]
    pub glitch_pixelation: f32,
    #[uniform(4)]
    pub glitch_abberration: f32,
    #[texture(5)]
    #[sampler(6)]
    pub noise: Handle<Image>,
}

impl Material for ChipCardMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/chip_card.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode { AlphaMode::Blend }
}

/// Rebuild this scene: https://entropia.de/images/8/83/Gpn21-plakat-dina2.png
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chip_card_materials: ResMut<Assets<ChipCardMaterial>>,
    assets: Res<AssetServer>,
    chip_spin_texture: Res<ChipSpinTexture>,
) {
    // 2D realm ---------

    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize(1.),
            near: 2000.,
            far: -2000.,
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(0.0, 0.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            order: 1000,
            ..default()
        },
        tonemapping: Tonemapping::None,
        ..default()
    })
        .insert(RenderLayers::layer(9))
    ;

    // Background/Plakat
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::splat(1.)).into()),
        material: materials.add(StandardMaterial{
            base_color_texture: Some(assets.load("images/plakat_scaled_clean.png")),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_scale(Vec3{x: 1527., y: 1080., z: 1.}),
        ..default()
    })
        .insert(RenderLayers::layer(9))
    ;

    // Chip cards
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::splat(1.)).into()),
        material: chip_card_materials.add(ChipCardMaterial{
            chip: chip_spin_texture.texture.clone(),
            glitch_offset: 0.,
            glitch_pixelation: 0.,
            glitch_abberration: 0.,
            noise: assets.load("images/noise512.png"),
        }),
        transform: Transform::from_scale(Vec3{x: 800., y: 800., z: 1.})
            .with_translation(Vec3{x: -450., y:-40., z:-1.})
        ,
        ..default()
    }).insert(RenderLayers::layer(9));
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::splat(1.)).into()),
        material: chip_card_materials.add(ChipCardMaterial{
            chip: chip_spin_texture.texture.clone(),
            glitch_offset: 0.5,
            glitch_pixelation: 0.,
            glitch_abberration: 0.,
            noise: assets.load("images/noise512.png"),
        }),
        transform: Transform::from_scale(Vec3{x: 800., y: 800., z: 1.})
            .with_translation(Vec3{x: 0., y:-40., z:-1.})
        ,
        ..default()
    }).insert(RenderLayers::layer(9));
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::splat(1.)).into()),
        material: chip_card_materials.add(ChipCardMaterial{
            chip: chip_spin_texture.texture.clone(),
            glitch_offset: 1.,
            glitch_pixelation: 0.,
            glitch_abberration: 0.,
            noise: assets.load("images/noise512.png"),
        }),
        transform: Transform::from_scale(Vec3{x: 800., y: 800., z: 1.})
            .with_translation(Vec3{x: 450., y:-40., z:-1.})
        ,
        ..default()
    }).insert(RenderLayers::layer(9));
}

fn gltf_render_layer_system(
    mut commands: Commands,
    mut query: Query<(&RenderLayers, &Children), (With<Handle<Scene>>, Changed<Children>, With<RenderLayers>)>,
    child_query: Query<&Children>
) {
    for (&render_layers, children) in query.iter_mut() {
        if render_layers.intersects(&RenderLayers::layer(10)) {
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
    commands.entity(child).insert(RenderLayers::layer(10));
    for child in child_query.get(child).map_err(drop)?.iter() {
        let _ = recursive_render_layer_insert(commands, *child, child_query);
    }
    Ok(())
}

fn spin_dip_system(
    mut query: Query<&mut Transform, With<DipModel>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_z(time.delta_seconds())
    }
}