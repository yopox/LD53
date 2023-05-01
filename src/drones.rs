use std::time::Duration;

use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;
use bevy_tweening::{Animator, Delay, EaseFunction, Tween, TweenCompleted};
use bevy_tweening::lens::TransformPositionLens;
use strum_macros::EnumIter;

use crate::collision::{body_size, BodyType, Contact, HitBox};
use crate::graphics::{sprite_f32, tween};
use crate::graphics::animation::Wiggle;
use crate::graphics::grid::{CurrentPath, GridElement};
use crate::graphics::loading::Textures;
use crate::graphics::package::{ClickablePackage, Package};
use crate::graphics::sprites::{DroneModels, TILE};
use crate::shot::{Bomb, Shot, Shots, spawn_bomb};
use crate::tower::Slow;
use crate::util;
use crate::util::vec3_with_battle_z;
use crate::util::size::{f32_tile_to_f32, tile_to_f32};

#[derive(Debug, Clone)]
pub struct Stats {
    pub(crate) hp: f32,
    speed: f32,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Drones {
    Simple1,
    Simple2,
    Simple3,
    Medium1,
    Medium2,
    Medium3,
    Medium4,
    Big1,
    Big2,
    Invader, // was here
}

#[derive(Component)]
pub struct Enemy {
    pub class: Drones,
    pub stats: Stats,
    pub advance: f32,
}

impl Drones {
    pub const fn get_default_stats(&self) -> Stats {
        match self {
            Drones::Simple1 | Drones::Simple2 | Drones::Simple3 =>
                Stats { hp: 25., speed: 0.5 },
            Drones::Medium1 | Drones::Medium2 | Drones::Medium3 | Drones::Medium4 =>
                Stats { hp: 80., speed: 0.35 },
            Drones::Big1 | Drones::Big2 =>
                Stats { hp: 300., speed: 0.25 },
            Drones::Invader =>
                Stats { hp: 1000., speed: 0.125 },
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
            Drones::Simple1 => DroneModels::Simple1,
            Drones::Simple2 => DroneModels::Simple2,
            Drones::Simple3 => DroneModels::Simple3,
            Drones::Medium1 => DroneModels::Medium1,
            Drones::Medium2 => DroneModels::Medium2,
            Drones::Medium3 => DroneModels::Medium3,
            Drones::Medium4 => DroneModels::Medium4,
            Drones::Big1 => DroneModels::Big1,
            Drones::Big2 => DroneModels::Big2,
            Drones::Invader => DroneModels::Invader,
        }
    }

    pub fn get_tiles(&self) -> &'static [TILE] { self.get_model().get_tiles() }
}

pub fn update_drones(
    mut drones: Query<(&mut Transform, &mut Enemy, Option<&Slow>)>,
    path: Option<Res<CurrentPath>>,
    time: Res<Time>,
) {
    let Some(path) = path else { return; };
    for (mut pos, mut drone, slowed) in drones.iter_mut() {
        let speed_modulator = match slowed {
            Some(slow) => slow.factor,
            None => 1.,
        };

        drone.advance += speed_modulator * drone.stats.speed * time.delta_seconds();

        let Some(progress) = path.0.pos(drone.advance) else { continue; };

        let size = body_size(drone.class.get_model().get_tiles());
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
                if let Some(entity_commands) = commands.get_entity(*e_shot) {
                    entity_commands.despawn_recursive()
                }

                let Ok((mut enemy, e_pos)) = enemies.get_mut(*e_enemy) else { continue };
                match shot.class {
                    Shots::Bomb => {
                        spawn_bomb(Bomb::from_shot_translation(shot, t_shot.translation), &mut commands);
                    }
                    Shots::Electricity => {
                        enemy.stats.hp -= shot.damage;
                        if enemy.stats.hp > 0. { continue; }
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
    commands: &mut Commands,
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
                    commands.entity(*e_enemy).remove_children(&[child_id]);
                    commands.entity(child_id).despawn();

                    // Respawn the package and make it fall on the road
                    let (_, _, i, bg, fg, f, r) = package.tile();
                    let progress = path.0.pos(enemy.advance).unwrap();
                    let offset = enemy.class.get_model().package_offset();
                    let start = vec3_with_battle_z(x + offset.x, y + offset.y);
                    let end = vec3_with_battle_z(
                        f32_tile_to_f32(progress.x * 2. + 0.5),
                        f32_tile_to_f32(progress.y * 2. + util::size::GUI_HEIGHT as f32 + 0.5),
                    );

                    commands
                        .spawn(sprite_f32(
                            i, start.x, start.y, start.z,
                            bg.into(), fg.into(), f, r,
                            textures.tileset.clone(),
                        ))
                        .insert(package.clone())
                        .insert(ClickablePackage)
                        .insert(GridElement)
                        .insert(Animator::new(Tween::new(
                            EaseFunction::CubicOut,
                            Duration::from_millis(util::tweening::PACKAGE_DROP),
                            TransformPositionLens { start, end },
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