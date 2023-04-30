use bevy::prelude::*;
use strum_macros::EnumIter;

use crate::{graphics, util};
use crate::collision::{body_size, BodyType, Contact, HitBox};
use crate::graphics::sprites::{DroneModels, TILE};
use crate::shot::{Bomb, Shot, Shots, spawn_bomb};
use crate::util::size::f32_tile_to_f32;

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
                speed: 0.5,
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
                 body_type: BodyType::Enemy,
                 width: body_size.x,
                 height: body_size.y,
                 bottom_right_anchor: false,
             }
         })
    }

    pub fn get_model(&self) -> DroneModels {
        match self {
            Self::Drone => DroneModels::Super,
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

        let size = drone.class.get_model().get_size();
        pos.translation.x = f32_tile_to_f32(progress.x)
            - size.x / 2. + f32_tile_to_f32(0.5); // Center sprite
        pos.translation.y = f32_tile_to_f32(progress.y + util::size::GUI_HEIGHT as f32)
            + f32_tile_to_f32(0.5); // Make sprite levitate over the road
    }
}

pub fn drones_dead(
    mut event_reader: EventReader<Contact>,
    mut commands: Commands,
    mut enemies: Query<&mut Enemy>,
    shots: Query<(&Shot, &Transform)>,
) {
    for event in event_reader.iter() {
        match event {
            Contact((BodyType::Enemy, e_enemy), (BodyType::ShipShot, e_shot)) |
            Contact((BodyType::ShipShot, e_shot), (BodyType::Enemy, e_enemy))
            => {
                if let Ok((&shot, &t_shot)) = shots.get(*e_shot) {
                    if let Some(mut entity_commands) = commands.get_entity(*e_shot) {
                        entity_commands.despawn_recursive()
                    }

                    if let Ok(mut enemy) = enemies.get_mut(*e_enemy) {
                        if shot.class == Shots::Bomb {
                            spawn_bomb(Bomb::from_shot_translation(shot, t_shot.translation), &mut commands);
                        } else {
                            enemy.stats.hp -= shot.damages;
                            if enemy.stats.hp <= 0. {
                                enemy.stats.hp = 0.;

                                if let Some(entity_commands) = commands.get_entity(*e_enemy) {
                                    entity_commands.despawn_recursive();
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}