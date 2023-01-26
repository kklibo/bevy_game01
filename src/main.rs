//! A simple 3D scene with light shining over a cube sitting on a plane.

mod arena;
mod player;
mod cameras;
mod explosion;

use std::f32::consts::PI;

use bevy::prelude::*;

use arena::*;
use player::*;
use cameras::*;
use explosion::*;

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

#[derive(Component)]
struct TargetDrone {
    radius: f32,
}

#[derive(Component)]
pub struct Blaster {
    cooldown_time: f32,
    time_of_last_shot: f32,
}

#[derive(Component, Debug)]
struct Projectile {
    creation_time_sec: f32,
    lifetime_sec: f32,
}

fn rotate_camera(mut query: Query<&mut Transform, With<Camera3d>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_z(time.delta_seconds() / 20.),
        );
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
