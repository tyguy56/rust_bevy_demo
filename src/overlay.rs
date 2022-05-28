use bevy::prelude::*;

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App){
        app
            .add_startup_system(create_overlay);
    }
}

fn create_overlay(commands: Commands, assets: Res<AssetServer>){

}