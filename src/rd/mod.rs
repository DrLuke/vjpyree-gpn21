mod ui;
mod wipes;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::render_resource::{
    AddressMode, AsBindGroup, Extent3d, FilterMode, SamplerDescriptor, ShaderRef,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::texture::ImageSampler;
use bevy::render::view::RenderLayers;

use crate::fractal::FractalRenderTarget;
use crate::rd::ui::ui_system;
use crate::rd::wipes::{wipe_event_listener_system, wipe_system, WipeEvent};
use bevy_pyree::render::{spawn_fs_quad, spawn_render_image_to_screen, FSQuad};
use bevy_smud::SmudPlugin;

pub struct RDPlugin;

impl Plugin for RDPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SmudPlugin)
            .add_startup_system(spawn_rd)
            .add_plugin(MaterialPlugin::<RDShaderMaterial>::default())
            .add_asset::<RDShaderMaterial>()
            .register_asset_reflect::<RDShaderMaterial>()
            .add_system(ui_system)
            .init_resource::<RDRenderTarget>()
            .add_event::<WipeEvent>()
            .add_system(wipe_event_listener_system)
            .add_system(wipe_system)
        ;
    }
}

#[derive(Resource)]
pub struct RDRenderTarget {
    pub render_target: Handle<Image>,
}

impl FromWorld for RDRenderTarget {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.get_resource_mut::<Assets<Image>>().unwrap();

        let size = Extent3d {
            width: 1024,
            height: 1024,
            ..default()
        };
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba16Float,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            sampler_descriptor: ImageSampler::Descriptor(SamplerDescriptor {
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                ..Default::default()
            }),
            ..default()
        };
        image.resize(size);

        Self {
            render_target: images.add(image),
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Clone, Reflect, FromReflect)]
#[uuid = "7a1722d8-d8a7-4166-96d2-646197a02bfe"]
pub struct RDShaderMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub previous_rt: Handle<Image>,
    #[uniform(2)]
    pub da: f32,
    #[uniform(3)]
    pub db: f32,
    #[uniform(4)]
    pub feed: f32,
    #[uniform(5)]
    pub kill: f32,
}

impl Material for RDShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/rd.wgsl".into()
    }
}

pub fn spawn_rd(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RDShaderMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    rd_rt: Res<RDRenderTarget>,
) {
    let material_handle = materials.add(RDShaderMaterial {
        previous_rt: rd_rt.render_target.clone(),
        da: 1.0,
        db: 0.3,
        feed: 0.0287,
        kill: 0.078,
    });

    spawn_fs_quad::<RDShaderMaterial>(
        &mut commands,
        &mut meshes,
        rd_rt.render_target.clone(),
        material_handle,
        4,
        1000,
    );

    commands
        .spawn(Camera2dBundle {
            camera: Camera {
                order: 1001,
                target: RenderTarget::Image(rd_rt.render_target.clone()),
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::Fixed {
                    width: 100.,
                    height: 100.,
                },
                ..default()
            },
            ..default()
        })
        .insert(RenderLayers::layer(4));

    /*spawn_render_image_to_screen(
        &mut commands,
        &mut meshes,
        &mut std_materials,
        rd_rt.render_target.clone(),
        RenderLayers::layer(31),
    );*/
}
