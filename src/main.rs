use bevy::prelude::*;
use bevy_ecs_tiled::{TiledMapPlugin, prelude::TiledPhysicsPlugin};
use bevy_quadtree::{CollisionCircle, CollisionRect, QuadTreePlugin};
use camera::{setup_camera, track_camera};
use leafwing_input_manager::plugin::InputManagerPlugin;
use player::{
    execute_movement_animation, handle_movement, start_movement, start_movement_animation,
};
use tile::QuadTreePhysicsBackend;

mod actions;
mod camera;
mod player;
mod tile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(QuadTreePlugin::<
            (
                (CollisionCircle, GlobalTransform),
                (CollisionRect, GlobalTransform),
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
        .add_plugins(TiledPhysicsPlugin::<QuadTreePhysicsBackend>::default())
        .add_plugins(InputManagerPlugin::<actions::MoveAction>::default())
        .add_systems(
            Startup,
            (setup_camera, tile::spawn_map, player::spawn_player),
        )
        .add_systems(
            Update,
            (
                handle_movement,
                start_movement_animation,
                execute_movement_animation,
            ),
        )
        .add_systems(PostUpdate, track_camera)
        .add_systems(
            FixedUpdate,
            (start_movement, tile::process_loaded_tiled_maps),
        )
        .run();
}
