use bevy::prelude::*;

use crate::ascii::{spawn_sprite, AsciiSheet};
use crate::TILE_SIZE;

pub struct UIPlugin;

#[derive(Component)]
pub struct oxygen_level;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_ui);
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
    commands.entity(camera_entity).push_children(&entities);
}
