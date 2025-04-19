use bevy::prelude::*;

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
    screen_bounds: Res<ScreenBounds>,
) {
    println!("Spawning tiles...");
    let texture_handle: Handle<Image> =
        asset_server.load("sprites/texture_pack/TX Tileset Grass.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 8, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let tile_coordinates = get_tile_to_world(IVec2 { x: 0, y: 0 }, &screen_bounds);
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

#[derive(Resource)]
pub struct ScreenBounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl Default for ScreenBounds {
    fn default() -> Self {
        Self {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(0.0, 0.0),
        }
    }
}

pub fn update_screen_bounds(
    mut screen_bounds: ResMut<ScreenBounds>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok(window) = windows.get_single() {
        let (camera, camera_transform) = camera_query.single();
        let window_size = Vec2::new(window.width(), window.height());
        if let Ok(bottom_left_cam) = camera.viewport_to_world(camera_transform, Vec2::new(0.0, 0.0))
        {
            let bottom_left = bottom_left_cam.origin.truncate();
            if let Ok(top_right_cam) = camera.viewport_to_world(camera_transform, window_size) {
                let top_right = top_right_cam.origin.truncate();
                screen_bounds.min = bottom_left;
                screen_bounds.max = top_right;
            }
        }
    }
}

pub fn get_world_to_tile(world_pos: Vec2, screen_bounds: &Res<ScreenBounds>) -> IVec2 {
    let offset_from_min = world_pos - screen_bounds.min;
    let tile_x = (offset_from_min.x / TILE_SIZE).floor() as i32;
    let tile_y = (offset_from_min.y / TILE_SIZE).floor() as i32;

    IVec2::new(tile_x, tile_y)
}

pub fn get_tile_to_world(tile_pos: IVec2, screen_bounds: &Res<ScreenBounds>) -> Vec2 {
    let world_x = screen_bounds.min.x + (tile_pos.x as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
    let world_y = screen_bounds.min.y + (tile_pos.y as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
    Vec2::new(world_x, world_y)
}
