use bevy::prelude::*;
use bevy_quadtree::{CollisionCircle, CollisionRect, QuadTreePlugin};
use leafwing_input_manager::plugin::InputManagerPlugin;
use player::{
    execute_movement_animation, handle_movement, start_movement, start_movement_animation,
};

mod actions;
mod player;
mod tile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(QuadTreePlugin::<
            (
                (CollisionCircle, GlobalTransform),
                (CollisionRect, (GlobalTransform)),
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
        .add_systems(
            Update,
            (
                handle_movement,
                start_movement_animation,
                execute_movement_animation,
            ),
        )
        .add_systems(FixedUpdate, start_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    println!("Spawning Camera2d...")
}
