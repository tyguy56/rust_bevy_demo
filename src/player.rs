use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;
use coord_transforms::d2::cartesian2polar;
use coord_transforms::prelude::Vector2;

use crate::{
    ascii::{spawn_sprite, AsciiSheet},
    tileMap::{Collisions, Tile, TileType},
    ui::oxygen_level,
    GameState, TILE_SIZE,
};
#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    grab_distance: f32,
    oxygen_level: f32,
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Overworld)
                .with_system(player_movement.label("movement"))
                .with_system(camera_follow.after("movement"))
                .with_system(reduce_oxygen),
        )
        .add_startup_system(spawn_player);
    }
}

fn reduce_oxygen(
    mut oxygen_query: Query<&mut Transform, With<oxygen_level>>,
    mut player_query: Query<&mut Player>,
) {
    let mut oxygen = oxygen_query.single_mut();
    let mut player = player_query.single_mut();
    player.oxygen_level = player.oxygen_level.clamp(0.0, 100.0);
    oxygen.scale = Vec3::new(
        oxygen.scale.x,
        player.oxygen_level / 5714.28,
        oxygen.scale.z,
    );
    oxygen.translation.y = -0.8 - (player.oxygen_level / -1250.0);

    player.oxygen_level -= 0.0025;
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    bound_query: Query<&Tile>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    collisions: Res<Collisions>,
) {
    let (player, mut transform) = player_query.single_mut();

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::W) {
        y_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        y_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) {
        x_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        x_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if wall_collision_check(target, &bound_query, &collisions) {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target, &bound_query, &collisions) {
        transform.translation = target;
    }

    let vec = Vector2::new(-x_delta as f64, -y_delta as f64);

    let player_rotation = cartesian2polar(&vec);

    let rot = Quat::from_rotation_z(player_rotation[1] as f32);
    if player_rotation[0] != 0.000 {
        transform.rotation = rot;
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    bound_query: &Query<&Tile>,
    collisions: &Res<Collisions>,
) -> bool {
    for wall_type in bound_query.iter() {
        if wall_type.tileType == TileType::Collider && collisions.allow_collisions {
            let collision = collide(
                target_player_pos,
                Vec2::splat(TILE_SIZE * 0.5),
                Vec3::new(wall_type.dimension.pos_x, wall_type.dimension.pos_y, 0.0),
                Vec2::new(
                    wall_type.dimension.width * TILE_SIZE,
                    wall_type.dimension.height * TILE_SIZE,
                ),
            );
            if collision.is_some() {
                return false;
            }
        }
    }
    true
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_sprite(
        &mut commands,
        &ascii,
        10,
        Vec3::new(0.0, 0.0, 900.0),
        Vec2::splat(TILE_SIZE * 2.0),
        Color::default(),
        Vec3::splat(1.0),
    );

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 5.0,
            grab_distance: 0.1,
            oxygen_level: 100.0,
        });
}
