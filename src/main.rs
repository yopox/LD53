use bevy::prelude::*;
use bevy_text_mode::TextModePlugin;

use graphics::palette::Palette;

use crate::battle::BattlePlugin;
use crate::collision::CollisionPlugin;
use crate::game_over::GameOverPlugin;
use crate::graphics::GraphicsPlugin;
use crate::level_select::LevelSelectPlugin;
use crate::music::MusicPlugin;
use crate::util::size;
use crate::util::size::tile_to_f32;

mod util;
mod graphics;
mod logic;
mod drones;
mod tower;
mod shot;
mod battle;
mod collision;
mod game_over;
mod music;
mod level_select;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Select,
    Battle,
    GameOver,
}

#[derive(Resource)]
pub struct Progress {
    pub level_unlocked: u8,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Palette::E.into()))
        .insert_resource(Msaa::Off)
        .insert_resource(Progress { level_unlocked: 1 })
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (
                        size::SCALE * tile_to_f32(size::WIDTH),
                        size::SCALE * tile_to_f32(size::HEIGHT),
                    ).into(),
                    title: "Sabotage, Inc.".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            })
        )
        .add_state::<GameState>()
        .add_plugin(TextModePlugin)
        .add_plugin(MusicPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(BattlePlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(LevelSelectPlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(1. / size::SCALE, 1. / size::SCALE, 1.),
            translation: Vec3::new(
                tile_to_f32(size::WIDTH) / 2.,
                tile_to_f32(size::HEIGHT) / 2.,
                100.),
            ..Default::default()
        },
        ..Default::default()
    });
}
