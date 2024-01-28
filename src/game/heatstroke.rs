use std::time::Duration;

use bevy::prelude::*;
use bevy_fmod::fmod_studio::FmodStudio;

use crate::AppState;

#[derive(Resource)]
pub struct HeastrokeResource{
    passed_out: f32,
    timer: Timer,
    pub pedestal_id: Entity
}

impl Default for HeastrokeResource{
    fn default() -> Self {
        HeastrokeResource {
            passed_out: 0.0,
            timer: Timer::new(Duration::from_secs(8), TimerMode::Once),
            pedestal_id: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Event, Default)]
pub struct Heatstroke {
    // identify who
}

pub fn watch_heatstroke(mut studio: ResMut<FmodStudio>, mut tracker: ResMut<HeastrokeResource>, time: Res<Time>) {
    // Slot1_A_Detune
    // let mut detune_param = studio.0.get_parameter_by_name("Slot1_A_Detune").expect("couldn't find paramter");
    // detune_param.1 = 1.0;

    studio.0.set_parameter_by_name("Slot1_A_Detune", 0.5, true).expect("unable to set fmod parameter");
}

pub fn track_heatstroke(keys: Res<Input<KeyCode>>, mut event_writer: EventWriter<Heatstroke>) {
    if keys.just_pressed(KeyCode::H) {
        event_writer.send(Heatstroke {});
    }
}

pub fn trigger_heatstroke(mut heatstroke: EventReader<Heatstroke>, state: Res<State<AppState>>) {
    for h in heatstroke.read() {
        println!("received heatstroke");
        if state.get() != &AppState::Game {
            println!("but not in game");
        }
    }
}
