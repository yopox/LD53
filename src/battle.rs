use bevy::prelude::*;

use crate::{collision, GameState};
use crate::drones::{despawn_drone, drones_dead, update_drones};
use crate::graphics::{MainBundle, package, sprite_from_tile};
use crate::graphics::animation::{Wiggle, wiggle};
use crate::graphics::grid::{GridElement, update_z};
use crate::graphics::loading::Textures;
use crate::graphics::package::collect_package;
use crate::graphics::palette::Palette;
use crate::logic::waves::{WaveIterator, WaveIteratorElement};
use crate::shot::{bomb_exploded, bomb_exploding, make_bomb_explode, remove_shots};
use crate::tower::{remove_slow_down, sell_tower, tower_fire, Towers, update_just_fired, upgrade_tower};
use crate::util::battle_z_from_y;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                setup.in_schedule(OnEnter(GameState::Battle))
            )
            .add_system(
                cleanup.in_schedule(OnExit(GameState::Battle))
            )
            .add_systems(
                (update_just_fired, remove_shots, tower_fire,
                 sell_tower, upgrade_tower,
                 update_drones, despawn_drone, drones_dead.after(collision::collide).after(wiggle))
                    .in_set(OnUpdate(GameState::Battle))
            )
            .add_systems(
                (bomb_exploding, make_bomb_explode, bomb_exploded, spawn_waves,
                 remove_slow_down, collect_package, reset_state, update_z)
                    .in_set(OnUpdate(GameState::Battle))
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
    /// Sell a tower
    Sell,
    /// Upgrade a tower
    Upgrade,
}

impl CursorState {
    pub fn get_color(&self) -> Palette {
        match self {
            CursorState::Select => Palette::B,
            CursorState::Build(_) => Palette::C,
            CursorState::Sell => Palette::K,
            CursorState::Upgrade => Palette::G,
        }
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.insert_resource(CursorState::Select);
    commands.insert_resource(Money(200));
    commands.insert_resource(WaveIterator::get_static())
}

fn reset_state(
    mouse: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    state: Option<ResMut<CursorState>>,
) {
    let Some(mut state) = state else { return; };
    // Return to [CursorState::Select]
    if mouse.just_pressed(MouseButton::Right) || keys.just_pressed(KeyCode::Escape) {
        state.set_if_neq(CursorState::Select);
    }
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

fn spawn_waves(
    mut wave_iterator: ResMut<WaveIterator>,
    mut commands: Commands,
    time: Res<Time>,
    textures: Res<Textures>,
) {
    wave_iterator.next.tick(time.delta());

    if wave_iterator.next.just_finished() {
        match wave_iterator.upcoming.pop() {
            Some(WaveIteratorElement::NextWave(t)) => {
                wave_iterator.next = Timer::from_seconds(t, TimerMode::Once);
            }
            Some(WaveIteratorElement::NextDrone(drone, t)) => {
                wave_iterator.next = Timer::from_seconds(t, TimerMode::Once);
                let atlas = &textures.tileset;
                commands
                    .spawn(drone.instantiate())
                    .insert(MainBundle::from_xyz(0., 0., battle_z_from_y(0.)))
                    .insert(Wiggle::with_frequency(Wiggle::slow()))
                    .with_children(|builder| {
                        sprite_from_tile(builder, drone.get_tiles(), atlas, 0.);
                        package::spawn(builder, drone.get_model().package_offset(), atlas);
                    })
                    .insert(BattleUI)
                    .insert(GridElement)
                ;
            }
            None => {}
        }
    }
}