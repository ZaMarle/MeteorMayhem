use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{movement::{MovingObjectBundle, Velocity, Acceleration}, asset_loader::SceneAssets, collision_detection::Collider};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating)
        })
            .add_systems(Update, spawn_asteroid)
            .add_systems(Update, rotate_asteroid)
            .add_systems(Update, handle_asteroid_collision);
    }
}

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 5.0;
const SPAWN_RANGE_X: Range<f32> = -20.0..20.0;
const SPAWN_RANGE_Z: Range<f32> = -20.0..20.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;

#[derive(Component)]
pub struct Asteroid;

#[derive(Resource)]
pub struct SpawnTimer {
    timer: Timer,
}

fn spawn_asteroid(
    mut commands: Commands, 
    mut spawn_timer: ResMut<SpawnTimer>, 
    time: Res<Time>, 
    scene_assets: Res<SceneAssets>
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let spawn_location = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector = 
        || Vec3::new(
            rng.gen_range(-1.0..1.0),
            0.0,
            rng.gen_range(-1.0..1.0),
        ).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            model: SceneBundle {
                scene: scene_assets.asteroid.clone(),
                transform: Transform::from_translation(spawn_location),
                ..default()
            },
            collider: Collider::new(2.0),
        },
        Asteroid,
    ));
}

fn rotate_asteroid(
    mut query: Query<&mut Transform, With<Asteroid>>, 
    time: Res<Time>
) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(5.0 * time.delta_seconds());
    }
}

fn handle_asteroid_collision(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // Asteroid colider with another asteroid.
            if query.get(collided_entity).is_ok() {
                continue;
            }

            // Despawn the asteriod.
            commands.entity(entity).despawn_recursive();
        }
    }
}
