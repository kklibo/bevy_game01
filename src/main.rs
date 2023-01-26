//! A simple 3D scene with light shining over a cube sitting on a plane.

mod arena;
mod player;
mod cameras;
mod explosion;
mod physics;

use std::f32::consts::PI;

use bevy::prelude::*;

use arena::*;
use player::*;
use cameras::*;
use explosion::*;
use physics::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(arena::setup)
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


fn rotate_camera(mut query: Query<&mut Transform, With<Camera3d>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_z(time.delta_seconds() / 20.),
        );
    }
}
