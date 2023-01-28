#![allow(clippy::type_complexity)]

mod arena;
mod cameras;
mod enemy;
mod explosion;
mod physics;
mod player;
mod ui;

use bevy::prelude::*;
use bevy_debug_text_overlay::OverlayPlugin;

use arena::*;
use cameras::*;
use enemy::*;
use explosion::*;
use physics::*;
use player::*;
use ui::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(OverlayPlugin {
            font_size: 32.0,
            ..default()
        })
        .init_resource::<SurvivalTime>()
        .add_startup_system(arena::setup)
        .add_startup_system(player::setup)
        .add_startup_system(cameras::setup)
        .add_startup_system(enemy::setup)
        .add_system(survival_time_system)
        .add_system(rotate)
        .add_system(player_location_system)
        .add_system(camera_select_system)
        .add_system(player_shoot_system)
        .add_system(projectile_physics_system)
        .add_system(explosion_system)
        .add_system(spawn_enemy_system)
        .add_system(enemy_movement_system)
        .add_system(enemy_shooting_system)
        .add_system(bevy::window::close_on_esc)
        .add_event::<ExplosionEvent>()
        .run();
}
