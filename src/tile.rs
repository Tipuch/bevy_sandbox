use bevy::{prelude::*, window::PrimaryWindow};

pub const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Tile {
    pub tile_x: usize,
    pub tile_y: usize,
    pub z: f32,
    pub walkable: bool,
}

pub fn spawn_tiles(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    println!("Spawning tiles...");
    let texture_handle: Handle<Image> =
        asset_server.load("sprites/texture_pack/TX Tileset Grass.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 8, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    if let Ok(window) = windows.get_single() {
        let tile_coordinates = get_tile_to_world(IVec2 { x: 0, y: 0 }, window);
        commands.spawn((
            Sprite::from_atlas_image(
                texture_handle,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 3,
                },
            ),
            Transform {
                translation: Vec3::new(tile_coordinates.x, tile_coordinates.y, 0.0),
                scale: Vec3::splat(1.0),
                ..default()
            },
            Tile {
                tile_x: 0,
                tile_y: 0,
                z: 0.0,
                walkable: true,
            },
        ));
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
