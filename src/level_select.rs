use bevy::math::{vec2, Vec3Swizzles};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{GameState, Progress, util};
use crate::graphics::{grid, sprite};
use crate::graphics::grid::GridUI;
use crate::graphics::loading::{Fonts, Textures};
use crate::graphics::palette::Palette;
use crate::graphics::text::{TextStyles, ttf_anchor};
use crate::graphics::transition::Transition;
use crate::music::{BGM, PlayBgmEvent};
use crate::util::is_in;
use crate::util::size::tile_to_f32;

pub struct LevelSelectPlugin;

impl Plugin for LevelSelectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(GameState::Select)))
            .add_system(update.in_set(OnUpdate(GameState::Select)))
            .add_system(clean.in_schedule(OnExit(GameState::Select)))
        ;
    }
}

#[derive(Resource)]
pub struct CurrentLevel(pub u8);

#[derive(Component)]
struct SelectUI;

#[derive(Component)]
struct MainText;

#[derive(Component)]
struct LevelButton(pub u8);

fn setup(
    mut commands: Commands,
    mut bgm: EventWriter<PlayBgmEvent>,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
    progress: Res<Progress>,
) {
    bgm.send(PlayBgmEvent(BGM::Title));

    let path = vec![
        vec2(3., 3.),
        vec2(16., 3.),
        vec2(16., 4.),
        vec2(3., 4.),
        vec2(5., 4.),
        vec2(5., 5.),
        vec2(14., 5.),
    ];
    grid::draw_road(&mut commands, &textures, path);

    commands
        .spawn(ttf_anchor(
            tile_to_f32(util::size::WIDTH / 2),
            tile_to_f32(util::size::GUI_HEIGHT / 2),
            util::z_pos::GUI_BG,
            "Select a level",
            TextStyles::Heading, &fonts, Palette::B,
            Anchor::Center,
        ))
        .insert(MainText)
        .insert(SelectUI);

    for (x, y, index, level) in [
        (8, 7, 422, 1),
        (9, 7, 423, 2),
        (10, 7, 423, 2),
        (11, 7, 423, 2),
        (11, 8, 423, 2),
        (11, 9, 423, 2),
        (12, 9, 423, 2),
        (13, 9, 422, 2),
        (14, 9, 423, 3),
        (15, 9, 423, 3),
        (16, 9, 423, 3),
        (17, 9, 423, 3),
        (18, 9, 423, 3),
        (19, 9, 422, 3),
        (20, 9, 423, 4),
        (21, 9, 423, 4),
        (22, 9, 423, 4),
        (23, 9, 423, 4),
        (24, 9, 423, 4),
        (25, 9, 423, 4),
        (26, 9, 422, 4),
        (27, 9, 423, 5),
        (28, 9, 423, 5),
        (28, 8, 423, 5),
        (28, 7, 423, 5),
        (29, 7, 423, 5),
        (30, 7, 423, 5),
        (31, 7, 422, 5),
        (22, 8, 423, 4),
        (22, 7, 423, 4),
        (22, 6, 423, 4),
        (22, 5, 423, 4),
        (22, 4, 423, 4),
        (22, 3, 423, 4),
        (21, 3, 423, 4),
        (20, 3, 423, 4),
        (19, 3, 423, 4),
        (18, 3, 423, 4),
        (17, 3, 423, 4),
        (16, 3, 422, 6),
    ] {
        let unlocked = level <= progress.level_unlocked || level == 6 && progress.level_unlocked >= 3;
        let fg = if unlocked { Palette::G } else { Palette::M };
        let bg = if y > 5 { Palette::E } else { Palette::Transparent };
        let sprite = sprite(
            index, x, y + util::size::GUI_HEIGHT, util::z_pos::GUI_BG,
            bg, fg,
            false, 0, textures.tileset.clone(),
        );

        if index == 422 {
            commands
                .spawn(sprite)
                .insert(SelectUI)
                .insert(LevelButton(level));
        } else {
            commands
                .spawn(sprite)
                .insert(SelectUI);
        }
    }
}

fn update(
    mut commands: Commands,
    windows: Query<&Window>,
    buttons: Query<(&Transform, &LevelButton)>,
    mut text: Query<&mut Text, With<MainText>>,
    mouse: Res<Input<MouseButton>>,
    progress: Res<Progress>,
    transition: Option<Res<Transition>>,
) {
    if transition.is_some() { return; }
    let Some(cursor_pos) = util::cursor_pos(windows) else { return; };

    for (pos, level) in &buttons {
        if !is_in(cursor_pos, pos.translation.xy(), vec2(tile_to_f32(1), tile_to_f32(1))) { continue; }

        let mut text = text.single_mut();
        text.sections[0].value = match level.0 {
            6 => "Endless".to_string(),
            i => format!("Level {}", i),
        };

        let unlocked = level.0 <= progress.level_unlocked || level.0 == 6 && progress.level_unlocked >= 3;
        if mouse.just_pressed(MouseButton::Left) && unlocked {
            commands.insert_resource(CurrentLevel(level.0));
            commands.insert_resource(Transition::to(GameState::Battle));
        }
    }
}

fn clean(
    mut commands: Commands,
    q1: Query<Entity, With<SelectUI>>,
    q2: Query<Entity, With<GridUI>>,
) {
    for id in &q1 { commands.entity(id).despawn_recursive(); }
    for id in &q2 { commands.entity(id).despawn_recursive(); }
}