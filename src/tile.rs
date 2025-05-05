use bevy::prelude::*;
use bevy::{
    color::palettes::css::{PURPLE, RED},
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_quadtree::CollisionRect;

pub const TILE_SIZE: f32 = 32.0;

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("sprites/texture_pack/untitled.tmx");
    commands.spawn((TiledMapHandle(map_handle), TiledMapAnchor::Center));
}

pub fn get_world_to_tile(world_pos: Vec2, window: &Window) -> IVec2 {
    let window_size = window.resolution.size();
    let offset = Vec2::new(window_size.x / 2.0, window_size.y / 2.0);
    let converted_world_pos = Vec2::new(world_pos.x + offset.x, world_pos.y + offset.y);
    let tile_x = (converted_world_pos.x / TILE_SIZE).floor() as i32;
    let tile_y = (converted_world_pos.y / TILE_SIZE).floor() as i32;
    IVec2::new(tile_x, tile_y)
}

pub fn get_tile_to_world(tile_pos: IVec2, window: &Window) -> Vec2 {
    let window_size = window.resolution.size();
    let offset = Vec2::new(window_size.x / 2.0, window_size.y / 2.0);
    let world_x = (tile_pos.x as f32 * TILE_SIZE) + (TILE_SIZE / 2.0) - offset.x;
    let world_y = (tile_pos.y as f32 * TILE_SIZE) + (TILE_SIZE / 2.0) - offset.y;
    Vec2::new(world_x, world_y)
}

// A typical usecase for regular events is to update components associated with tiles, objects or layers.
// Here, we will add a small offset on the Z axis to our objects to prevent them
// from Z-fighting if they are on the same layer (by default, all objects on a given layer have the same Z offset)
fn evt_object_created(
    mut object_events: EventReader<TiledObjectCreated>,
    mut object_query: Query<(&Name, &mut Transform), With<TiledMapObject>>,
    mut z_offset: Local<f32>,
) {
    for e in object_events.read() {
        let Ok((name, mut transform)) = object_query.get_mut(e.entity) else {
            return;
        };

        info!("=> Received TiledObjectCreated event for object '{}'", name);

        // Obviously, this is a very naive implementation and you would
        // probably want to do something else in a real usecase
        info!("Apply z-offset = {:?}", *z_offset);
        transform.translation.z += *z_offset;
        *z_offset += 0.01;
    }
}

// TODO add resource to get the tile map limits max_x and max_y min_x min_y such that the camera
// cannot show outside of that.
