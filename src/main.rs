use bevy::prelude::*;
use bevy_quadtree::{CollisionCircle, CollisionRect, QuadTreePlugin};
use leafwing_input_manager::plugin::InputManagerPlugin;
use player::{handle_movement, start_movement};

mod actions;
mod player;
mod tile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(QuadTreePlugin::<
            (
                (CollisionCircle, GlobalTransform),
                (CollisionRect, (GlobalTransform, Sprite)),
            ),
            40,
            8,
            10000,
            10000,
            0,
            0,
            20,
            1,
        >::default())
        .add_plugins(InputManagerPlugin::<actions::MoveAction>::default())
        .add_systems(Startup, (setup, tile::spawn_tiles, player::spawn_player))
        .add_systems(Update, handle_movement)
        .add_systems(FixedUpdate, start_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    println!("Spawning Camera2d...")
}
