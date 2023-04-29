use std::time::Duration;

use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::graphics::sprites;
use crate::graphics::sprites::TILE;
use crate::shot::Shots;

#[derive(Component)]
pub struct Tower {
    class: Towers,
    /// Time between two shots in seconds
    reloading_delay: f32,
    shot: Shots,
}

#[derive(Debug, Copy, Clone)]
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
    mut towers: Query<(Entity, &Tower), Without<JustFired>>,
    mut enemies: Query<(Entity, &mut Enemy)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (e_tower, tower) in towers.iter_mut() {
        let mut chosen_enemy = None;
        let mut max_advance: f32 = -1.;
        for (e_enemy, enemy) in enemies.iter_mut() {
            if enemy.advance >= max_advance {
                chosen_enemy = Some((e_enemy, enemy));
                max_advance = enemy.advance;
            }
        }

        if let Some((e_enemy, mut enemy)) = chosen_enemy {
            enemy.stats.hp -= tower.shot.get_default_damages();
            if enemy.stats.hp <= 0. {
                enemy.stats.hp = 0.;
                if let Some(entity_commands) = commands.get_entity(e_enemy) {
                    entity_commands.despawn_recursive();
                }
            }

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
