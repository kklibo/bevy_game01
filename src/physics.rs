use bevy::prelude::*;

use crate::spawn_explosion;

#[derive(Component)]
pub struct TargetDrone {
    pub radius: f32,
}

#[derive(Component)]
pub struct Blaster {
    pub cooldown_time: f32,
    pub time_of_last_shot: f32,
}

#[derive(Component, Debug)]
pub struct Projectile {
    pub creation_time_sec: f32,
    pub lifetime_sec: f32,
}

pub fn projectile_physics_system(
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