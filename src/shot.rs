use std::time::Duration;

use bevy::ecs::system::EntityCommands;
use bevy::math::{vec2, Vec3Swizzles};
use bevy::prelude::*;
use bevy_tweening::{AssetAnimator, Tween, TweenCompleted};
use bevy_tweening::EaseFunction::CubicOut;
use bevy_tweening::lens::ColorMaterialColorLens;
use strum_macros::EnumIter;

use crate::battle::BattleUI;
use crate::collision::{BodyType, HitBox};
use crate::drones::Enemy;
use crate::graphics::{circle, MainBundle};
use crate::graphics::palette::Palette;
use crate::graphics::sprites::TILE;
use crate::tower::Tower;
use crate::util::size::battle::BOMB_RANGE;
use crate::util::tweening::{BOMB_EXPLODED, SHOT_DESPAWN};
use crate::util::z_pos;

#[derive(Component, Copy, Clone)]
pub struct Shot {
    pub class: Shots,
    pub damage: f32,
    pub speed: f32,
}

#[derive(Copy, Clone, EnumIter, Debug, PartialEq, Eq)]
pub enum Shots {
    Electricity,
    Bomb,
}

impl Shots {
    fn get_shot(&self, tower: &Tower) -> Shot {
        Shot {
            class: *self,
            damage: tower.damage(),
            speed: tower.shot_speed(),
        }
    }

    pub fn instantiate(&self, tower: &Tower) -> (Shot, HitBox) {
        let hitbox: Vec2 = self.get_hitbox();
        let shot = self.get_shot(tower);
        let solid_body = HitBox {
            body_type: BodyType::ShipShot,
            width: hitbox.x,
            height: hitbox.y,
            offset: self.get_offset(),
            single_hit: self.is_single_hit(),
        };
        (shot, solid_body)
    }

    pub const fn is_single_hit(&self) -> bool {
        match self {
            Shots::Electricity => true,
            Shots::Bomb => true,
        }
    }

    pub const fn get_tile(&self) -> TILE {
        match self {
            Shots::Electricity => (0, 0, 35, 16, 8, false, 0),
            Shots::Bomb => (0, 0, 32, 16, 10, false, 0),
        }
    }

    pub const fn get_hitbox(&self) -> Vec2 {
        match self {
            _ => Vec2::new(4., 4.),
        }
    }

    pub const fn get_offset(&self) -> Vec2 {
        match self {
            _ => Vec2::new(2., 2.),
        }
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
    radius: f32,
    damages: f32,
}

impl Bomb {
    pub const fn new(pos: Vec2, radius: f32, damages: f32) -> Self {
        Bomb {
            x: pos.x,
            y: pos.y,
            radius,
            damages,
        }
    }

    pub const fn position(&self) -> Vec2 {
        vec2(self.x, self.y)
    }

    pub fn from_shot_translation(shot: Shot, tr: Vec3) -> Self {
        Bomb {
            x: tr.x,
            y: tr.y,
            radius: shot.get_bomb_range(),
            damages: shot.damage,
        }
    }
}

pub fn spawn_bomb(bomb: Bomb, commands: &mut Commands) {
    commands
        .spawn(bomb)
        .insert(MainBundle::from_xyz(bomb.x, bomb.y, z_pos::BOMB))
        .insert(BattleUI)
    ;
}

pub fn bomb_exploding(
    bombs: Query<&Bomb, Added<Bomb>>,
    mut enemies: Query<(&mut Enemy, &Transform)>,
) {
    for bomb in bombs.iter() {
        for (mut enemy, t_enemy) in enemies.iter_mut() {
            if t_enemy.translation.xy().distance_squared(bomb.position()) <= bomb.radius * bomb.radius {
                enemy.stats.hp = (enemy.stats.hp - bomb.damages).max(0.);
            }
        }
    }
}

pub fn make_bomb_explode(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    bombs: Query<(Entity, &Bomb), Added<Bomb>>,
    circles: Res<circle::Circles>,
) {
    for (e, bomb) in bombs.iter() {
        if let Some(mut entity_commands) = commands.get_entity(e) {
            let color: Color = Palette::K.transparent(0.25);
            let material = materials.add(color.into());
            let mut end_color = color;
            end_color.set_a(0.0);
            entity_commands
                .insert(circle::mesh(
                    &circles, &material, bomb.radius,
                    bomb.x, bomb.y, z_pos::EXPLOSION,
                ))
                .insert(AssetAnimator::<ColorMaterial>::new(
                    material,
                    Tween::new(
                        CubicOut,
                        Duration::from_secs_f32(0.5),
                        ColorMaterialColorLens {
                            start: color,
                            end: end_color,
                        }).with_completed_event(BOMB_EXPLODED),
                ))
            ;
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