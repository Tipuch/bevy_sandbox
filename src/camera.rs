use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{player::Player, tile::MainTileMap};

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
    println!("Spawning Camera2d...")
}

pub fn track_camera(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &Camera), With<MainCamera>>,
    tilemap_query: Query<(&TilemapSize, &TilemapGridSize, &GlobalTransform), With<MainTileMap>>,
) {
    if let Ok(player_global_transform) = player_query.single() {
        if let Ok((mut cam_transform, camera)) = camera_query.single_mut() {
            if let Ok((map_size, grid_size, map_global_transform)) = tilemap_query.single() {
                let player_pos = player_global_transform.translation();
                let mut target_cam_x = player_pos.x;
                let mut target_cam_y = player_pos.y;

                if let Some(projection) = camera.logical_viewport_size() {
                    let cam_half_width = projection.x / 2.0;
                    let cam_half_height = projection.y / 2.0;

                    let map_world_min_x = map_global_transform.translation().x
                        - (map_size.x as f32 * grid_size.x / 2.0);
                    let map_world_max_x = map_world_min_x + map_size.x as f32 * grid_size.x;
                    let map_world_min_y = map_global_transform.translation().y
                        - (map_size.y as f32 * grid_size.y / 2.0);
                    let map_world_max_y = map_world_min_y + map_size.y as f32 * grid_size.y;
                    println!(
                        "map_world_min_x: {}, map_world_max_x: {}, map_world_min_y: {}, map_world_max_y: {}",
                        map_world_min_x, map_world_max_x, map_world_min_y, map_world_max_y
                    );

                    let cam_min_x = map_world_min_x + cam_half_width;
                    let cam_max_x = map_world_max_x - cam_half_width;
                    let cam_min_y = map_world_min_y + cam_half_height;
                    let cam_max_y = map_world_max_y - cam_half_height;

                    if cam_min_x > cam_max_x {
                        target_cam_x = (map_world_min_x + map_world_max_x) / 2.0;
                    } else {
                        target_cam_x = target_cam_x.clamp(cam_min_x, cam_max_x);
                    }

                    if cam_min_y > cam_max_y {
                        target_cam_y = (map_world_min_y + map_world_max_y) / 2.0;
                    } else {
                        target_cam_y = target_cam_y.clamp(cam_min_y, cam_max_y);
                    }
                    cam_transform.translation.x = target_cam_x;
                    cam_transform.translation.y = target_cam_y;
                }
            }
        }
    }
}
