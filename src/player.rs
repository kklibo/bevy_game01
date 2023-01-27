use bevy::prelude::*;

use crate::physics::Hittable;
use crate::{Blaster, CameraName, Projectile, SelectableCamera};

#[derive(Component)]
pub struct Player;

impl Player {
    pub const MPS: f32 = 5.0;
    pub const DPS: f32 = 180.0;
    pub const RADIUS: f32 = 0.1;
}

pub fn player_location_system(
    mut query: Query<(&mut Transform, &SelectableCamera), Without<Player>>,
    mut query2: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_loc = match query2.iter_mut().next() {
        Some(x) => x,
        None => return,
    };

    if keyboard_input.pressed(KeyCode::Up) {
        let step = player_loc.forward() * Player::MPS * time.delta_seconds();
        player_loc.translation += step;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        let step = player_loc.back() * Player::MPS * time.delta_seconds();
        player_loc.translation += step;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        let step = Player::DPS * time.delta_seconds();
        player_loc.rotate_z(step.to_radians());
    }
    if keyboard_input.pressed(KeyCode::Right) {
        let step = Player::DPS * time.delta_seconds() * -1.0;
        player_loc.rotate_z(step.to_radians());
    }

    //constrain to arena
    fn set_at_least(v: &mut f32, min: f32) {
        *v = f32::max(*v, min)
    }
    fn set_at_most(v: &mut f32, max: f32) {
        *v = f32::min(*v, max)
    }

    set_at_least(&mut player_loc.translation.x, -5.);
    set_at_most(&mut player_loc.translation.x, 5.);
    set_at_least(&mut player_loc.translation.y, -5.);
    set_at_most(&mut player_loc.translation.y, 5.);

    for (mut camera_loc, name) in &mut query {
        if name.0 == CameraName::Chase {
            *camera_loc = *player_loc;
            camera_loc.translation += player_loc.forward() * -5.0;
            camera_loc.translation += Vec3::new(0., 0., 4.);
            camera_loc.look_at(player_loc.translation, Vec3::Z);
        }
    }
}

pub fn player_shoot_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Transform, &mut Blaster), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (entity, player_loc, mut blaster) = match query.iter_mut().next() {
        Some(x) => x,
        None => return,
    };

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
                    owner: entity,
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
    // player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(0.8, 0.2, 0.1).into()),
            transform: Transform::from_xyz(0., -5., 0.).looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        Blaster {
            cooldown_time: 1.,
            time_of_last_shot: 0.,
        },
        Hittable {
            radius: Player::RADIUS,
        },
        Player,
    ));
}
