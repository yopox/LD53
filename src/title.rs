use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::GameState;
use crate::graphics::loading::Fonts;
use crate::graphics::palette::Palette;
use crate::graphics::text;
use crate::graphics::transition::Transition;
use crate::util::size::{tile_to_f32, WIDTH};
use crate::util::z_pos;

pub struct TitlePlugin;

#[derive(Component)]
struct TitleUI;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_title.in_schedule(OnEnter(GameState::Title)))
            .add_system(exit_title.in_set(OnUpdate(GameState::Title)))
            .add_system(clean_title.in_schedule(OnExit(GameState::Title)))
        ;
    }
}

fn setup_title(
    mut commands: Commands,
    fonts: Res<Fonts>,
) {
    for (t, x, y, ts) in [
        ("Sabotage prime Inc", WIDTH / 4, 15, text::TextStyles::Heading),
        ("Press any key to continue", WIDTH / 4, 6, text::TextStyles::Body),
    ] {
        commands
            .spawn(text::ttf_anchor(tile_to_f32(x), tile_to_f32(y), z_pos::TITLE_TEXT, t, ts, &fonts, Palette::A, Anchor::BottomLeft))
            .insert(TitleUI);
    }
}

fn exit_title (
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    transition: Option<Res<Transition>>,
) {
    if transition.is_none() {
        for _ in keys.get_just_pressed() {
            commands.insert_resource(Transition::to(GameState::Battle))
        }
    }
}

fn clean_title (
    query: Query<Entity, With<TitleUI>>,
    mut commands: Commands,
) {
    for e in query.iter () {
        if let Some(entity_commands) = commands.get_entity(e) {
            entity_commands.despawn_recursive()
        }
    }
}

