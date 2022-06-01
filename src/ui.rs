use bevy::prelude::*;

use crate::ascii::{spawn_sprite, AsciiSheet};
use crate::items::item_list;
use crate::player::Inventory;
use crate::GameState;
use crate::TILE_SIZE;

pub struct UIPlugin;

#[derive(Component)]
pub struct oxygen_level;
#[derive(Component)]
pub struct item;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_ui)
            .add_system_set(
                SystemSet::on_update(GameState::Overworld).with_system(update_inventory_ui),
            );
    }
}

fn update_inventory_ui(
    inventory_query: Query<&Inventory>,
    mut inventory_slot: Query<(&mut Visibility, &mut TextureAtlasSprite), With<item>>,
) {
    let inventory = inventory_query.single();
    let (mut vis, mut texture) = inventory_slot.single_mut();
    if inventory.item.item == item_list::none {
        vis.is_visible = false;
    } else {
        vis.is_visible = true;
        texture.index = inventory.item.sprite_index;
    }
}

fn spawn_ui(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    camera_query: Query<Entity, With<Camera>>,
) {
    let camera_entity = camera_query.single();

    let mut entities = Vec::new();

    let oxygen_bar = spawn_sprite(
        &mut commands,
        &ascii,
        13,
        Vec3::new(-1.593, -0.72, -1.0),
        Vec2::new(4.0, -10.0),
        Color::default(),
        Vec3::new(0.018, 0.0175, 0.5),
    );

    commands.entity(oxygen_bar).insert(oxygen_level);

    entities.push(oxygen_bar);

    entities.push(spawn_sprite(
        &mut commands,
        &ascii,
        14,
        Vec3::new(-1.6, -0.7, -1.0),
        Vec2::splat(TILE_SIZE * 2.0),
        Color::default(),
        Vec3::new(0.5, 0.5, 0.5),
    ));

    let inventory = spawn_sprite(
        &mut commands,
        &ascii,
        15,
        Vec3::new(-1.4, -0.7, -1.0),
        Vec2::splat(TILE_SIZE * 2.0),
        Color::default(),
        Vec3::new(0.5, 0.5, 0.5),
    );

    commands.entity(inventory).insert(item);
    entities.push(inventory);

    commands.entity(camera_entity).push_children(&entities);
}
