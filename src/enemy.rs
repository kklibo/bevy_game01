use bevy::prelude::*;

use crate::Blaster;
use crate::Player;
use crate::Projectile;

#[derive(Component)]
pub struct Enemy {
    pub radius: f32,
    pub next_waypoint: Option<Vec3>,
}

#[derive(Component)]
pub struct EnemyProjectile;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // enemy cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(0.8, 0.8, 0.1).into()),
            transform: Transform::from_xyz(-4., 2., 0.).looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        Enemy {
            radius: 0.1,
            next_waypoint: Some(Vec3::ZERO),
        },
        Blaster {
            time_of_last_shot: 0.,
            cooldown_time: 1.,
        },
    ));

    // enemy cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(0.8, 0.8, 0.1).into()),
            transform: Transform::from_xyz(-4., 2., 0.).looking_at(Vec3::new(-5., 5., 0.), Vec3::Z),
            ..default()
        },
        Enemy {
            radius: 0.1,
            next_waypoint: Some(Vec3::ZERO),
        },
        Blaster {
            time_of_last_shot: 0.,
            cooldown_time: 1.,
        },
    ));
}

pub fn enemy_movement_system(
    mut query: Query<(&mut Transform, &mut Enemy), Without<Player>>,
    mut query2: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    const MPS: f32 = 1.0;
    const DPS: f32 = 180.0;

    const WAYPOINT_RADIUS_M: f32 = 1.0;
    const VISION_RADIUS_M: f32 = 2.0;

    for (mut loc, mut enemy) in query.iter_mut() {
        let mut selected_target = None;

        if selected_target.is_none() {
            //try to target player
            if let Some(player_loc) = query2.iter_mut().next() {
                let to_player = player_loc.translation - loc.translation;
                if to_player.length() < VISION_RADIUS_M {
                    selected_target = Some(player_loc.translation);
                }
            }
        }

        if selected_target.is_none() {
            //waypoint target
            if let Some(x) = enemy.next_waypoint {
                let to_waypoint = x - loc.translation;
                if to_waypoint.length() < WAYPOINT_RADIUS_M {
                    //new waypoint
                    // temp
                    if enemy.next_waypoint == Some(Vec3::new(-4., -4., 0.)) {
                        enemy.next_waypoint = None;
                    } else {
                        enemy.next_waypoint = Some(Vec3::new(-4., -4., 0.));
                    }
                    continue;
                }
                selected_target = Some(x);
            }
        }

        if let Some(x) = selected_target {
            let to_target = x - loc.translation;

            if loc.forward().cross(to_target).dot(Vec3::Z) > 0. {
                //turn left
                let step = DPS * time.delta_seconds();
                loc.rotate_z(step.to_radians());
            } else {
                //turn right
                let step = DPS * time.delta_seconds() * -1.;
                loc.rotate_z(step.to_radians());
            }

            //move forward
            let step = loc.forward() * MPS * time.delta_seconds();
            loc.translation += step;
        }
    }
}

pub fn enemy_shooting_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Transform, &mut Enemy, &mut Blaster), Without<Player>>,
    mut query2: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    const SHOOT_RADIUS_M: f32 = 1.0;
    const SHOOT_ANGLE_DEG: f32 = 45.0;

    for (loc, _, mut blaster) in query.iter_mut() {
        if let Some(player_loc) = query2.iter_mut().next() {
            let to_player = player_loc.translation - loc.translation;
            if to_player.length() < SHOOT_RADIUS_M
                && to_player.angle_between(loc.forward()).to_degrees() < SHOOT_ANGLE_DEG
            {
                let now = time.elapsed_seconds();
                if blaster.time_of_last_shot + blaster.cooldown_time < now {
                    blaster.time_of_last_shot = now;

                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.05 })),
                            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
                            transform: *loc,
                            ..default()
                        },
                        Projectile {
                            creation_time_sec: now,
                            lifetime_sec: 1.,
                        },
                        EnemyProjectile,
                    ));
                }
            }
        }
    }
}
