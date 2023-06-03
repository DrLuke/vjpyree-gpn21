use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::fractal::FractalRenderTarget;
use crate::rd::RDRenderTarget;

pub struct ProjectionMapPlugin;

impl Plugin for ProjectionMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
        ;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    fractal_rt: Res<FractalRenderTarget>,
    rd_rt: Res<RDRenderTarget>,
)
{
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: 1920., height: 1080. },
            ..default()
        },
        ..default()
    });

    // Top rectangle
    /*let shape = shapes::Rectangle {
        extents: Vec2::new(1920., 610.),
        ..default()
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform::from_xyz(0., 235., 0.),
            material: color_materials.add(ColorMaterial{
                texture: Some(fractal_rt.render_target.clone()),
                ..default()
            });
            ..default()
        },
    ));*/
    commands.spawn(SpriteBundle {
        texture: rd_rt.render_target.clone(),
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
        [0., 0., 0.], // bottom right
        [-77., 340., 0.], // top right
        [-935., 340., 0.], // top left
        [-858., 0., 0.], // bottom left
    ];
    parallelogram.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    let v_uv = vec![
        [1., 1.],
        [1., 0.],
        [0., 0.],
        [0., 1.],
    ];
    parallelogram.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_uv);

    parallelogram.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 4]);


    let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];
    parallelogram.set_indices(Some(Indices::U32(indices)));

    let mesh_handle = Mesh2dHandle(meshes.add(parallelogram));
    let mat_handle = color_materials.add(ColorMaterial{
            texture: Some(fractal_rt.render_target.clone()),
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
}
