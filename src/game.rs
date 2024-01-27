use bevy::prelude::*;

use self::heatstroke::track_heatstroke;

mod heatstroke;

pub struct MyGamePlugin;
impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, set_scene)
            .add_systems(Update, track_heatstroke);
    }

    fn is_unique(&self) -> bool {
        true
    }
}

fn set_scene() {}