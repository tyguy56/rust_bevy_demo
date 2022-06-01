use crate::ascii::{spawn_sprite, AsciiSheet};
use crate::TILE_SIZE;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
pub struct itemsPlugin;

#[derive(Component, Inspectable)]
pub struct Pickupable {
    pub description: ItemType,
}

impl Plugin for itemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_test)
            .add_startup_system(spawn_test2);
    }
}

fn spawn_test(commands: Commands, ascii: Res<AsciiSheet>) {
    spawn_item(
        commands,
        ascii,
        ItemType {
            sprite_index: (15),
            item: (item_list::fire_extinguisher),
        },
        Vec3::new(0.0, 0.0, 900.0),
    );
}
fn spawn_test2(commands: Commands, ascii: Res<AsciiSheet>) {
    spawn_item(
        commands,
        ascii,
        ItemType {
            sprite_index: (14),
            item: (item_list::oxygen_tank),
        },
        Vec3::new(2.0, 2.0, 900.0),
    );
}
#[derive(Debug, Inspectable, Clone, Copy, PartialEq, Eq)]
pub struct ItemType {
    pub sprite_index: usize,
    pub item: item_list,
}

#[derive(Debug, Inspectable, Copy, Clone, PartialEq, Eq)]
pub enum item_list {
    none,
    oxygen_tank,
    fire_extinguisher,
}

fn spawn_item(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    item_description: ItemType,
    location: Vec3,
) {
    let item = spawn_sprite(
        &mut commands,
        &ascii,
        item_description.sprite_index,
        location,
        Vec2::splat(TILE_SIZE),
        Color::default(),
        Vec3::splat(1.0),
    );

    commands.entity(item).insert(Pickupable {
        description: item_description,
    });
}
