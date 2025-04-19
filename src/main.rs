use bevy::prelude::*;
use bevy_quadtree::{CollisionCircle, CollisionRect, QuadTreePlugin};
use leafwing_input_manager::plugin::InputManagerPlugin;
use player::{handle_movement, start_movement};
use tile::update_screen_bounds;

mod actions;
mod player;
mod tile;

fn main() {
    App::new()
        .insert_resource(tile::ScreenBounds::default())
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(QuadTreePlugin::<
            (
                (CollisionCircle, GlobalTransform),
                (CollisionRect, (GlobalTransform, Sprite)),
            ),
            40,
            8,
            100,
            100,
            0,
            0,
            20,
            114514,
        >::default())
        .add_plugins(InputManagerPlugin::<actions::MoveAction>::default())
        .add_systems(
            Startup,
            (
                setup,
                tile::update_screen_bounds.after(setup),
                tile::spawn_tiles.after(update_screen_bounds),
                player::spawn_player.after(update_screen_bounds),
            ),
        )
        .add_systems(Update, handle_movement)
        .add_systems(FixedUpdate, start_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    println!("Spawning Camera2d...")
}
