use bevy::app::{App, Plugin};
use bevy::asset::Handle;
use bevy::prelude::{Commands, EventReader, Res, Resource};
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, AudioSource};
use rand::{RngCore, thread_rng};

use crate::graphics::loading::Ost;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(bevy_kira_audio::AudioPlugin)
            .add_event::<PlayBgmEvent>()
            .add_event::<PlaySfxEvent>()
            .add_audio_channel::<BgmChannel>()
            .add_audio_channel::<SfxChannel>()
            .add_system(update)
        ;
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum BGM {
    Title,
    Theme,
    ThemeMadness,
    Boss,
    Pause,
}

impl BGM {
    fn handle(&self, ost: &Res<Ost>) -> Handle<AudioSource> {
        match self {
            BGM::Title => ost.bgm_title.clone(),
            BGM::Theme => ost.bgm_theme.clone(),
            BGM::ThemeMadness => ost.bgm_theme_madness.clone(),
            BGM::Boss => ost.bgm_boss.clone(),
            BGM::Pause => ost.bgm_pause.clone(),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum SFX {
    Hit,
    PackageMalus,
    PackageBonus,
    TowerAura,
    TowerBomb,
    TowerShot,
    PlaceTower,
    SellTower,
    UpgradeTower,
    GameOver,
    Pause,
}

impl SFX {
    fn handle(&self, ost: &Res<Ost>) -> Handle<AudioSource> {
        match self {
            SFX::Hit => {
                match thread_rng().next_u32() % 4 {
                    1 => ost.sfx_hit1.clone(),
                    2 => ost.sfx_hit2.clone(),
                    3 => ost.sfx_hit3.clone(),
                    _ => ost.sfx_hit4.clone(),
                }
            }
            SFX::PackageMalus => ost.sfx_package_malus.clone(),
            SFX::PackageBonus => ost.sfx_package_bonus.clone(),
            SFX::TowerAura => ost.sfx_tower_aura.clone(),
            SFX::TowerBomb => ost.sfx_bomb.clone(),
            SFX::TowerShot => ost.sfx_shot.clone(),
            SFX::PlaceTower => ost.sfx_place_tower.clone(),
            SFX::SellTower => ost.sfx_sell_tower.clone(),
            SFX::UpgradeTower => ost.sfx_upgrade_tower.clone(),
            SFX::GameOver => ost.sfx_game_over.clone(),
            SFX::Pause => ost.sfx_pause.clone(),
        }
    }
}

#[derive(Resource)]
pub struct BgmChannel;

#[derive(Resource)]
pub struct SfxChannel;

pub struct PlayBgmEvent(pub BGM);

pub struct PlaySfxEvent(pub SFX);

#[derive(Resource)]
struct CurrentBGM(BGM);

fn update(
    mut commands: Commands,
    mut bgm_events: EventReader<PlayBgmEvent>,
    mut sfx_events: EventReader<PlaySfxEvent>,
    ost: Option<Res<Ost>>,
    bgm_channel: Res<AudioChannel<BgmChannel>>,
    sfx_channel: Res<AudioChannel<SfxChannel>>,
    current: Option<Res<CurrentBGM>>,
) {
    let Some(ost) = ost else { return; };

    // Play BGMs
    for PlayBgmEvent(bgm) in bgm_events.iter() {
        if let Some(c) = current {
            if c.0 == *bgm { return; }
        }

        commands.insert_resource(CurrentBGM(*bgm));
        bgm_channel.stop();
        bgm_channel.set_volume(0.6);
        bgm_channel
            .play(bgm.handle(&ost))
            .looped();
        break;
    }
    bgm_events.clear();

    // Play SFXs
    for PlaySfxEvent(sfx) in sfx_events.iter() {
        sfx_channel.set_volume(0.3);
        sfx_channel.play(sfx.handle(&ost));
    }
}