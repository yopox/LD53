use std::time::Duration;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_tweening::{Animator, Tween};
use bevy_tweening::EaseMethod::Linear;
use bevy_tweening::lens::TransformPositionLens;
use strum_macros::EnumIter;

use crate::enemy::{Enemies, Enemy};
use crate::graphics::{MainBundle, sprite_from_tile, sprites};
use crate::graphics::loading::Textures;
use crate::graphics::sprites::TILE;
use crate::playing::PlayingUI;
use crate::shot::Shots;
use crate::util::{with_z, z_pos};
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

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum Towers {
    Basic,
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
            Towers::Basic => Tower {
                class: *self,
                reloading_delay: 10.,
                range: 120.,
                radius: 120.,
                shot: Shots::Basic,
            }
        }
    }

    pub const fn get_tiles(&self) -> &[TILE] {
        match &self {
            Towers::Basic => &sprites::TOWER_1,
        }
    }
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
            commands
                .spawn(tower.shot.instantiate())
                .insert(MainBundle::from_xyz(t_tower.translation.x, t_tower.translation.y, z_pos::SHOT))
                .insert(Animator::new(Tween::new(
                    Linear,
                    Duration::from_secs_f32(distance / tower.shot.get_speed()),
                    TransformPositionLens {
                        start: with_z(t_tower.translation, z_pos::SHOT),
                        end: with_z(enemy_position, z_pos::SHOT),
                    },
                ).with_completed_event(SHOT_DESPAWNED)))
                .with_children(|builder|
                    sprite_from_tile(builder, Enemies::Drone.get_tiles(), &textures.mrmotext, 0.)
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
