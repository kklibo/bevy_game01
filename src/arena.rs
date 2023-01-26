use bevy::prelude::*;

#[derive(Component)]
pub struct Shape;

pub fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_seconds() / 2.);
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in 0..10 {
        for y in 0..10 {
            let color = if (x + y) % 2 == 0 {
                Color::rgb(0.3, 0.3, 0.3)
            } else {
                Color::rgb(0.1, 0.1, 0.1)
            };

            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                material: materials.add(color.into()),
                transform: Transform::from_xyz(x as f32 * 1. - 4.5, y as f32 * 1. - 4.5, 0.)
                    .with_rotation(Quat::from_rotation_x(90_f32.to_radians())),
                ..default()
            });
        }
    }

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0).with_rotation(Quat::from_rotation_arc(
                Vec3::new(0., 0., 1.).normalize(),
                Vec3::new(1., 1., 1.).normalize(),
            )),
            ..default()
        },
        Shape,
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 2.0, 8.0),
        ..default()
    });
}