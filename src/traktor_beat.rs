use bevy::prelude::*;
use bevy::utils::tracing::event;
use bevy_pyree::beat::{BeatCounter, BeatEvent};
use bevy_rosc::{MultiAddressOscMethod, OscDispatcher, SingleAddressOscMethod};
use rosc::OscType;

pub struct TraktorPlugin;

impl Plugin for TraktorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_traktor)
            .add_system(traktor_beat_system)
            .insert_resource(TraktorBeat::default())
        ;
    }
}

fn spawn_traktor(mut commands: Commands) {
    commands.spawn((
        MultiAddressOscMethod::new(vec!["/traktor/beat".to_owned(), "/traktor/volume".to_owned()]).unwrap(),
        TraktorReceiver{},
    ));
}

#[derive(Resource)]
pub struct TraktorBeat {
    pub count: isize,
    pub last_volume: isize,
}

#[derive(Component)]
pub struct TraktorReceiver;

impl Default for TraktorBeat {
    fn default() -> Self {
        Self { count: 0, last_volume: 0 }
    }
}

pub fn traktor_beat_system(
    mut query: Query<(&mut MultiAddressOscMethod, &TraktorReceiver), Changed<MultiAddressOscMethod>>,
    mut event_writer: EventWriter<BeatEvent>,
    mut beat_counter: ResMut<BeatCounter>,
    mut traktor_beat: ResMut<TraktorBeat>,
) {
    let maybe = query.get_single_mut();
    if maybe.is_err() { return; }

    let (mut osc, TraktorReceiver) = maybe.unwrap();

    while let Some(new_msg) = osc.get_message() {
        if new_msg.addr == "/traktor/beat".to_owned() {
            traktor_beat.count += 1;
        }
        if new_msg.addr == "/traktor/volume".to_owned() {
            if let Some(OscType::Int(volume)) = new_msg.args.first() {
                traktor_beat.last_volume = *volume as isize;
            }
        }
        if traktor_beat.count >= 24 {
            traktor_beat.count = 0;
            event_writer.send(BeatEvent { count: beat_counter.count, bpm: None });
        }
    }
}
