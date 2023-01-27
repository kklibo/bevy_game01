use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_debug_text_overlay::screen_print;

use crate::Player;

#[derive(Resource, Default)]
pub struct SurvivalTime(pub Stopwatch);

pub fn survival_time_system(
    time: Res<Time>,
    mut stopwatch: ResMut<SurvivalTime>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let elapsed = stopwatch.0.elapsed_secs();
    screen_print!("survival time: {elapsed:.2}");

    if query.iter_mut().next().is_some() {
        stopwatch.0.tick(time.delta());
    }
}
