use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, AsBindGroup, Extent3d, SamplerDescriptor, ShaderRef, ShaderType, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::reflect::TypeUuid;
use bevy::render::texture::{DEFAULT_IMAGE_HANDLE, ImageSampler};
use bevy::render::view::RenderLayers;

use bevy_pyree::render::{FSQuad, spawn_fs_quad, spawn_render_image_to_screen};
use crate::chipspin::ChipSpinTexture;


pub struct FractalPlugin;

impl Plugin for FractalPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_feedback_shader)
            .add_plugin(MaterialPlugin::<FractalMaterial>::default())
            .add_asset::<FractalMaterial>()
            .register_asset_reflect::<FractalMaterial>()
            .init_resource::<FractalRenderTarget>()
        ;
    }
}

#[derive(Resource)]
pub struct FractalRenderTarget {
    pub render_target: Handle<Image>
}

impl FromWorld for FractalRenderTarget {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.get_resource_mut::<Assets<Image>>().unwrap();

        let size = Extent3d { width: 1920, height: 1080, ..default() };
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

#[derive(Clone, Reflect, FromReflect, ShaderType)]
pub struct JuliaC {
    pub re: f32,
    pub im: f32,
}

#[derive(AsBindGroup, TypeUuid, Clone, Reflect, FromReflect)]
#[uuid = "8f890807-2d1a-4312-86fc-07660a06e39c"]
pub struct FractalMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub previous_rt: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub image_trap: Handle<Image>,
    #[uniform(4)]
    pub julia_c: JuliaC,
}

impl Material for FractalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fractal.wgsl".into()
    }
}

pub fn spawn_feedback_shader(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FractalMaterial>>,
    chip_spin_texture: Res<ChipSpinTexture>,
    fractal_rt: Res<FractalRenderTarget>,
) {

    let material_handle = materials.add(FractalMaterial {
        previous_rt: fractal_rt.render_target.clone(),
        //image_trap: assets.load("images/trip2.png"),
        image_trap: chip_spin_texture.texture.clone(),
        julia_c: JuliaC {re: -0.8696, im: 0.26},
    });

    spawn_fs_quad::<FractalMaterial>(
        &mut commands,
        &mut meshes,
        fractal_rt.render_target.clone(),
        material_handle,
        2,
        0,
    );

    /*spawn_render_image_to_screen(
        &mut commands,
        &mut meshes,
        &mut std_materials,
        rt_res.render_target.clone(),
        RenderLayers::layer(31),
    );*/
}