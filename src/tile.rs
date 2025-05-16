use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize};
use bevy_quadtree::CollisionRect;

pub const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct MainTileMap;

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("sprites/texture_pack/untitled.tmx");

    commands.spawn((
        TiledMapHandle(map_handle),
        TilemapAnchor::Center,
        MainTileMap,
        TilemapGridSize {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
    ));
}

pub fn process_loaded_tiled_maps(
    mut commands: Commands,
    tiled_map_assets: Res<Assets<TiledMap>>,
    mut tiled_map_events: EventReader<TiledMapCreated>,
    maps_query: Query<Entity, (With<MainTileMap>, Without<TilemapSize>)>,
) {
    for entity in maps_query.iter() {
        for event in tiled_map_events.read() {
            if let Some(tiled_map) = event.get_map_asset(&tiled_map_assets) {
                commands.entity(entity).insert(tiled_map.tilemap_size);
            }
        }
    }
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

#[derive(Default, Debug, Clone, Reflect)]
#[reflect(Default, Debug)]
pub struct QuadTreePhysicsBackend;

// This simple example will just spawn an entity with a `MyCustomPhysicsComponent` Component,
// at the center of where the Tiled collider is.
impl TiledPhysicsBackend for QuadTreePhysicsBackend {
    fn spawn_colliders(
        &self,
        commands: &mut Commands,
        _tiled_map: &TiledMap,
        _filter: &TiledNameFilter,
        collider: &TiledCollider,
        _anchor: &TilemapAnchor,
    ) -> Vec<TiledColliderSpawnInfos> {
        match collider {
            TiledCollider::Object {
                layer_id: _,
                object_id: _,
            } => {
                vec![TiledColliderSpawnInfos {
                    name: String::from("Colliders[Object]"),
                    entity: commands
                        .spawn(CollisionRect::new(Rect::from_corners(
                            Vec2::default(),
                            Vec2::splat(TILE_SIZE),
                        )))
                        .id(),
                    transform: Transform::default(),
                }]
            }
            TiledCollider::TilesLayer { layer_id: _ } => {
                vec![]
            }
        }
    }
}

// TODO add resource to get the tile map limits max_x and max_y min_x min_y such that the camera
// cannot show outside of that.
