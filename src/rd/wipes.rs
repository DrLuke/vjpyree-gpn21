use bevy::prelude::*;

use crate::rd::wipes::WipeShape::Circle;
use crate::RenderLayers;
use bevy_smud::prelude::*;

#[derive(Clone)]
pub enum WipeShape {
    Circle,
    Octagon,
    Cross,
    Square,
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

fn spawn_wipe(trigger: WipeEvent, commands: &mut Commands, shaders: &mut ResMut<Assets<Shader>>) {
    let fill = shaders.add_fill_body(
        r"
            let d_2 = abs(d - 1.) - 1.;
            let a = sd_fill_alpha_fwidth(d_2);
            return vec4<f32>(color.rgb, a * color.a);
            ",
    );

    for step in 0..trigger.steps {
        let step_size = trigger.step_size(step);
        let sdf = match trigger.shape {
            WipeShape::Circle => shaders.add_sdf_expr(format!("sd_circle(p, {step_size:.10})")),
            WipeShape::Octagon => shaders.add_sdf_expr(format!("sd_octagon(p, {step_size:.10})")),
            WipeShape::Cross => {
                let cross_size = step_size * 0.3;
                shaders.add_sdf_expr(format!(
                    "sd_cross(p, vec2<f32>({step_size:.10}, {cross_size:.10}), 0.1)"
                ))
            }
            WipeShape::Square => {
                shaders.add_sdf_expr(format!("sd_circle(p, {:.10})", trigger.step_size(step)))
            }
        };
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
                            sdf,
                            frame: Frame::Quad(55.),
                            fill: fill.clone(),
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
) {
    for event in event_reader.iter() {
        spawn_wipe(event.clone(), &mut commands, &mut shaders)
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
