use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub radius: f32,
    pub next_waypoint: Option<Vec3>,
}

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
    ));
}

pub fn enemy_movement_system(mut query: Query<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
    const MPS: f32 = 5.0;
    const DPS: f32 = 180.0;

    const WAYPOINT_RADIUS_M: f32 = 1.0;

    for (mut loc, mut enemy) in query.iter_mut() {
        if let Some(x) = enemy.next_waypoint {
            let to_waypoint = x - loc.translation;

            if to_waypoint.length() > WAYPOINT_RADIUS_M {
                if loc.forward().cross(to_waypoint).dot(Vec3::Z) > 0. {
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
            } else {
                //new waypoint
                enemy.next_waypoint = Some(Vec3::new(-4., -4., 0.));
            }
        }
    }
}
