use bevy::prelude::*;


// #[derive(Resource)]
// struct MusicianSlot {

// }

// const MUSICIAN_RGBS: [Color::Rgba; 3] = [
//     Color::Rgba { red:  0.0, green:  0.0, blue:  1.0, alpha:  1.0},
//     Color::Rgba { red:  0.0, green:  1.0, blue:  0.0, alpha:  1.0},
//     Color::Rgba { red:  1.0, green:  0.0, blue:  0.0, alpha:  1.0}
// ];


// fn spawn_musician(){
    
// }

#[derive(Event)]
struct SwitchMusicianEvent {
    pub musician_slot_id: Entity,
    pub new_scene_handle: Handle<Scene>
}

fn switch_musician(mut commands: Commands, mut events: EventReader<SwitchMusicianEvent>){
    for event in events.read(){
        match commands.get_entity(event.musician_slot_id) {
            Some(musician_slot) => {
                musician_slot
                ;
            },
            None => {debug!("faulty SwitchMusicianEvent, couldn't resolve musician_slot_id");},
        };
    }
}