mod camera;
mod debug;
mod movement;
mod collision_detection;
mod spaceship;
mod asteroids;
mod asset_loader;
mod despawn;

use bevy::prelude::*;
use asset_loader::AssetLoaderPlugin;
use movement::MovementPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
// use debug::DebugPlugin;
use spaceship::SpaceshipPlugin;
use asteroids::AsteroidPlugin;
use despawn::DespawnPlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        // .add_plugins(DebugPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(DespawnPlugin)
        .run();
}
