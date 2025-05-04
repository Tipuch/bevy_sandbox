use bevy::reflect::Reflect;
use leafwing_input_manager::Actionlike;

use crate::player::AnimationConfig;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum MoveAction {
    Forward,
    Backward,
    Left,
    Right,
}

pub fn get_animation_from_action(move_action: MoveAction) -> AnimationConfig {
    match move_action {
        MoveAction::Forward => AnimationConfig::new(104, 112, 9),
        MoveAction::Backward => AnimationConfig::new(130, 138, 9),
        MoveAction::Left => AnimationConfig::new(117, 125, 9),
        MoveAction::Right => AnimationConfig::new(143, 151, 9),
    }
}
