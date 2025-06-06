use std::time::Duration;

use bevy::ecs::system::QueryLens;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        system::{Commands, Query},
    },
    input::keyboard::KeyCode,
    sprite::Sprite,
};
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize};
use bevy_quadtree::{CollisionRect, Contain, Contained, Overlap, QOr, QuadTree};
use leafwing_input_manager::prelude::{ActionState, InputMap};

use crate::actions::{MoveAction, get_animation_from_action};
use crate::tile::{MainTileMap, TILE_SIZE, get_tile_to_world, get_world_to_tile};

const PIXEL_SCALE: f32 = 1.0;
const VELOCITY: f32 = 8.0;
const CHARACTER_HITBOX_SIZE: Vec2 = Vec2::new(30.0, 23.0);

type CollisionQuadTree = QuadTree<1>;

#[derive(Component)]
pub struct Player {
    tile_position: IVec2,
    z: f32,
}

#[derive(Component)]
pub struct Movement {
    start: Vec3,
    target: Vec3,
    progress: f32,
    speed: f32,
}

#[derive(Component)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first_sprite_index: usize, last_sprite_index: usize, fps: u8) -> Self {
        Self {
            first_sprite_index,
            last_sprite_index,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let input_map = InputMap::new([
        (MoveAction::Forward, KeyCode::KeyW),
        (MoveAction::Backward, KeyCode::KeyS),
        (MoveAction::Left, KeyCode::KeyA),
        (MoveAction::Right, KeyCode::KeyD),
    ]);

    let texture_handle: Handle<Image> = asset_server.load("sprites/char.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        13,
        54,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    if let Ok(window) = windows.single() {
        let tile_pos = IVec2::new(0, 0);
        let world_pos = get_tile_to_world(tile_pos, window);
        commands
            .spawn(input_map)
            .insert(Player {
                tile_position: tile_pos,
                z: 1.0,
            })
            .insert(CollisionRect::from(Rect::from_center_size(
                Vec2::new(world_pos.x, world_pos.y),
                CHARACTER_HITBOX_SIZE,
            )))
            .insert(Sprite {
                image: texture_handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 26,
                }),
                anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, -0.25)),
                ..Default::default()
            })
            .insert(
                Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 1.0))
                    .with_scale(Vec3::splat(PIXEL_SCALE)),
            );
    }
}

pub fn start_movement(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &ActionState<MoveAction>, &Player), Without<Movement>>,
    tilemap_query: Query<(&TilemapSize, &TilemapGridSize, &GlobalTransform), With<MainTileMap>>,
    quadtree: Res<CollisionQuadTree>,
) {
    if let Ok((entity, action_state, player)) = query.single_mut() {
        let mut destination_x: f32 = 0.0;
        let mut destination_y: f32 = 0.0;
        if action_state.pressed(&MoveAction::Forward) {
            destination_y += 1.0;
        }
        if action_state.pressed(&MoveAction::Backward) {
            destination_y -= 1.0;
        }
        if action_state.pressed(&MoveAction::Left) {
            destination_x -= 1.0;
        }
        if action_state.pressed(&MoveAction::Right) {
            destination_x += 1.0;
        }
        if destination_x == 0.0 && destination_y == 0.0 {
            return;
        }
        if let Ok((map_size, grid_size, map_global_transform)) = tilemap_query.single() {
            let map_world_min_x =
                map_global_transform.translation().x - (map_size.x as f32 * grid_size.x / 2.0);
            let map_world_max_x = map_world_min_x + map_size.x as f32 * grid_size.x;
            let map_world_min_y =
                map_global_transform.translation().y - (map_size.y as f32 * grid_size.y / 2.0);
            let map_world_max_y = map_world_min_y + map_size.y as f32 * grid_size.y;

            if let Ok(window) = windows.single() {
                let is_diagonal = destination_x != 0.0 && destination_y != 0.0;
                let start_2d = get_tile_to_world(player.tile_position, window);
                let start = Vec3::new(start_2d.x, start_2d.y, player.z);
                let target = Vec3::new(
                    (start.x + (destination_x * TILE_SIZE)).clamp(map_world_min_x, map_world_max_x),
                    (start.y + (destination_y * TILE_SIZE)).clamp(map_world_min_y, map_world_max_y),
                    player.z,
                );
                let collision_query = quadtree.query::<QOr<(Overlap, Contain, Contained)>>(
                    &CollisionRect::from(Rect::from_center_size(
                        Vec2::new(target.x, target.y),
                        Vec2::new(15.0, 15.0),
                    )),
                );
                if collision_query.is_empty() {
                    commands.entity(entity).insert(Movement {
                        start,
                        target,
                        progress: 0.0,
                        speed: if is_diagonal {
                            VELOCITY / 2.0f32.sqrt()
                        } else {
                            VELOCITY
                        },
                    });
                }
            }
        }
    }
}

pub fn handle_movement(
    time: Res<Time>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &mut Transform, &mut Movement, &mut Player)>,
) {
    if let Ok((entity, mut transform, mut movement, mut player)) = query.single_mut() {
        movement.progress += time.delta_secs() * movement.speed;
        movement.progress = movement.progress.min(1.0);
        if let Ok(window) = windows.single() {
            //TODO add Z to the equation, in case we're going up or down
            let start = Vec3::new(movement.start.x, movement.start.y, movement.start.z);
            let end = Vec3::new(movement.target.x, movement.target.y, movement.target.z);
            transform.translation = start.lerp(end, movement.progress);
            if movement.progress >= 1.0 {
                player.tile_position =
                    get_world_to_tile(Vec2::new(movement.target.x, movement.target.y), window);
                commands.entity(entity).remove::<Movement>();
            }
        }
    }
}

pub fn start_movement_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &ActionState<MoveAction>)>,
    mut animation_query: Query<(Option<&AnimationConfig>, &mut Sprite, &Player)>,
) {
    let mut query_with_animation: QueryLens<(
        Entity,
        &ActionState<MoveAction>,
        &mut Sprite,
        Option<&AnimationConfig>,
    )> = query.join_filtered(&mut animation_query);
    if let Ok((entity, action_state, mut sprite, animation_config_option)) =
        query_with_animation.query().single_mut()
    {
        if action_state.pressed(&MoveAction::Forward) {
            overwrite_animation(
                commands,
                sprite.reborrow(),
                animation_config_option,
                entity,
                get_animation_from_action(MoveAction::Forward),
            );
        } else if action_state.pressed(&MoveAction::Backward) {
            overwrite_animation(
                commands,
                sprite.reborrow(),
                animation_config_option,
                entity,
                get_animation_from_action(MoveAction::Backward),
            );
        } else if action_state.pressed(&MoveAction::Left) {
            overwrite_animation(
                commands,
                sprite.reborrow(),
                animation_config_option,
                entity,
                get_animation_from_action(MoveAction::Left),
            );
        } else if action_state.pressed(&MoveAction::Right) {
            overwrite_animation(
                commands,
                sprite.reborrow(),
                animation_config_option,
                entity,
                get_animation_from_action(MoveAction::Right),
            );
        } else if let Ok((Some(animation_config), mut sprite, _player)) =
            animation_query.single_mut()
        {
            {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = animation_config.first_sprite_index;
                    commands.entity(entity).remove::<AnimationConfig>();
                }
            }
        }
    }
}

fn overwrite_animation(
    mut commands: Commands,
    mut sprite: Mut<'_, Sprite>,
    animation_config_option: Option<&AnimationConfig>,
    entity: Entity,
    new_animation_config: AnimationConfig,
) {
    if let Some(animation_config) = animation_config_option {
        if animation_config.first_sprite_index != new_animation_config.first_sprite_index {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animation_config.first_sprite_index;
                commands.entity(entity).remove::<AnimationConfig>();
            }
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = new_animation_config.first_sprite_index;
            }
            commands.entity(entity).insert(new_animation_config);
        }
    } else {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = new_animation_config.first_sprite_index;
        }
        commands.entity(entity).insert(new_animation_config);
    }
}

// timer not working iterating more than x times per second
pub fn execute_movement_animation(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut AnimationConfig)>,
) {
    if let Ok((entity, mut sprite, mut animation_config)) = query.single_mut() {
        animation_config.frame_timer.tick(time.delta());
        if animation_config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == animation_config.last_sprite_index {
                    atlas.index = animation_config.first_sprite_index;
                    commands.entity(entity).remove::<AnimationConfig>();
                } else {
                    atlas.index += 1;
                    animation_config.frame_timer =
                        AnimationConfig::timer_from_fps(animation_config.fps);
                }
            }
        }
    }
}
