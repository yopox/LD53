use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::battle::DronesStats;
use crate::GameState;
use crate::graphics::loading::Fonts;
use crate::graphics::palette::Palette;
use crate::graphics::text;
use crate::util::size::{tile_to_f32, WIDTH};
use crate::util::z_pos;

pub struct GameOverPlugin;

#[derive(Component)]
pub struct GameOverUI;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                setup.in_schedule(OnEnter(GameState::GameOver))
            )
            .add_system(
                cleanup.in_schedule(OnExit(GameState::GameOver))
            )
        ;
    }
}

fn setup(
    stats: ResMut<DronesStats>,
    mut commands: Commands,
    fonts: Res<Fonts>,
) {
    let mut texts: Vec<(String, usize)> = vec![
        ("You've seen all drones!".into(), 16),
    ];

    if stats.survived == 0 {
        texts.push((format!("You've taken down all {} of them!", stats.killed), 11));
        texts.push((format!("Nice job! See you at next level..."), 6));
    } else if stats.killed == 0 {
        texts.push(("You've not taken down a single drone.".to_string(), 11));
        texts.push(("Do I need to teach you how to build a tower?".to_string(), 6));
    } else {
        texts.push((format!("You've taken down {} of them,", stats.killed), 11));
        texts.push((format!("but {} of them survived.", stats.killed), 8));
        if stats.survived > 5 {
            texts.push((format!("Try harder next time..."), 5));
        } else {
            texts.push((format!("Nice job! Can you survive next level?"), 5));
        }
    }

    for (t, y) in texts {
        commands
            .spawn(text::ttf_anchor(tile_to_f32(WIDTH / 2), tile_to_f32(y), z_pos::TITLE_TEXT, &t, text::TextStyles::Heading, &fonts, Palette::A, Anchor::BottomCenter))
            .insert(GameOverUI);
    }
}

fn cleanup(
    query: Query<Entity, With<GameOverUI>>,
    mut commands: Commands,
) {
    for e in &query {
        if let Some(entity_commands) = commands.get_entity(e) {
            entity_commands.despawn_recursive();
        }
    }
}