use bevy::prelude::*;

use crate::{GameState, util};
use crate::enemy::{drones_dead, Enemies, update_drones};
use crate::graphics::{MainBundle, package, sprite_from_tile};
use crate::graphics::animation::Wiggle;
use crate::graphics::loading::Textures;
use crate::shot::remove_shots;
use crate::tower::{tower_fire, Towers, update_just_fired};
use crate::util::size::{f32_tile_to_f32, tile_to_f32};

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
                (update_just_fired, tower_fire, update_drones, remove_shots, drones_dead)
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
    let atlas = &textures.tileset;

    commands.spawn(Enemies::Drone.instantiate())
        .insert(
            MainBundle::from_xyz(0., 0., util::z_pos::ENEMIES)
        )
        .insert(Wiggle::with_frequency(Wiggle::slow()))
        .with_children(|builder| {
            sprite_from_tile(builder, Enemies::Drone.get_tiles(), atlas, 0.);
            package::spawn(builder, Enemies::Drone.get_model().package_offset(), atlas);
        })
        .insert(PlayingUI);

    commands.spawn(Towers::Basic.instantiate())
        .insert(
            MainBundle::from_xyz(tile_to_f32(6), f32_tile_to_f32(8.), util::z_pos::ENEMIES)
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