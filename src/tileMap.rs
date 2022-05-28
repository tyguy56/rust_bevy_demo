use crate::TILE_SIZE;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use noise::{NoiseFn, Perlin};

use crate::{ascii::AsciiSheet, GameState};

pub struct TileMapPlugin;

#[derive(Component, Inspectable)]
pub struct BoundingEntity {
    pub sprite_box: Entity,
}

#[derive(Component, Inspectable, Default, Debug)]
pub struct Dimension {
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub pos_x: f32,
    pub pos_y: f32,
}

#[derive(Inspectable, Debug, Eq, PartialEq)]
pub enum TileType {
    Ground,
    Collider,
    Trigger,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Ground
    }
}

#[derive(Component, Default, Inspectable, Debug)]
pub struct Tile {
    pub tileType: TileType,
    pub dimension: Dimension,
}

#[derive(Component, Default, Inspectable)]
pub struct TileCollider {}

#[derive(Inspectable, Default)]
pub struct Collisions {
    pub show_collisions: bool,
    pub allow_collisions: bool,
}

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Overworld).with_system(show_collision_box),
        )
        .add_startup_system(create_map);
    }
}

fn show_collision_box(
    mut parent_query: Query<&Children>,
    mut child_query: Query<(&mut Visibility, &mut Tile, &mut Transform)>,
    collisions: Res<Collisions>,
) {
    for children in parent_query.iter_mut() {
        for child in children.iter() {
            if let Ok(block) = child_query.get_mut(*child) {
                let (mut vis, tile, mut transform) = block;
                if tile.tileType == TileType::Collider {
                    transform.scale = Vec3::new(tile.dimension.width, tile.dimension.height, 400.0);
                    transform.rotation = Quat::from_scaled_axis(Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        tile.dimension.rotation,
                    ));
                    vis.is_visible = collisions.show_collisions;
                }
            }
        }
    }
}

fn create_map(mut commands: Commands, ascii: Res<AsciiSheet>, time: Res<Time>) {
    let perlin = Perlin::new();
    let mut tiles = Vec::new();
    for y in 0..10 {
        for x in 0..10 {
            let val = (perlin
                .get([x as f64, y as f64, time.seconds_since_startup()])
                .abs()
                * 10.0)
                .floor();
            let tile = commands.spawn().id();
            let mut sprite = TextureAtlasSprite::new(0);
            sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
            commands
                .entity(tile)
                .insert(Tile {
                    tileType: TileType::Ground,
                    dimension: Dimension {
                        width: TILE_SIZE,
                        height: TILE_SIZE,
                        rotation: 0.0,
                        pos_x: x as f32 * TILE_SIZE,
                        pos_y: y as f32 * TILE_SIZE,
                    },
                })
                .insert_bundle(SpriteSheetBundle {
                    sprite: sprite,
                    texture_atlas: ascii.0.clone(),
                    transform: Transform {
                        translation: Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new(format!("{},{}", x, y)));

            if val == 5.0 {
                let mut sprite2 = TextureAtlasSprite::new(13);
                sprite2.custom_size = Some(Vec2::splat(TILE_SIZE));
                let collideSprite = commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: sprite2,
                        texture_atlas: ascii.0.clone(),
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 200.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Name::new("Collider"))
                    .insert(Tile {
                        tileType: TileType::Collider,
                        dimension: Dimension {
                            width: TILE_SIZE,
                            height: TILE_SIZE,
                            rotation: 0.0,
                            pos_x: x as f32 * TILE_SIZE,
                            pos_y: y as f32 * TILE_SIZE,
                        },
                    })
                    .id();

                commands.entity(tile).push_children(&[collideSprite]);
            }
            tiles.push(tile);
        }
    }
    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);

    commands.insert_resource(Collisions {
        show_collisions: false,
        allow_collisions: true,
    });
}
