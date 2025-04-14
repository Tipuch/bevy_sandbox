use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use player::{move_backward, move_forward, move_left, move_right};

mod actions;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(InputManagerPlugin::<actions::Action>::default())
        .add_systems(Startup, (setup, player::spawn_player))
        .add_systems(
            FixedUpdate,
            (move_forward, move_backward, move_left, move_right),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
