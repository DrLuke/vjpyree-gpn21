use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, AsBindGroup, Extent3d, SamplerDescriptor, ShaderRef, ShaderType, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::reflect::TypeUuid;
use bevy::render::texture::ImageSampler;
use bevy::render::view::RenderLayers;

use bevy_pyree::render::{FSQuad, spawn_fs_quad, spawn_render_image_to_screen};
use crate::chipspin::ChipSpinTexture;


pub struct MandelbrotPlugin;

impl Plugin for MandelbrotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_feedback_shader)
            .add_plugin(MaterialPlugin::<MandelBrotMaterial>::default())
            .add_asset::<MandelBrotMaterial>()
            .register_asset_reflect::<MandelBrotMaterial>()
        ;
    }
}

#[derive(Resource)]
struct MandelBrotRenderTarget {
    render_target: Handle<Image>
}

impl MandelBrotRenderTarget {
    fn new(
        images: &mut ResMut<Assets<Image>>
    ) -> Self {
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
pub struct MandelBrotMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub previous_rt: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub image_trap: Handle<Image>,
    #[uniform(4)]
    pub julia_c: JuliaC,
}

impl Material for MandelBrotMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mandelbrot.wgsl".into()
    }
}

pub fn spawn_feedback_shader(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MandelBrotMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
    chip_spin_texture: Res<ChipSpinTexture>,
) {
    let rt_res = MandelBrotRenderTarget::new(&mut images);

    let material_handle = materials.add(MandelBrotMaterial {
        previous_rt: rt_res.render_target.clone(),
        //image_trap: assets.load("images/trip2.png"),
        image_trap: chip_spin_texture.texture.clone(),
        julia_c: JuliaC {re: 0.2, im: 0.55},
    });

    spawn_fs_quad::<MandelBrotMaterial>(
        &mut commands,
        &mut meshes,
        rt_res.render_target.clone(),
        material_handle,
        2,
        0,
    );

    spawn_render_image_to_screen(
        &mut commands,
        &mut meshes,
        &mut std_materials,
        rt_res.render_target.clone(),
        RenderLayers::layer(31),
    );

    commands.insert_resource(rt_res);
}