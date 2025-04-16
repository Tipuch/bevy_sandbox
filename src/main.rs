use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use player::{handle_movement, start_movement};

mod actions;
mod player;
mod tile;

fn main() {
    App::new()
        .insert_resource(tile::ScreenBounds::default())
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(InputManagerPlugin::<actions::MoveAction>::default())
        .add_systems(Startup, (setup, player::spawn_player))
        .add_systems(Update, (tile::update_screen_bounds, handle_movement))
        .add_systems(FixedUpdate, start_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
