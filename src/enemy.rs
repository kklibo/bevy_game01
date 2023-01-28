use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::Blaster;
use crate::Hittable;
use crate::Player;
use crate::Projectile;

#[derive(Component)]
pub struct Enemy {
    pub next_waypoint: Option<Vec3>,
}

impl Enemy {
    pub const MPS: f32 = 1.0;
    pub const DPS: f32 = 180.0;
    pub const WAYPOINT_RADIUS_M: f32 = 1.0;
    pub const VISION_RADIUS_M: f32 = 2.0;
    pub const SHOOT_RADIUS_M: f32 = 1.0;
    pub const SHOOT_ANGLE_DEG: f32 = 45.0;
}

#[derive(Resource)]
pub struct SpawnTimer(Timer);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(SpawnTimer(Timer::new(
        Duration::from_secs(2),
        TimerMode::Repeating,
    )));
}

pub fn spawn_enemy_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        spawn_enemy(
            &mut commands,
            &mut meshes,
            &mut materials,
            Transform::from_translation(random_coord()).looking_at(random_coord(), Vec3::Z),
        );
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: Transform,
) {
    // enemy cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(0.8, 0.8, 0.1).into()),
            transform,
            ..default()
        },
        Enemy {
            next_waypoint: Some(Vec3::ZERO),
        },
        Blaster {
            time_of_last_shot: 0.,
            cooldown_time: 1.,
        },
        Hittable { radius: 0.1 },
    ));
}

fn random_coord() -> Vec3 {
    let mut rng = rand::thread_rng();

    Vec3::new(
        rng.gen_range(-4..=4) as f32,
        rng.gen_range(-4..=4) as f32,
        0.,
    )
}

pub fn enemy_movement_system(
    mut query: Query<(&mut Transform, &mut Enemy), Without<Player>>,
    mut query2: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    for (mut loc, mut enemy) in query.iter_mut() {
        let mut selected_target = None;

        if selected_target.is_none() {
            //try to target player
            if let Some(player_loc) = query2.iter_mut().next() {
                let to_player = player_loc.translation - loc.translation;
                if to_player.length() < Enemy::VISION_RADIUS_M {
                    selected_target = Some(player_loc.translation);
                }
            }
        }

        if selected_target.is_none() {
            //waypoint target
            if let Some(x) = enemy.next_waypoint {
                let to_waypoint = x - loc.translation;
                if to_waypoint.length() < Enemy::WAYPOINT_RADIUS_M {
                    //new waypoint
                    enemy.next_waypoint = Some(random_coord());
                    continue;
                }
                selected_target = Some(x);
            }
        }

        if let Some(x) = selected_target {
            let to_target = x - loc.translation;

            if loc.forward().cross(to_target).dot(Vec3::Z) > 0. {
                //turn left
                let step = Enemy::DPS * time.delta_seconds();
                loc.rotate_z(step.to_radians());
            } else {
                //turn right
                let step = Enemy::DPS * time.delta_seconds() * -1.;
                loc.rotate_z(step.to_radians());
            }

            //move forward
            let step = loc.forward() * Enemy::MPS * time.delta_seconds();
            loc.translation += step;
        }
    }
}

pub fn enemy_shooting_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Transform, &mut Enemy, &mut Blaster), Without<Player>>,
    mut query2: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    for (entity, loc, _, mut blaster) in query.iter_mut() {
        if let Some(player_loc) = query2.iter_mut().next() {
            let to_player = player_loc.translation - loc.translation;
            if to_player.length() < Enemy::SHOOT_RADIUS_M
                && to_player.angle_between(loc.forward()).to_degrees() < Enemy::SHOOT_ANGLE_DEG
            {
                let now = time.elapsed_seconds();
                if blaster.time_of_last_shot + blaster.cooldown_time < now {
                    blaster.time_of_last_shot = now;

                    let mut s: StandardMaterial = Color::rgb(0.5, 1.0, 0.5).into();
                    s.emissive = Color::rgba(0.5, 1.0, 0.5, 1.0);

                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.05 })),
                            material: materials.add(s),
                            transform: *loc,
                            ..default()
                        },
                        Projectile {
                            owner: entity,
                            creation_time_sec: now,
                            lifetime_sec: 1.,
                            mps: Projectile::DEFAULT_MPS,
                        },
                    ));
                }
            }
        }
    }
}
