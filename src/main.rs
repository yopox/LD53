use crate::util::size;
use crate::util::size::tile_to_f32;
use bevy::prelude::*;
use bevy_text_mode::TextModePlugin;
use graphics::palette::Palette;
use crate::graphics::GraphicsPlugin;
use crate::graphics::loading::LoadingPlugin;
use crate::graphics::text::TextPlugin;
use crate::playing::PlayingPlugin;
use crate::title::TitlePlugin;

mod util;
mod title;
mod graphics;
mod logic;
mod enemy;
mod tower;
mod shot;
mod playing;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    Main,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Palette::Black.into()))
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (
                        size::SCALE * tile_to_f32(size::WIDTH),
                        size::SCALE * tile_to_f32(size::HEIGHT),
                    ).into(),
                    title: "bevy_template".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            })
        )
        .add_state::<GameState>()
        .add_plugin(TextModePlugin)
        .add_plugin(TitlePlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(PlayingPlugin)
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
