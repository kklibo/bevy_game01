use bevy::prelude::*;

#[derive(Debug, PartialEq)]
pub enum CameraName {
    Chase,
    Main,
}

#[derive(Component)]
pub struct SelectableCamera(pub CameraName);

pub fn camera_select_system(
    mut query: Query<(&mut Camera, &SelectableCamera)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let selected = if keyboard_input.pressed(KeyCode::Key1) {
        CameraName::Main
    } else if keyboard_input.pressed(KeyCode::Key2) {
        CameraName::Chase
    } else {
        return;
    };

    for (mut camera, name) in &mut query {
        match &name.0 {
            x if *x == selected => camera.is_active = true,
            _ => camera.is_active = false,
        }
    }
}
