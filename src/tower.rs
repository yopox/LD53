use std::time::Duration;

use bevy::ecs::system::EntityCommands;
use bevy::math::{vec3, Vec3Swizzles};
use bevy::prelude::*;
use bevy_tweening::{Animator, Tween};
use bevy_tweening::EaseMethod::Linear;
use bevy_tweening::lens::TransformPositionLens;
use strum_macros::EnumIter;

use crate::battle::BattleUI;
use crate::collision::body_size;
use crate::enemy::Enemy;
use crate::graphics::{gui, MainBundle, sprite_from_tile, sprites};
use crate::graphics::loading::Textures;
use crate::graphics::sprites::TILE;
use crate::shot::Shots;
use crate::util;
use crate::util::{with_z, z_pos};
use crate::util::misc::SLOW_DOWN_DELAY;
use crate::util::size::tile_to_f32;
use crate::util::tweening::SHOT_DESPAWN;

#[derive(Component, Clone)]
pub struct Tower {
    model: Towers,
    /// Time between two shots in seconds
    reloading_delay: f32,
    range: f32,
    radius: f32,
    shot: Option<Shots>,
    rank: u8,
}

impl Tower {
    pub fn upgrade_cost(&self) -> Option<u16> {
        match self.rank {
            1 => Some(2 * self.model.get_cost()),
            2 => Some(4 * self.model.get_cost()),
            _ => None,
        }
    }

    pub fn get_name(&self) -> &str {
        match self.model {
            Towers::Lightning => "Lightning Tower",
            Towers::PaintBomb => "Paint Bomb",
            Towers::Scrambler => "Scrambler",
        }
    }

    pub fn get_description(&self) -> String {
        match self.upgrade_cost() {
            Some(n) => format!("Rank {} (up: €{})", self.rank, n),
            None => format!("Rank {} (rank max)", self.rank),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq)]
pub enum Towers {
    Lightning,
    PaintBomb,
    Scrambler,
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
                model: *self,
                reloading_delay: 10.,
                range: tile_to_f32(5),
                radius: tile_to_f32(5),
                shot: Some(Shots::Basic),
                rank: 1,
            },
            Towers::PaintBomb => Tower {
                model: *self,
                reloading_delay: 15.,
                range: tile_to_f32(8),
                radius: tile_to_f32(9),
                shot: Some(Shots::Bomb),
                rank: 1,
            },
            Towers::Scrambler => Tower {
                model: *self,
                reloading_delay: 2.,
                range: tile_to_f32(3),
                radius: tile_to_f32(4),
                shot: None,
                rank: 2,
            }
        }
    }

    pub const fn get_tiles(&self) -> &[TILE] {
        match &self {
            Towers::Lightning => &sprites::TOWER_1,
            Towers::Scrambler => &sprites::TOWER_2,
            Towers::PaintBomb => &sprites::TOWER_3,
        }
    }

    /// Returns the delay on tower construction
    pub const fn initial_delay(&self) -> f32 {
        5.
    }

    pub const fn get_cost(&self) -> u16 {
        match self {
            Towers::Lightning => 40,
            Towers::PaintBomb => 60,
            Towers::Scrambler => 50,
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
    let tower = tower.instantiate();
    let size = body_size(tower.model.get_tiles());
    commands
        .spawn(tower.clone())
        .insert(
            MainBundle::from_xyz(tile_to_f32(2 * x), tile_to_f32(2 * y + util::size::GUI_HEIGHT), z_pos::TOWERS)
        )
        .with_children(|builder|
            sprite_from_tile(builder, tower.model.get_tiles(), atlas, 0.)
        )
        .insert(JustFired::new(time, tower.model.initial_delay()))
        .insert(gui::HoverPopup::new(tower.get_name(), &tower.get_description(), Some(("Damage", 1)), Some(("Speed", 4)), size.x, size.y))
        .insert(BattleUI)
    ;
}

pub fn tower_fire(
    towers: Query<(Entity, &Transform, &Tower), Without<JustFired>>,
    enemies: Query<(Entity, &Transform, &Enemy)>,
    mut commands: Commands,
    time: Res<Time>,
    textures: Res<Textures>,
) {
    for (e_tower, &t_tower, tower) in towers.iter() {
        match tower.model {
            Towers::Lightning | Towers::PaintBomb => {
                let chosen_enemy = enemies.iter()
                    .filter(|(_, t, _)| t.translation.xy().distance(t_tower.translation.xy()) <= tower.range)
                    .max_by_key(|(_, _, enemy)| (enemy.advance * 4096.) as usize);

                if let Some((_, t_enemy, _)) = chosen_enemy {
                    shoot(&mut commands, &textures, t_tower, tower, t_enemy.translation);
                }
            }
            Towers::Scrambler => {
                enemies.iter()
                    .filter(|(_, t, _)| t.translation.xy().distance(t_tower.translation.xy()) <= tower.range)
                    .for_each(|(e, _, _)| {
                        if let Some(mut entity_commands) = commands.get_entity(e) {
                            entity_commands.insert(Slow {
                                index: SlowIndex::Level0,
                                t_final: time.elapsed() + Duration::from_secs_f32(SLOW_DOWN_DELAY),
                            });
                        }
                    })
                ;
            }
        }

        if let Some(mut entity_commands) = commands.get_entity(e_tower) {
            entity_commands.insert(JustFired::new(&time, tower.reloading_delay));
        }
    }
}

fn shoot(commands: &mut Commands, textures: &Res<Textures>, t_tower: Transform, tower: &Tower, enemy_position: Vec3) {
    let distance = t_tower.translation.distance(enemy_position);
    let shot_translation = vec3(t_tower.translation.x, t_tower.translation.y + 1., z_pos::SHOT);
    let shot = tower.shot.expect("For shooting something.");
    commands
        .spawn(shot.instantiate())
        .insert(MainBundle::from_translation(shot_translation))
        .insert(Animator::new(Tween::new(
            Linear,
            Duration::from_secs_f32(tower.radius / shot.get_speed()),
            TransformPositionLens {
                start: shot_translation,
                end: with_z(
                    shot_translation + (enemy_position - shot_translation) * tower.radius / distance,
                    z_pos::SHOT),
            },
        ).with_completed_event(SHOT_DESPAWN)))
        .with_children(|builder|
            sprite_from_tile(builder, &[shot.get_tile()], &textures.tileset, 0.)
        )
        .insert(BattleUI)
    ;
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

#[derive(Debug, Copy, Clone)]
pub enum SlowIndex {
    Level0 = 0,
    Level1,
    Level2,
}

impl SlowIndex {
    pub fn to_f32(&self) -> f32 {
        match &self {
            SlowIndex::Level0 => 0.66,
            SlowIndex::Level1 => 0.5,
            SlowIndex::Level2 => 0.33,
        }
    }
}

#[derive(Component)]
pub struct Slow {
    index: SlowIndex,
    t_final: Duration,
}

impl Slow {
    pub fn to_f32(&self) -> f32 {
        self.index.to_f32()
    }
}

pub fn remove_slow_down(
    slown_down: Query<(Entity, &Slow)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (e, &Slow { t_final, .. }) in slown_down.iter() {
        if t_final <= time.elapsed() {
            if let Some(mut entity_commands) = commands.get_entity(e) {
                entity_commands.remove::<Slow>();
            }
        }
    }
}