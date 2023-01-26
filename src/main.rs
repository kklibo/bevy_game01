mod arena;
mod cameras;
mod enemy;
mod explosion;
mod physics;
mod player;

use bevy::prelude::*;

use arena::*;
use cameras::*;
use enemy::*;
use explosion::*;
use physics::*;
use player::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(arena::setup)
        .add_startup_system(player::setup)
        .add_startup_system(cameras::setup)
        .add_startup_system(enemy::setup)
        .add_system(rotate)
        .add_system(player_location_system)
        .add_system(camera_select_system)
        .add_system(player_shoot_system)
        .add_system(projectile_physics_system)
        .add_system(explosion_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}
