use bevy::prelude::*;
use strum_macros::EnumIter;

use crate::{graphics, util};
use crate::collision::{body_size, HitBox};
use crate::graphics::sprites::{DroneModels, TILE};

#[derive(Debug, Clone)]
pub struct EnemyStats {
    pub(crate) hp: f32,
    speed: f32,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Enemies {
    Drone
}

#[derive(Component)]
pub struct Enemy {
    pub class: Enemies,
    pub stats: EnemyStats,
    pub advance: f32,
}

impl Enemies {
    pub const fn get_default_stats(&self) -> EnemyStats {
        match self {
            Self::Drone => EnemyStats {
                hp: 10.,
                speed: 2.,
            }
        }
    }

    pub fn instantiate(&self) -> (Enemy, HitBox) {
        (Enemy {
            class: *self,
            stats: self.get_default_stats().clone(),
            advance: 0.,
        }, {
             let body_size = body_size(self.get_tiles());
             HitBox {
                 dx: 0.,
                 dy: 0.,
                 width: body_size.x,
                 height: body_size.y,
             }
         })
    }

    pub fn get_model(&self) -> DroneModels {
        match self {
            Self::Drone => DroneModels::Simple,
        }
    }

    pub fn get_tiles(&self) -> &'static [TILE] {
        match self {
            Self::Drone => self.get_model().get_tiles(),
        }
    }
}

pub fn update_drones(
    mut drones: Query<(&mut Transform, &mut Enemy)>,
    path: Option<Res<graphics::grid::CurrentPath>>,
    time: Res<Time>,
) {
    let Some(path) = path else { return; };
    for (mut pos, mut drone) in drones.iter_mut() {
        drone.advance += drone.stats.speed * time.delta_seconds();

        let Some(progress) = path.0.pos(drone.advance) else { continue };
        pos.translation.x = util::size::path_to_f32(progress.x);
        pos.translation.y = util::size::path_to_f32(progress.y);
    }
}