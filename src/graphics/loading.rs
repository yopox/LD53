use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Title),
            )
            .add_collection_to_loading_state::<_, Textures>(GameState::Loading)
            .add_collection_to_loading_state::<_, Fonts>(GameState::Loading)
            .add_collection_to_loading_state::<_, Ost>(GameState::Loading)
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

#[derive(AssetCollection, Resource)]
pub struct Ost {
    // BGM
    #[asset(path = "ost/title-1.ogg")]
    pub bgm_title: Handle<AudioSource>,
    #[asset(path = "ost/theme normal.ogg")]
    pub bgm_theme: Handle<AudioSource>,
    #[asset(path = "ost/theme madness.ogg")]
    pub bgm_theme_madness: Handle<AudioSource>,
    #[asset(path = "ost/boss.ogg")]
    pub bgm_boss: Handle<AudioSource>,
    #[asset(path = "ost/pause [delay 1.5s].ogg")]
    pub bgm_pause: Handle<AudioSource>,

    // SFX
    #[asset(path = "sfx/base damage/sfx hit base 1.ogg")]
    pub sfx_hit1: Handle<AudioSource>,
    #[asset(path = "sfx/base damage/sfx hit base 2.ogg")]
    pub sfx_hit2: Handle<AudioSource>,
    #[asset(path = "sfx/base damage/sfx hit base 3.ogg")]
    pub sfx_hit3: Handle<AudioSource>,

    #[asset(path = "sfx/packages interactions/sfx MALUS.ogg")]
    pub sfx_package_malus: Handle<AudioSource>,
    #[asset(path = "sfx/packages interactions/sfx MONEY.ogg")]
    pub sfx_package_bonus: Handle<AudioSource>,

    #[asset(path = "sfx/tower effects/sfx aura.ogg")]
    pub sfx_tower_aura: Handle<AudioSource>,
    #[asset(path = "sfx/tower effects/sfx bombe.ogg")]
    pub sfx_bomb: Handle<AudioSource>,
    #[asset(path = "sfx/tower effects/sfx projectile rapide.ogg")]
    pub sfx_shot: Handle<AudioSource>,

    #[asset(path = "sfx/tower interactions/sfx place tower.ogg")]
    pub sfx_place_tower: Handle<AudioSource>,
    #[asset(path = "sfx/tower interactions/sfx sell tower.ogg")]
    pub sfx_sell_tower: Handle<AudioSource>,
    #[asset(path = "sfx/tower interactions/sfx upgrade tower.ogg")]
    pub sfx_upgrade_tower: Handle<AudioSource>,

    #[asset(path = "sfx/sfx game over.ogg")]
    pub sfx_game_over: Handle<AudioSource>,
    #[asset(path = "sfx/sfx PAUSE.ogg")]
    pub sfx_pause: Handle<AudioSource>,
}