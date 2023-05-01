use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Battle),
            )
            .add_collection_to_loading_state::<_, Textures>(GameState::Loading)
            .add_collection_to_loading_state::<_, Fonts>(GameState::Loading)
        ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct Textures {
    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 32, rows = 32, padding_x = 0., padding_y = 0.))]
    #[asset(path = "tileset_Ado.png")]
    pub tileset: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct Fonts {
    #[asset(path = "Axones 6p.ttf")]
    pub axones: Handle<Font>,
    #[asset(path = "Yesterday 10h.ttf")]
    pub yesterday: Handle<Font>,
}