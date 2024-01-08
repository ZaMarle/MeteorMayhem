use bevy::prelude::*;

use crate::{
    movement::{
        Acceleration, 
        MovingObjectBundle, 
        Velocity
    }, 
    asset_loader::SceneAssets, collision_detection::Collider
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;

#[derive(Component, Debug)]
pub struct SpaceShip;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, spaceship_movement_controls)
            .add_systems(Update, spaceship_weapon_controlls);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform {
                    translation: STARTING_TRANSLATION,
                    rotation: Quat::from_xyzw(0.0, 0.0, 1.0, 0.0),
                    scale: Vec3::new(0.01,0.01,0.01),
                    ..default()
                },
                ..default()
            },
            collider: Collider::new(3.0)
        }, 
        SpaceShip
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<SpaceShip>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (mut transform, mut velocity) = query.single_mut();

    let movement = match (
        keyboard_input.pressed(KeyCode::S), 
        keyboard_input.pressed(KeyCode::W),
    ) {
        (true, _) => -SPACESHIP_SPEED,
        (_, true) => SPACESHIP_SPEED,
        _ => 0.0,
    };
    
    let rotation = match (
        keyboard_input.pressed(KeyCode::A),
        keyboard_input.pressed(KeyCode::D),
    ) {
        (true, _) => -SPACESHIP_ROTATION_SPEED * time.delta_seconds(),
        (_, true) => SPACESHIP_ROTATION_SPEED * time.delta_seconds(),
        _ => 0.0
    };

    let  roll = match (
        keyboard_input.pressed(KeyCode::ShiftLeft),
        keyboard_input.pressed(KeyCode::ShiftRight),
    ) {
        (true, _) => -SPACESHIP_ROLL_SPEED * time.delta_seconds(),
        (_, true) => SPACESHIP_ROLL_SPEED * time.delta_seconds(),
        _ => 0.0
    };

    velocity.value = -transform.forward() * movement;
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
}

fn spaceship_weapon_controlls(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
    spaceship_query: Query<&Transform, With<SpaceShip>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        
        let transform = spaceship_query.single();
        println!("{:?}", transform.rotation);

        let velocity = transform.forward() * -25.0;
        
       commands.spawn((
            MovingObjectBundle{
                velocity: Velocity::new(velocity),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform{
                        translation: transform.translation + -transform.forward() * 4.0,
                        rotation: transform.rotation * Quat::from_xyzw(0.71, 0.0,0.0,0.71),
                        scale: Vec3::new(0.25, 0.25, 0.25),
                        ..default()
                    },
                    ..default()
                },
                collider: Collider::new(1.0)
            },
            SpaceshipMissile
        ));
    }
}
