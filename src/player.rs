use bevy::prelude::*;

use crate::{Blaster, CameraName, Projectile, SelectableCamera};

#[derive(Component)]
pub struct PlayerLocation;

#[derive(Component)]
pub struct PlayerModel;

pub fn player_location_system(
    mut query: Query<
        (&mut Transform, &SelectableCamera),
        (Without<PlayerLocation>, Without<PlayerModel>),
    >,
    mut query2: Query<&mut Transform, (With<PlayerLocation>, Without<PlayerModel>)>,
    mut query3: Query<&mut Transform, (Without<PlayerLocation>, With<PlayerModel>)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_loc = match query2.iter_mut().next() {
        Some(x) => x,
        None => return,
    };

    let mut player_model_loc = match query3.iter_mut().next() {
        Some(x) => x,
        None => return,
    };

    const MPS: f32 = 5.0;
    const DPS: f32 = 180.0;

    if keyboard_input.pressed(KeyCode::Up) {
        let step = player_loc.forward() * MPS * time.delta_seconds();
        player_loc.translation += step;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        let step = player_loc.back() * MPS * time.delta_seconds();
        player_loc.translation += step;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        let step = DPS * time.delta_seconds();
        player_loc.rotate_z(step.to_radians());
    }
    if keyboard_input.pressed(KeyCode::Right) {
        let step = DPS * time.delta_seconds() * -1.0;
        player_loc.rotate_z(step.to_radians());
    }

    for (mut camera_loc, name) in &mut query {
        if name.0 == CameraName::Chase {
            *camera_loc = *player_loc;
            camera_loc.translation += player_loc.forward() * -5.0;
            camera_loc.translation += Vec3::new(0., 0., 4.);
            camera_loc.look_at(player_loc.translation, Vec3::Z);

            *player_model_loc = *player_loc;
        }
    }
}

pub fn player_shoot_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Transform, &mut Blaster), (With<PlayerLocation>, Without<PlayerModel>)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player_loc, mut blaster) = query.iter_mut().next().unwrap();

    if keyboard_input.pressed(KeyCode::Space) {
        let now = time.elapsed_seconds();
        if blaster.time_of_last_shot + blaster.cooldown_time < now {
            blaster.time_of_last_shot = now;

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.05 })),
                    material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
                    transform: *player_loc,
                    ..default()
                },
                Projectile {
                    creation_time_sec: now,
                    lifetime_sec: 1.,
                },
            ));
        }
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Transform::from_xyz(0., -5., 0.).looking_at(Vec3::ZERO, Vec3::Z),
        PlayerLocation,
        Blaster {
            cooldown_time: 1.,
            time_of_last_shot: 0.,
        },
    ));

    // player location cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(0.8, 0.2, 0.1).into()),
            ..default()
        },
        PlayerModel,
    ));
}
