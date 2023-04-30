use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Tween};
use bevy_tweening::EaseMethod::Linear;
use bevy_tweening::lens::TransformPositionLens;
use strum_macros::EnumIter;

use crate::battle::PlayingUI;
use crate::enemy::Enemy;
use crate::graphics::{gui, MainBundle, sprite_from_tile, sprites};
use crate::graphics::loading::Textures;
use crate::graphics::sprites::TILE;
use crate::shot::Shots;
use crate::util;
use crate::util::{with_z, z_pos};
use crate::util::size::tile_to_f32;
use crate::util::tweening::SHOT_DESPAWNED;

#[derive(Component)]
pub struct Tower {
    class: Towers,
    /// Time between two shots in seconds
    reloading_delay: f32,
    range: f32,
    radius: f32,
    shot: Shots,
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq)]
pub enum Towers {
    Lightning,
}

#[derive(Component)]
pub struct JustFired {
    /// time of firing, as an interval from startup time
    t0: Duration,
    /// Reloading time, as an interval from t0
    delay: Duration,
}

impl JustFired {
    pub fn new(time: &Time, delay: f32) -> Self {
        Self { t0: time.elapsed(), delay: Duration::from_secs_f32(delay) }
    }
}

impl Towers {
    pub const fn instantiate(&self) -> Tower {
        match &self {
            Towers::Lightning => Tower {
                class: *self,
                reloading_delay: 10.,
                range: tile_to_f32(5),
                radius: tile_to_f32(5),
                shot: Shots::Basic,
            }
        }
    }

    pub const fn get_tiles(&self) -> &[TILE] {
        match &self {
            Towers::Lightning => &sprites::TOWER_1,
        }
    }

    /// Returns the delay on tower construction
    pub const fn initial_delay(&self) -> f32 {
        5.
    }

    pub const fn get_cost(&self) -> u16 {
        match self {
            Towers::Lightning => 40,
        }
    }
}

/// Place a tower on (x, y) in grid coordinates.
pub fn place_tower(
    x: usize, y: usize,
    commands: &mut Commands,
    tower: Towers, atlas: &Handle<TextureAtlas>,
    time: &Time,
) {
    commands.spawn(tower.instantiate())
        .insert(
            MainBundle::from_xyz(tile_to_f32(x), tile_to_f32(y + util::size::GUI_HEIGHT), z_pos::TOWERS)
        )
        .with_children(|builder|
            sprite_from_tile(builder, tower.get_tiles(), atlas, 0.)
        )
        .insert(JustFired::new(time, tower.initial_delay()))
        .insert(gui::HoverPopup::new("Lightning tower", "Tier 1 (up: €80)", Some(("Damage", 1)), Some(("Speed", 4)), 8., 16.))
        .insert(PlayingUI)
    ;
}

pub fn tower_fire(
    towers: Query<(Entity, &Transform, &Tower), Without<JustFired>>,
    mut enemies: ParamSet<(
        Query<(Entity, &Transform, &Enemy)>,
        Query<&mut Enemy>,
    )>,
    mut commands: Commands,
    time: Res<Time>,
    textures: Res<Textures>,
) {
    for (e_tower, &t_tower, tower) in towers.iter() {
        let mut chosen_enemy: Option<(Vec3, f32)> = None;
        let mut max_advance: f32 = -1.;
        for (_e_enemy, t_enemy, enemy) in enemies.p0().iter() {
            let advance = enemy.advance;
            let distance = t_tower.translation.distance(t_enemy.translation);
            if advance >= max_advance && distance <= tower.range {
                chosen_enemy = Some((t_enemy.translation, distance));
                max_advance = advance;
            }
        }

        if let Some((enemy_position, distance)) = chosen_enemy {
            let mut shot_translation = t_tower.translation.clone();
            shot_translation.y += tile_to_f32(1);
            commands
                .spawn(tower.shot.instantiate())
                .insert(MainBundle::from_xyz(shot_translation.x, shot_translation.y, z_pos::SHOT))
                .insert(Animator::new(Tween::new(
                    Linear,
                    Duration::from_secs_f32(tower.radius / tower.shot.get_speed()),
                    TransformPositionLens {
                        start: with_z(shot_translation, z_pos::SHOT),
                        end: with_z(
                            shot_translation + (enemy_position - shot_translation) * tower.radius / distance,
                            z_pos::SHOT),
                    },
                ).with_completed_event(SHOT_DESPAWNED)))
                .with_children(|builder|
                    sprite_from_tile(builder, &[tower.shot.get_tile()], &textures.tileset, 0.)
                )
                .insert(PlayingUI)
            ;

            if let Some(mut entity_commands) = commands.get_entity(e_tower) {
                entity_commands.insert(JustFired::new(&time, tower.reloading_delay));
            }
        }
    }
}

pub fn update_just_fired(
    query: Query<(Entity, &JustFired)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (e, just_fired) in query.iter() {
        if just_fired.t0 + just_fired.delay <= time.elapsed() {
            if let Some(mut entity_commands) = commands.get_entity(e) {
                entity_commands.remove::<JustFired>();
            }
        }
    }
}
