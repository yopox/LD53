use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Main),
            )
            .add_collection_to_loading_state::<_, Textures>(GameState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct Textures {
    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 32, rows = 32, padding_x = 2., padding_y = 2.))]
    #[asset(path = "MRMOTEXT EX.png")]
    pub mrmotext: Handle<TextureAtlas>,
}