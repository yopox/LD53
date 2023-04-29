use bevy::prelude::*;

use crate::{graphics, util};
use crate::graphics::sprites;
use crate::graphics::sprites::TILE;

#[derive(Debug, Clone)]
pub struct EnemyStats {
    pub(crate) hp: f32,
    speed: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Enemies {
    Drone
}

#[derive(Component)]
pub struct Enemy {
    class: Enemies,
    pub(crate) stats: EnemyStats,
    pub(crate) advance: f32,
}

impl Enemies {
    pub const fn get_default_stats(&self) -> EnemyStats {
        match self {
            Self::Drone => EnemyStats {
                hp: 10.,
                speed: 0.02,
            }
        }
    }

    pub fn instantiate(&self) -> Enemy {
        Enemy {
            class: *self,
            stats: self.get_default_stats().clone(),
            advance: 0.,
        }
    }

    pub const fn get_tiles(&self) -> &[TILE] {
        match self {
            Self::Drone => &sprites::DRONE_1,
        }
    }
}

pub fn update_drones(
    mut drones: Query<(&mut Transform, &mut Enemy)>,
    path: Option<Res<graphics::grid::CurrentPath>>,
) {
    let Some(path) = path else { return; };
    for (mut pos, mut drone) in drones.iter_mut() {
        drone.advance += drone.stats.speed;

        let Some(progress) = path.0.pos(drone.advance) else { continue };
        pos.translation.x = util::size::path_to_f32(progress.x);
        pos.translation.y = util::size::path_to_f32(progress.y);
    }
}