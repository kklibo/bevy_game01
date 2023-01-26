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

pub fn setup(
    mut commands: Commands,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 2.5).looking_at(Vec3::ZERO, Vec3::Z),
            camera: Camera {
                is_active: true,
                ..default()
            },
            ..default()
        },
        SelectableCamera(CameraName::Chase),
    ));
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., 10.).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                is_active: false,
                ..default()
            },
            ..default()
        },
        SelectableCamera(CameraName::Main),
    ));
}