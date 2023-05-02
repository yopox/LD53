use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;
use bevy::sprite::Anchor;

use crate::{GameState, Progress};
use crate::battle::DronesStats;
use crate::graphics::loading::Fonts;
use crate::graphics::palette::Palette;
use crate::graphics::text;
use crate::graphics::transition::Transition;
use crate::level_select::CurrentLevel;
use crate::music::{BGM, PlayBgmEvent};
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
            .add_system(
                exit_game_over.in_set(OnUpdate(GameState::GameOver))
            )
        ;
    }
}

fn setup(
    mut commands: Commands,
    mut bgm: EventWriter<PlayBgmEvent>,
    mut progress: ResMut<Progress>,
    stats: ResMut<DronesStats>,
    current_level: Res<CurrentLevel>,
    fonts: Res<Fonts>,
) {
    bgm.send(PlayBgmEvent(BGM::Pause));

    let mut texts: Vec<(String, usize)> = vec![
        ("You've seen all drones!".into(), 16),
    ];

    let mut won = true;

    if stats.survived == 0 {
        texts.push((format!("You've taken down all {} of them!", stats.killed), 11));
    } else if stats.killed == 0 {
        won = false;
        texts.push(("You've not taken down a single drone.".to_string(), 11));
        texts.push(("Do I need to teach you how to build a tower?".to_string(), 6));
    } else {
        texts.push((format!("You've taken down {} of them,", stats.killed), 11));
        texts.push((format!("but {} of them survived.", stats.survived), 8));
        if stats.survived > 5 {
            won = false;
            texts.push((format!("Try harder next time..."), 5));
        }
    }

    if won {
        if current_level.0 == 6 {
            texts.push(("You are the new boss of Sabotage, Inc.!".to_string(), 5));
        } else if current_level.0 == 5 {
            texts.push(("Nice job! Thank you for playing our game.".to_string(), 5));
            texts.push(("Can you survive the endless mode?".to_string(), 2));
        } else {
            texts.push(("Nice job! Can you survive the next level?".to_string(), 5));
        }
        if progress.level_unlocked <= current_level.0 { progress.level_unlocked += 1; }
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

fn exit_game_over(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    transition: Option<Res<Transition>>,
) {
    if transition.is_some() { return; }
    for _ in keys.get_just_pressed() {
        commands.insert_resource(Transition::to(GameState::Select));
        return;
    }
    if mouse.just_pressed(MouseButton::Left) {
        commands.insert_resource(Transition::to(GameState::Select));
    }
}
