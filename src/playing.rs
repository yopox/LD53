use bevy::prelude::*;
use crate::enemy::{Enemies, spawn_enemy};
use crate::GameState;
use crate::graphics::loading::Textures;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
            setup_playing.in_schedule(OnEnter(GameState::Main))
            )
            .add_system(
                exit_playing.in_schedule(OnExit(GameState::Main))
            )
        ;
    }
}

#[derive(Component)]
pub struct PlayingUI;

fn setup_playing (
    mut commands: Commands,
    textures: Res<Textures>,
) {
    spawn_enemy(Enemies::Drone, Vec2::new(200., 48.), &textures.mrmotext, commands);
}

fn exit_playing (
    query: Query<Entity, With<PlayingUI>>,
    mut commands: Commands,
) {
    for e in &query {
        if let Some(entity_commands) = commands.get_entity(e) {
            entity_commands.despawn_recursive();
        }
    }
}