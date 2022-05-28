use crate::TILE_SIZE;
use bevy::prelude::*;

pub struct AsciiPlugin;

pub struct AsciiSheet(pub Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_asset);
    }
}

pub fn spawn_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    translation: Vec3,
    size: Vec2,
    color: Color,
    scale: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(size);
    sprite.color = color;

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: translation,
                scale: scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

fn load_asset(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("New Piskel (8).png");
    let atlas =
        TextureAtlas::from_grid_with_padding(image, Vec2::splat(30.0), 4, 4, Vec2::splat(4.0));

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}
