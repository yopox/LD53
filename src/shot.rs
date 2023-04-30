use std::borrow::Borrow;
use std::time::Duration;

use bevy::ecs::system::EntityCommands;
use bevy::math::{vec2, vec3, Vec3Swizzles};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_tweening::{Animator, Tween, TweenCompleted};
use bevy_tweening::EaseFunction::ElasticIn;
use bevy_tweening::lens::TransformScaleLens;
use strum_macros::EnumIter;

use crate::battle::BattleUI;
use crate::collision::{body_size, BodyType, HitBox};
use crate::enemy::Enemy;
use crate::graphics::MainBundle;
use crate::graphics::sprites::TILE;
use crate::util::z_pos;
use crate::util::size::battle::BOMB_RANGE;
use crate::util::tweening::{BOMB_EXPLODED, SHOT_DESPAWN};

#[derive(Component, Copy, Clone)]
pub struct Shot {
    pub class: Shots,
    pub damages: f32,
    pub speed: f32,
}

#[derive(Copy, Clone, EnumIter, Debug, PartialEq, Eq)]
pub enum Shots {
    Basic,
    Bomb,
}

impl Shots {
    const fn get_shot(&self) -> Shot {
        match &self {
            Self::Basic => Shot {
                class: *self,
                damages: 6.,
                speed: 120.,
            },
            Shots::Bomb => Shot {
                class: *self,
                damages: 10.,
                speed: 120.,
            }
        }
    }

    pub fn instantiate(&self) -> (Shot, HitBox) {
        let body_size = body_size(&[self.get_tile()]);
        let solid_body = HitBox {
            body_type: BodyType::ShipShot,
            width: body_size.x,
            height: body_size.y,
            bottom_right_anchor: false,
        };
        (self.get_shot(), solid_body)
    }

    pub const fn get_tile(&self) -> TILE {
        match &self {
            Shots::Basic => (0, 0, 65, 16, 8, false, 0),
            Shots::Bomb => (0, 0, 65, 16, 7, false, 0),
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.get_shot().speed
    }

    pub fn get_bomb_range(&self) -> f32 {
        match &self {
            Shots::Bomb => BOMB_RANGE,
            _ => {
                warn!("Cannot get bomb range of basic shot");
                0.
            },
        }
    }
}

impl Shot {
    fn get_bomb_range(&self) -> f32 {
        self.class.get_bomb_range()
    }
}

pub fn remove_shots(
    mut tween_completed: EventReader<TweenCompleted>,
    mut commands: Commands,
) {
    for event in tween_completed.iter() {
        if event.user_data == SHOT_DESPAWN {
            if let Some(entity_commands) = commands.get_entity(event.entity) {
                entity_commands.despawn_recursive();
            }
        }
    }
}

#[derive(Component, Copy, Clone)]
pub struct Bomb {
    x: f32,
    y: f32,
    range: f32,
    damages: f32,
}

impl Bomb {
    pub const fn position(&self) -> Vec2 {
        vec2(self.x, self.y)
    }

    pub fn from_shot_translation(shot: Shot, tr: Vec3) -> Self {
        Bomb {
            x: tr.x,
            y: tr.y,
            range: shot.get_bomb_range(),
            damages: shot.damages,
        }
    }
}

pub fn spawn_bomb(bomb: Bomb, mut commands: &mut Commands) {
    commands
        .spawn(bomb)
        .insert(MainBundle::from_xyz(bomb.x, bomb.y, z_pos::BOMB))
        .insert(BattleUI)
    ;
}

pub fn bomb_exploding(
    bombs: Query<&Bomb, Added<Bomb>>,
    mut enemies: Query<(Entity, &mut Enemy, &Transform)>,
    mut commands: Commands,
) {
    for bomb in bombs.iter() {
        for (e_enemy, mut enemy, t_enemy) in enemies.iter_mut() {
            if t_enemy.translation.xy().distance_squared(bomb.position()) <= bomb.range * bomb.range {
                enemy.stats.hp -= bomb.damages;

                if enemy.stats.hp <= 0. {
                    enemy.stats.hp = 0.;
                    commands.get_entity(e_enemy).map(EntityCommands::despawn_recursive);
                }
            }
        }
    }
}

pub fn make_bomb_explode(
    bombs: Query<(Entity, &Bomb), Added<Bomb>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (e, bomb) in bombs.iter() {
        if let Some(mut entity_commands) = commands.get_entity(e) {
            entity_commands.insert(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(bomb.range).into()).into(),
                material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
                transform: Transform {
                    translation: vec3(bomb.x, bomb.y, z_pos::EXPLOSION),
                    scale: Vec3::ZERO,
                    ..default()
                },
                ..default()
            })
                .insert(Animator::new(
                    Tween::new(
                        ElasticIn,
                        Duration::from_secs_f32(0.5),
                        TransformScaleLens {
                            start: Vec3::ZERO,
                            end: Vec3::ONE,
                        }).with_completed_event(BOMB_EXPLODED)
                ));
        }
    }
}

pub fn bomb_exploded(
    mut events: EventReader<TweenCompleted>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if event.user_data == BOMB_EXPLODED {
            commands.get_entity(event.entity).map(EntityCommands::despawn_recursive);
        }
    }
}