use bevy::prelude::*;

use crate::GameState;
use crate::graphics::palette::Palette;
use crate::graphics::text::color_text;
use crate::graphics::transition::Transition;
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

fn setup_title (
    mut commands: Commands,
) {
    for (t, x, y) in [
        ("Bevy game template", 11, 15),
        ("Change this for more fun.", 5, 12),
        ("Press any key to continue", 20, 6),
    ] {
        commands
            .spawn(color_text(t, x, y, z_pos::TITLE_TEXT, Palette::A, Palette::E))
            .insert(TitleUI);
    }
}

fn exit_title (
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    for _ in keys.get_just_released() {
        commands.insert_resource(Transition::to(GameState::Battle))
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

