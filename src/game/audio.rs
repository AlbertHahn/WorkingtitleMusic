use bevy::prelude::*;
use bevy_fmod::fmod_studio::FmodStudio;
use crossbeam_channel::{unbounded, Receiver, Sender};
use libfmod::ffi::{FMOD_RESULT, FMOD_STUDIO_EVENTINSTANCE, FMOD_STUDIO_EVENT_CALLBACK_TYPE};

pub enum LevelEvent {
    TriggerHeatstroke,
    LevelEndEvent,
}

#[derive(Event)]
pub struct FmodEvent(LevelEvent);

// Struct to hold the closure and context data
struct CallbackWrapper<F> {
    callback: F,
}

// Implement a function with the expected FFI signature that calls the closure
extern "C" fn callback_bridge<F>(
    param1: u32,
    param2: *mut FMOD_STUDIO_EVENTINSTANCE,
    param3: *mut std::ffi::c_void,
) -> i32
where
    F: FnMut(u32, *mut FMOD_STUDIO_EVENTINSTANCE, *mut std::ffi::c_void) -> i32,
{
    unsafe {
        // Get the closure from the context data
        let callback_wrapper: &mut CallbackWrapper<F> = &mut *(param3 as *mut CallbackWrapper<F>);

        // Call the closure with the provided parameters
        (callback_wrapper.callback)(param1, param2, param3)
    }
}

impl<F> CallbackWrapper<F>
where
    F: FnMut(u32, *mut FMOD_STUDIO_EVENTINSTANCE, *mut std::ffi::c_void) -> i32,
{
    // Function to create a new CallbackWrapper
    fn new(callback: F) -> CallbackWrapper<F> {
        CallbackWrapper { callback }
    }

    // Function to get the function pointer to the bridge function
    fn get_callback_bridge(&self) -> libfmod::ffi::FMOD_STUDIO_EVENT_CALLBACK {
        Some(callback_bridge::<F>)
    }
}

#[derive(Resource)]
struct FmodReceiver(Receiver<LevelEvent>);

pub fn setup_fmod_callbacks(
    mut commands: Commands,
    studio: Res<FmodStudio>,
    event_sender: EventWriter<FmodEvent>,
) {
    let (sender, receiver): (Sender<LevelEvent>, Receiver<LevelEvent>) = unbounded();

    commands.insert_resource(FmodReceiver(receiver));

    // Spawn a new thread to handle the callback
    // std::thread::spawn(move || {
    let mut my_closure = move |param1: u32,
                               param2: *mut FMOD_STUDIO_EVENTINSTANCE,
                               param3: *mut std::ffi::c_void| {
        print!("test!!!");
        // Send the event to the main thread
        sender.send(LevelEvent::TriggerHeatstroke).unwrap();
        0 // Placeholder return value
    };

    let callback_wrapper = CallbackWrapper::new(my_closure);
    let callback_bridge = callback_wrapper.get_callback_bridge();

    // let _ = studio
    //     .0
    //     .get_event("event:/triggers/trigger_heatstroke")
    //     .unwrap()
    //     .set_callback(callback_bridge, 0x0000_0004u32);
    
    // std::mem::forget(callback_wrapper);

    // The callback wrapper and bridge will live as long as the thread is running

    // The receiver can be used to receive events in this thread if needed
    // let received_event = receiver.recv().unwrap();
    // println!("Received event: {:?}", received_event);
    // });

    // In the Bevy application thread, receive events from the callback thread
    // and forward them to the event system
    // AppBuilder::set_runner(|_, _| {
    //     std::thread::spawn(move || loop {
    //         if let Ok(event) = receiver.recv() {
    //             event_sender.send(FmodEvent(event)).unwrap();
    //         }
    //     });
    // });
}
