use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        system::{Commands, Query},
    },
    input::keyboard::KeyCode,
    sprite::Sprite,
};
use leafwing_input_manager::{
    InputManagerBundle,
    prelude::{ActionState, InputMap},
};

use crate::actions::MoveAction;
use crate::tile::{TILE_SIZE, get_tile_to_world, get_world_to_tile};

const PIXEL_SCALE: f32 = 1.0;
const VELOCITY: f32 = 8.0;

#[derive(Component)]
pub struct Player {
    tile_position: IVec2,
    z: f32,
}

#[derive(Component)]
pub struct Movement {
    start: Vec3,
    target: Vec3,
    progress: f32,
    speed: f32,
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let input_map = InputMap::new([
        (MoveAction::Forward, KeyCode::KeyW),
        (MoveAction::Backward, KeyCode::KeyS),
        (MoveAction::Left, KeyCode::KeyA),
        (MoveAction::Right, KeyCode::KeyD),
    ]);
    commands
        .spawn(InputManagerBundle::with_map(input_map))
        .insert(Player {
            tile_position: IVec2::new(0, 0),
            z: 1.0,
        })
        .insert(Sprite::from_image(asset_server.load("sprites/test.png")))
        .insert(
            Transform::from_translation(Vec3::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0, 1.0))
                .with_scale(Vec3::splat(PIXEL_SCALE)),
        );
}

pub fn start_movement(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &ActionState<MoveAction>, &Player), Without<Movement>>,
) {
    if let Ok((entity, action_state, player)) = query.get_single_mut() {
        let mut destination_x: f32 = 0.0;
        let mut destination_y: f32 = 0.0;
        if action_state.pressed(&MoveAction::Forward) {
            destination_y += 1.0;
        }
        if action_state.pressed(&MoveAction::Backward) {
            destination_y -= 1.0;
        }
        if action_state.pressed(&MoveAction::Left) {
            destination_x -= 1.0;
        }
        if action_state.pressed(&MoveAction::Right) {
            destination_x += 1.0;
        }
        if destination_x == 0.0 && destination_y == 0.0 {
            return;
        }
        if let Ok(window) = windows.get_single() {
            let is_diagonal = destination_x != 0.0 && destination_y != 0.0;
            let start_2d = get_tile_to_world(player.tile_position, window);
            let start = Vec3::new(start_2d.x, start_2d.y, player.z);
            let target = Vec3::new(
                start.x + (destination_x * TILE_SIZE),
                start.y + (destination_y * TILE_SIZE),
                player.z,
            );
            commands.entity(entity).insert(Movement {
                start,
                target,
                progress: 0.0,
                speed: if is_diagonal {
                    VELOCITY / 2.0f32.sqrt()
                } else {
                    VELOCITY
                },
            });
        }
    }
}

pub fn handle_movement(
    time: Res<Time>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &mut Transform, &mut Movement, &mut Player)>,
) {
    if let Ok((entity, mut transform, mut movement, mut player)) = query.get_single_mut() {
        movement.progress += time.delta_secs() * movement.speed;
        movement.progress = movement.progress.min(1.0);
        if let Ok(window) = windows.get_single() {
            //TODO add Z to the equation, in case we're going up or down
            let start = Vec3::new(movement.start.x, movement.start.y, movement.start.z);
            let end = Vec3::new(movement.target.x, movement.target.y, movement.target.z);
            transform.translation = start.lerp(end, movement.progress);
            if movement.progress >= 1.0 {
                player.tile_position =
                    get_world_to_tile(Vec2::new(movement.target.x, movement.target.y), window);
                commands.entity(entity).remove::<Movement>();
            }
        }
    }
}
