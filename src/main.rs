//! Full screen shader effect

mod feedback_shader;
mod mandelbrot;
mod motto;
mod chipspin;

use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, Extent3d, SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::view::RenderLayers;
use bevy_egui::EguiPlugin;
use bevy_rosc::BevyRoscPlugin;
use bevy_pyree::render::{FSQuad, spawn_fs_quad, spawn_render_image_to_screen};
use bevy::reflect::TypeUuid;
use bevy::{
    render::{
        render_resource::{AsBindGroup, ShaderRef},
    },
};
use bevy::render::texture::ImageSampler;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pyree::beat::{BeatEvent, OscBeatReceiverPlugin};
use crate::chipspin::ChipSpin;
use crate::feedback_shader::FeedbackShaderPlugin;
use crate::mandelbrot::MandelbrotPlugin;
use crate::motto::Motto;


fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins
            .set(AssetPlugin {
                watch_for_changes: true,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "VJ Pyree".into(),
                    resolution: WindowResolution::new(1920., 1080.)
                        .with_scale_factor_override(1.75),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::default())

        // Send out a beat event once a second
        // Uncomment this if you want to receive OSC beat events instead
        .add_plugin(BevyRoscPlugin::new("0.0.0.0:31337").unwrap())
        .add_plugin(OscBeatReceiverPlugin::default())

        .add_plugin(ChipSpin)
        //.add_plugin(FeedbackShaderPlugin)
        .add_plugin(MandelbrotPlugin)
        //.add_plugin(Motto)
    ;

    app.run();
}