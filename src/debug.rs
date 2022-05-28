use bevy::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};

use crate::player::Player;
use crate::tileMap::{BoundingEntity, Collisions, Dimension, Tile, TileCollider};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(InspectorPlugin::<Collisions>::new())
                .add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<TileCollider>()
                .register_inspectable::<Dimension>()
                .register_inspectable::<BoundingEntity>()
                .register_inspectable::<Tile>()
                .register_inspectable::<Player>();
        }
    }
}
