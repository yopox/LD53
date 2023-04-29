use bevy::prelude::*;

use crate::{GameState, util};
use crate::enemy::Enemies;
use crate::graphics::{MainBundle, sprite_from_tile};
use crate::graphics::loading::Textures;
use crate::tower::{tower_fire, Towers, update_just_fired};

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
            .add_systems(
                (update_just_fired, tower_fire)
                    .in_set(OnUpdate(GameState::Main))
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
    let atlas = &textures.mrmotext;

    commands.spawn(Enemies::Drone.instantiate())
        .insert(
            MainBundle::from_xyz(200., 48., util::z_pos::ENEMIES)
        )
        .with_children(|builder|
            sprite_from_tile(builder, Enemies::Drone.get_tiles(), atlas, 0.))
        .insert(PlayingUI);

    commands.spawn(Enemies::Drone.instantiate())
        .insert(
            MainBundle::from_xyz(28., 48., util::z_pos::ENEMIES)
        )
        .with_children(|builder|
            sprite_from_tile(builder, Enemies::Drone.get_tiles(), atlas, 0.))
        .insert(PlayingUI);

    commands.spawn(Towers::Basic.instantiate())
        .insert(
            MainBundle::from_xyz(100., 80., util::z_pos::ENEMIES)
        )
        .with_children(|builder|
            sprite_from_tile(builder, Towers::Basic.get_tiles(), atlas, 0.))
        .insert(PlayingUI);
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