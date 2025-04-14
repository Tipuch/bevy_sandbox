use bevy::prelude::*;
use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query},
    },
    input::keyboard::KeyCode,
    sprite::Sprite,
};
use leafwing_input_manager::{
    InputManagerBundle,
    prelude::{ActionState, InputMap},
};

use crate::actions::Action;

const PIXEL_SCALE: f32 = 2.0;
const VELOCITY: f32 = 3.0;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let input_map = InputMap::new([
        (Action::MoveForward, KeyCode::KeyW),
        (Action::MoveBackward, KeyCode::KeyS),
        (Action::MoveLeft, KeyCode::KeyA),
        (Action::MoveRight, KeyCode::KeyD),
    ]);
    commands
        .spawn(InputManagerBundle::with_map(input_map))
        .insert(Player)
        .insert(Sprite::from_image(asset_server.load("sprites/test.png")))
        .insert(Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_SCALE)));
}

pub fn move_forward(mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>) {
    if let Ok((action_state, mut transform)) = query.get_single_mut() {
        if action_state.pressed(&Action::MoveForward) {
            transform.translation.y += 1.0 * VELOCITY;
        }
    }
}

pub fn move_backward(mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>) {
    if let Ok((action_state, mut transform)) = query.get_single_mut() {
        if action_state.pressed(&Action::MoveBackward) {
            transform.translation.y -= 1.0 * VELOCITY;
        }
    }
}

pub fn move_left(mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>) {
    if let Ok((action_state, mut transform)) = query.get_single_mut() {
        if action_state.pressed(&Action::MoveLeft) {
            transform.translation.x -= 1.0 * VELOCITY;
        }
    }
}

pub fn move_right(mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>) {
    if let Ok((action_state, mut transform)) = query.get_single_mut() {
        if action_state.pressed(&Action::MoveRight) {
            transform.translation.x += 1.0 * VELOCITY;
        }
    }
}
