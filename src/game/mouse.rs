use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use std::f32::consts::FRAC_PI_2;


pub fn mouse_setup(
    mut commands: Commands,
){
    // Spawn squares
    commands.spawn((
        PickableBundle::default(), // <- Makes the mesh pickable.
        On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
        On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.translation.x += drag.delta.x; // Make the square follow the mouse
            transform.translation.y -= drag.delta.y;
        }),
        On::<Pointer<Drop>>::commands_mut(|event, commands| {
            commands.entity(event.dropped).insert(Spin(FRAC_PI_2)); // Spin dropped entity
            commands.entity(event.target).insert(Spin(-FRAC_PI_2)); // Spin dropped-on entity
        }),
    ));
}

#[derive(Component)]
struct Spin(f32);

fn spin(mut square: Query<(&mut Spin, &mut Transform)>) {
    for (mut spin, mut transform) in square.iter_mut() {
        transform.rotation = Quat::from_rotation_z(spin.0);
        let delta = -spin.0.clamp(-1.0, 1.0) * 0.05;
        spin.0 += delta;
    }
}