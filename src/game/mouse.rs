use bevy::prelude::*;
use bevy_mod_picking::prelude::*;


pub fn mouse_setup(
    mut commands: Commands,
){
    // Spawn squares
    commands.spawn((
        super::InGame,
        PickableBundle::default(), // <- Makes the mesh pickable.
        On::<Pointer<Click>>::target_commands_mut(|_click, target_commands| {    
            target_commands.despawn();
        }),
    ));
}