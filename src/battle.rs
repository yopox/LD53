use bevy::prelude::*;

use crate::{GameState, util};
use crate::enemy::{despawn_drone, drones_dead, Enemies, update_drones};
use crate::graphics::{MainBundle, package, sprite_from_tile};
use crate::graphics::animation::{Wiggle, wiggle};
use crate::graphics::loading::Textures;
use crate::shot::{bomb_exploded, bomb_exploding, make_bomb_explode, remove_shots};
use crate::tower::{tower_fire, Towers, update_just_fired};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                setup.in_schedule(OnEnter(GameState::Main))
            )
            .add_system(
                cleanup.in_schedule(OnExit(GameState::Main))
            )
            .add_systems(
                (update_just_fired, tower_fire, update_drones, remove_shots,
                 bomb_exploding, make_bomb_explode, bomb_exploded, despawn_drone,
                 drones_dead.after(wiggle))
                    .in_set(OnUpdate(GameState::Main))
            )
        ;
    }
}

#[derive(Component)]
pub struct BattleUI;

#[derive(Resource)]
pub struct Money(pub u16);

#[derive(Resource, PartialEq)]
pub enum CursorState {
    /// Default state
    Select,
    /// Place a tower
    Build(Towers),
}

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    commands.insert_resource(CursorState::Select);
    commands.insert_resource(Money(100));

    let atlas = &textures.tileset;

    // TODO: Move drones spawn logic out of playing
    commands.spawn(Enemies::Drone.instantiate())
        .insert(
            MainBundle::from_xyz(0., 0., util::z_pos::ENEMIES)
        )
        .insert(Wiggle::with_frequency(Wiggle::slow()))
        .with_children(|builder| {
            sprite_from_tile(builder, Enemies::Drone.get_tiles(), atlas, 0.);
            package::spawn(builder, Enemies::Drone.get_model().package_offset(), atlas);
        })
        .insert(BattleUI);
}

fn cleanup(
    query: Query<Entity, With<BattleUI>>,
    mut commands: Commands,
) {
    for e in &query {
        if let Some(entity_commands) = commands.get_entity(e) {
            entity_commands.despawn_recursive();
        }
    }
}