mod ui;

use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, AsBindGroup, Extent3d, SamplerDescriptor, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::reflect::TypeUuid;
use bevy::render::texture::ImageSampler;
use bevy::render::view::RenderLayers;

use bevy_pyree::render::{FSQuad, spawn_fs_quad, spawn_render_image_to_screen};
use crate::feedback_shader::ui::ui_system;
use crate::fractal::FractalRenderTarget;
use crate::rd::RDRenderTarget;


pub struct FeedbackShaderPlugin;

impl Plugin for FeedbackShaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_feedback_shader)
            .add_plugin(MaterialPlugin::<FeedbackShaderMaterial>::default())
            .add_asset::<FeedbackShaderMaterial>()
            .register_asset_reflect::<FeedbackShaderMaterial>()
            .init_resource::<FeedbackShaderRenderTarget>()

            .add_system(ui_system)
        ;
    }
}

#[derive(Resource)]
pub struct FeedbackShaderRenderTarget{
    pub render_target: Handle<Image>
}

impl FromWorld for FeedbackShaderRenderTarget {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.get_resource_mut::<Assets<Image>>().unwrap();

        let size = Extent3d { width: 1920, height: 610, ..default() };
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
            render_target: images.add(image),
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Clone, Reflect, FromReflect)]
#[uuid = "4f8c9212-8d94-44ca-91f0-be4e177fe418"]
pub struct FeedbackShaderMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub previous_rt: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub fractal_rt: Handle<Image>,
    #[texture(4)]
    #[sampler(5)]
    pub rd_rt: Handle<Image>,
    #[uniform(6)]
    pub col_rot: Vec4,
}

impl Material for FeedbackShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/feedback.wgsl".into()
    }
}

pub fn spawn_feedback_shader(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FeedbackShaderMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    feedback_rt: Res<FeedbackShaderRenderTarget>,
    fractal_rt: Res<FractalRenderTarget>,
    rd_rt: Res<RDRenderTarget>
) {
    let material_handle = materials.add(FeedbackShaderMaterial {
        previous_rt: feedback_rt.render_target.clone(),
        fractal_rt: fractal_rt.render_target.clone(),
        rd_rt: rd_rt.render_target.clone(),
        col_rot: Vec4::new(0.5, 0.5, 0.5, 1.),
    });

    spawn_fs_quad::<FeedbackShaderMaterial>(
        &mut commands,
        &mut meshes,
        feedback_rt.render_target.clone(),
        material_handle,
        1,
        1000,
    );

    /*spawn_render_image_to_screen(
        &mut commands,
        &mut meshes,
        &mut std_materials,
        rt_res.render_target.clone(),
        RenderLayers::layer(31),
    );*/
}