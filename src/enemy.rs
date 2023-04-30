use std::time::Duration;

use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;
use bevy_tweening::{Animator, Delay, EaseFunction, Tween, TweenCompleted};
use bevy_tweening::lens::TransformPositionLens;
use strum_macros::EnumIter;

use crate::{graphics, util};
use crate::collision::{body_size, BodyType, Contact, HitBox};
use crate::graphics::{sprite_f32, tween};
use crate::graphics::animation::Wiggle;
use crate::graphics::grid::CurrentPath;
use crate::graphics::loading::Textures;
use crate::graphics::package::{ClickablePackage, Package};
use crate::graphics::sprites::{DroneModels, TILE};
use crate::shot::{Bomb, Shot, Shots, spawn_bomb};
use crate::util::size::{f32_tile_to_f32, tile_to_f32};

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

        let Some(progress) = path.0.pos(drone.advance) else { continue; };

        let size = drone.class.get_model().get_size();
        pos.translation.x = f32_tile_to_f32(progress.x * 2.)
            - size.x / 2. + f32_tile_to_f32(1.); // Center sprite
        pos.translation.y = f32_tile_to_f32(progress.y * 2. + util::size::GUI_HEIGHT as f32)
            + f32_tile_to_f32(1.5); // Make sprite levitate over the road
    }
}

pub fn drones_dead(
    mut event_reader: EventReader<Contact>,
    mut commands: Commands,
    mut enemies: Query<(&mut Enemy, &Transform)>,
    children: Query<&Children>,
    shots: Query<(&Shot, &Transform)>,
    package: Query<&Package>,
    path: Res<CurrentPath>,
    textures: Res<Textures>,
) {
    for event in event_reader.iter() {
        match event {
            Contact((BodyType::Enemy, e_enemy), (BodyType::ShipShot, e_shot)) |
            Contact((BodyType::ShipShot, e_shot), (BodyType::Enemy, e_enemy))
            => {
                let Ok((&shot, &t_shot)) = shots.get(*e_shot) else { continue };

                // Despawn shot
                if let Some(mut entity_commands) = commands.get_entity(*e_shot) {
                    entity_commands.despawn_recursive()
                }

                let Ok((mut enemy, e_pos)) = enemies.get_mut(*e_enemy) else { continue };
                match shot.class {
                    Shots::Bomb => {
                        spawn_bomb(Bomb::from_shot_translation(shot, t_shot.translation), &mut commands);
                    }
                    Shots::Basic => {
                        enemy.stats.hp -= shot.damages;
                        if enemy.stats.hp > 0. { continue }
                        enemy.stats.hp = 0.;

                        // Enemy death animation
                        kill_drone(&mut commands, &children, enemy.as_ref(), e_enemy, e_pos, &package, &path, &textures);
                    }
                }
            }
            _ => {}
        }
    }
}

fn kill_drone(
    mut commands: &mut Commands,
    children: &Query<&Children>,
    enemy: &Enemy,
    e_enemy: &Entity,
    e_pos: &Transform,
    package: &Query<&Package>,
    path: &Res<CurrentPath>,
    textures: &Res<Textures>,
) {
    let (x, y) = (e_pos.translation.x, e_pos.translation.y);

    if let Some(mut entity_commands) = commands.get_entity(*e_enemy) {
        entity_commands
            .remove::<HitBox>()
            .remove::<Enemy>()
            .remove::<Wiggle>()
            .insert(Animator::new(Delay::<Transform>::new(Duration::from_millis(util::tweening::DRONE_DEATH_FREEZE)).then(Tween::new(
                EaseFunction::CubicOut,
                Duration::from_millis(util::tweening::DRONE_DEATH_POS),
                TransformPositionLens {
                    start: e_pos.translation,
                    end: Vec3::new(x, y + tile_to_f32(1), e_pos.translation.z),
                },
            ).with_completed_event(util::tweening::DRONE_DESPAWN)
            )));
    }

    children
        .iter_descendants(*e_enemy)
        .for_each(|child_id| {
            match package.get(child_id) {
                Ok(package) => {
                    // Despawn drone package
                    commands
                        .entity(child_id)
                        .despawn();

                    // Respawn the package and make it fall on the road
                    let (_, _, i, bg, fg, f, r) = package.tile();
                    let progress = path.0.pos(enemy.advance).unwrap();
                    let offset = enemy.class.get_model().package_offset();
                    let (px, py, pz) = (x + offset.x, y + offset.y, util::z_pos::PACKAGES);
                    commands
                        .spawn(sprite_f32(
                            i, px, py, pz,
                            bg.into(), fg.into(), f, r,
                            textures.tileset.clone(),
                        ))
                        .insert(package.clone())
                        .insert(ClickablePackage)
                        .insert(Animator::new(Tween::new(
                            EaseFunction::CubicOut,
                            Duration::from_millis(util::tweening::PACKAGE_DROP),
                            TransformPositionLens {
                                start: Vec3::new(px, py, pz),
                                end: Vec3::new(
                                    f32_tile_to_f32(progress.x * 2. + 0.5),
                                    f32_tile_to_f32(progress.y * 2. + util::size::GUI_HEIGHT as f32 + 0.5),
                                    util::z_pos::PACKAGES,
                                ),
                            },
                        )))
                    ;
                }
                Err(_) => {
                    // Regular tile -> animate alpha
                    commands
                        .entity(child_id)
                        .insert(Animator::new(Delay::<TextModeTextureAtlasSprite>::new(Duration::from_millis(util::tweening::DRONE_DEATH_FREEZE)).then(
                            tween::tween_text_mode_sprite_opacity(util::tweening::DRONE_DEATH_ALPHA, false)
                        )));
                }
            }
        })
}

pub fn despawn_drone(
    mut commands: Commands,
    mut tween_completed: EventReader<TweenCompleted>,
) {
    for TweenCompleted { entity, user_data } in tween_completed.iter() {
        if *user_data != util::tweening::DRONE_DESPAWN { continue }
        commands.entity(*entity).despawn_recursive();
    }
}