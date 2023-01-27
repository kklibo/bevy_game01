use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Explosion {
    pub creation_time_sec: f32,
    pub lifetime_sec: f32,
}

pub struct ExplosionEvent {
    pub location: Vec3,
}

pub fn explosion_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut explosion_events: EventReader<ExplosionEvent>,
    mut query: Query<(Entity, &mut Transform, &mut Explosion)>,
    time: Res<Time>,
) {
    let now = time.elapsed_seconds();

    for x in explosion_events.iter() {
        let mut entity = commands.spawn((
            PointLightBundle {
                point_light: PointLight {
                    intensity: 3000.0,
                    shadows_enabled: true,
                    color: Color::RED,
                    ..default()
                },
                transform: Transform::from_translation(x.location),
                ..default()
            },
            Explosion {
                lifetime_sec: 1.,
                creation_time_sec: now,
            },
        ));
        //adds explosion cube, overwriting some previous components (fix this?)
        entity.insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgba(0.8, 0.2, 0.1, 0.5).into()),
            transform: Transform::from_translation(x.location),
            ..default()
        });
    }

    for (entity, mut transform, explosion) in query.iter_mut() {
        if explosion.lifetime_sec == 0. {
            continue;
        }
        let progress = (now - explosion.creation_time_sec) / explosion.lifetime_sec;

        if progress >= 1. {
            commands.entity(entity).despawn();
        }

        transform.scale = Vec3::ONE * progress;
    }
}
