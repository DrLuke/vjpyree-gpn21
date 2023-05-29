use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, AsBindGroup, Extent3d, SamplerDescriptor, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::reflect::TypeUuid;
use bevy::render::texture::ImageSampler;
use bevy::render::view::RenderLayers;

use bevy_pyree::render::{FSQuad, spawn_fs_quad, spawn_render_image_to_screen};
use crate::fractal::FractalRenderTarget;


pub struct RDPlugin;

impl Plugin for RDPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_rd)
            .add_plugin(MaterialPlugin::<RDShaderMaterial>::default())
            .add_asset::<RDShaderMaterial>()
            .register_asset_reflect::<RDShaderMaterial>()

            .init_resource::<RDRenderTarget>()
        ;
    }
}

#[derive(Resource)]
pub struct RDRenderTarget {
   pub render_target: Handle<Image>
}

impl FromWorld for RDRenderTarget {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.get_resource_mut::<Assets<Image>>().unwrap();

        let size = Extent3d { width: 1024, height: 1024, ..default() };
        let mut image = Image {
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
        image.resize(size);

        Self {
            render_target: images.add(image)
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Clone, Reflect, FromReflect)]
#[uuid = "7a1722d8-d8a7-4166-96d2-646197a02bfe"]
pub struct RDShaderMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub previous_rt: Handle<Image>,
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
    });

    spawn_fs_quad::<RDShaderMaterial>(
        &mut commands,
        &mut meshes,
        rd_rt.render_target.clone(),
        material_handle,
        4,
        1000,
    );

    spawn_render_image_to_screen(
        &mut commands,
        &mut meshes,
        &mut std_materials,
        rd_rt.render_target.clone(),
        RenderLayers::layer(31),
    );
}