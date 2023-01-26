//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::f32::consts::PI;

use bevy::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate)
        //.add_system(rotate_camera)
        .add_system(player_location_system)
        .add_system(camera_select_system)
        .add_system(player_shoot_system)
        .add_system(projectile_physics_system)
        .add_system(explosion_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}

/// set up a simple 3D scene
fn setup(
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

    // target drone cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(0.8, 0.8, 0.1).into()),
            transform: Transform::from_xyz(-4., 2., 0.),
            ..default()
        },
        TargetDrone { radius: 0.1 },
    ));
}

#[derive(Debug, PartialEq)]
enum CameraName {
    Chase,
    Main,
}

#[derive(Component)]
struct SelectableCamera(CameraName);

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

#[derive(Component)]
struct PlayerLocation;

#[derive(Component)]
struct PlayerModel;

#[derive(Component)]
struct TargetDrone {
    radius: f32,
}

#[derive(Component)]
struct Blaster {
    cooldown_time: f32,
    time_of_last_shot: f32,
}

#[derive(Component, Debug)]
struct Projectile {
    creation_time_sec: f32,
    lifetime_sec: f32,
}

#[derive(Component, Debug)]
struct Explosion {
    creation_time_sec: f32,
    lifetime_sec: f32,
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_seconds() / 2.);
    }
}

fn rotate_camera(mut query: Query<&mut Transform, With<Camera3d>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_z(time.delta_seconds() / 20.),
        );
    }
}

fn player_location_system(
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

fn camera_select_system(
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

fn player_shoot_system(
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

fn projectile_physics_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Projectile), Without<TargetDrone>>,
    mut query2: Query<(Entity, &mut Transform, &mut TargetDrone), Without<Projectile>>,
    time: Res<Time>,
) {
    const MPS: f32 = 0.1;

    let now = time.elapsed_seconds();
    for (entity, mut loc, mut projectile) in query.iter_mut() {
        if projectile.creation_time_sec + projectile.lifetime_sec < now {
            commands.entity(entity).despawn();
            continue;
        }

        let step = loc.forward() * MPS;
        loc.translation += step;

        for (target_entity, target_loc, target_drone) in query2.iter_mut() {
            if loc.translation.distance(target_loc.translation) < target_drone.radius {
                commands.entity(entity).despawn();
                commands.entity(target_entity).despawn();
                spawn_explosion(target_loc.translation, &mut commands, &time);
            }
        }
    }
}

fn spawn_explosion(loc: Vec3, commands: &mut Commands, time: &Res<Time>) {
    let now = time.elapsed_seconds();

    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 15000.0,
                shadows_enabled: true,
                color: Color::RED,
                ..default()
            },
            transform: Transform::from_translation(loc),
            ..default()
        },
        Explosion {
            lifetime_sec: 1.,
            creation_time_sec: now,
        },
    ));
}

fn explosion_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Explosion)>,
    time: Res<Time>,
) {
    let now = time.elapsed_seconds();
    for (entity, _, mut explosion) in query.iter_mut() {
        if explosion.creation_time_sec + explosion.lifetime_sec < now {
            commands.entity(entity).despawn();
        }
    }
}
