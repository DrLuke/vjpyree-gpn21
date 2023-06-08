use bevy::prelude::*;

use crate::rd::wipes::WipeShape::{Circle, Cross, Hexagram, Octagon, Square};
use crate::RenderLayers;
use bevy_smud::prelude::*;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum WipeShape {
    Circle,
    Octagon,
    Cross,
    Square,
    Hexagram,
}

impl Distribution<WipeShape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WipeShape {
        match rng.gen_range(0..=5) {
            0 => Circle,
            1 => Octagon,
            2 => Cross,
            3 => Square,
            _ => Hexagram,
        }
    }
}

#[derive(Clone)]
pub struct WipeEvent {
    pub(crate) shape: WipeShape,
    pub(crate) duration: f32,
    pub(crate) steps: isize,
    pub(crate) start_size: f32,
    pub(crate) end_size: f32,
}

impl WipeEvent {
    /// Calculates the start time of a step
    pub fn step_time(&self, step: isize) -> f32 {
        return self.duration / (self.steps as f32) * (step as f32);
    }
    pub fn step_size(&self, step: isize) -> f32 {
        let step_size = (self.end_size - self.start_size) / ((self.steps - 1) as f32);
        return self.start_size + step_size * (step as f32);
    }
}

impl Default for WipeEvent {
    fn default() -> Self {
        WipeEvent {
            shape: Circle,
            duration: 1.,
            steps: 5,
            start_size: 1.,
            end_size: 10.,
        }
    }
}

#[derive(Component, Clone)]
pub struct WipeElement {
    trigger: WipeEvent,
    step: isize,
    age: f32,
}

#[derive(Resource)]
pub struct WipeShaders {
    pub circle: Handle<Shader>,
    pub octagon: Handle<Shader>,
    pub cross: Handle<Shader>,
    pub square: Handle<Shader>,
    pub hexagram: Handle<Shader>,
}

impl FromWorld for WipeShaders {
    fn from_world(world: &mut World) -> Self {
        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
        Self {
            circle: shaders.add_sdf_expr("sd_circle(p, params.x)"),
            octagon:  shaders.add_sdf_expr("sd_octagon(p, params.x)"),
            cross:  shaders.add_sdf_expr("sd_cross(p, vec2<f32>(params.x, params.x * 0.3), 0.1)"),
            square: shaders.add_sdf_expr("sd_box(p, vec2<f32>(params.x))"),
            hexagram: shaders.add_sdf_expr("sd_hexagram(p, params.x)"),
        }
    }
}

fn spawn_wipe(
    trigger: WipeEvent,
    commands: &mut Commands,
    shaders: &mut ResMut<Assets<Shader>>,
    wipe_shader: Handle<Shader>,
) {
    let fill = shaders.add_fill_body(
        r"
            let d_2 = abs(d - 1.) - 1.;
            let a = sd_fill_alpha_fwidth(d_2);
            return vec4<f32>(color.rgb, a * color.a);
            ",
    );

    for step in 0..trigger.steps {
        let step_size = trigger.step_size(step);
        commands
            .spawn((
                WipeElement {
                    trigger: trigger.clone(),
                    step,
                    age: 0.0,
                },
                SpatialBundle {
                    visibility: Visibility::Hidden,
                    ..default()
                },
            ))
            .with_children(|child_builder| {
                child_builder.spawn((
                    ShapeBundle {
                        shape: SmudShape {
                            color: Color::GREEN,
                            sdf: wipe_shader.clone(),
                            frame: Frame::Quad(55.),
                            fill: fill.clone(),
                            params: Vec4::new(step_size, 0., 0., 0.),
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 1.),
                        ..default()
                    },
                    RenderLayers::layer(4),
                ));
            });
    }
}

pub fn wipe_event_listener_system(
    mut event_reader: EventReader<WipeEvent>,
    mut commands: Commands,
    mut shaders: ResMut<Assets<Shader>>,
    wipe_shaders: Local<WipeShaders>,
) {
    for event in event_reader.iter() {
        let sdf = match event.shape {
            WipeShape::Circle => wipe_shaders.circle.clone(),
            WipeShape::Octagon => wipe_shaders.octagon.clone(),
            WipeShape::Cross => wipe_shaders.cross.clone(),
            WipeShape::Square => wipe_shaders.square.clone(),
            WipeShape::Hexagram => wipe_shaders.hexagram.clone(),
        };
        spawn_wipe(event.clone(), &mut commands, &mut shaders, sdf)
    }
}

pub fn wipe_system(
    mut query: Query<(Entity, &mut WipeElement, &mut Visibility)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut wipe, mut visibility) in query.iter_mut() {
        wipe.age += time.delta_seconds();

        let start_time = wipe.trigger.step_time(wipe.step);
        let end_time = wipe.trigger.step_time(wipe.step + 1);
        // Start visibility
        if wipe.age > start_time && *visibility == Visibility::Hidden {
            *visibility = Visibility::Inherited;
        }
        // Delete when done
        if wipe.age > end_time && *visibility == Visibility::Inherited {
            commands.entity(entity).despawn_recursive();
        }
    }
}
