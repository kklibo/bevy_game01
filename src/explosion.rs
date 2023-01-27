use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Explosion {
    pub creation_time_sec: f32,
    pub lifetime_sec: f32,
}

pub struct ExplosionEvent {
    pub location: Vec3,
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

pub fn explosion_system(
    mut commands: Commands,
    mut explosion_events: EventReader<ExplosionEvent>,
    mut query: Query<(Entity, &mut Transform, &mut Explosion)>,
    time: Res<Time>,
) {
    for x in explosion_events.iter() {
        spawn_explosion(x.location, &mut commands, &time);
    }

    let now = time.elapsed_seconds();
    for (entity, _, explosion) in query.iter_mut() {
        if explosion.creation_time_sec + explosion.lifetime_sec < now {
            commands.entity(entity).despawn();
        }
    }
}
