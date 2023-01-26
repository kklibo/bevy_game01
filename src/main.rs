mod arena;
mod cameras;
mod explosion;
mod physics;
mod player;

use bevy::prelude::*;

use arena::*;
use cameras::*;
use explosion::*;
use physics::*;
use player::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(arena::setup)
        .add_startup_system(player::setup)
        .add_startup_system(cameras::setup)
        .add_system(rotate)
        .add_system(player_location_system)
        .add_system(camera_select_system)
        .add_system(player_shoot_system)
        .add_system(projectile_physics_system)
        .add_system(explosion_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
