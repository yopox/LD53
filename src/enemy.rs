use bevy::prelude::*;
use crate::graphics::{MainBundle, sprite_from_tile};
use crate::graphics::sprites::{TILE, CASH_KNIGHT, DEFAULT_PALETTE};
use crate::util;

#[derive(Debug, Default, Clone, Copy)]
pub struct EnemyStats {
    hp: f32,
    speed: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Enemies {
    Drone
}

impl Enemies {
    pub const fn get_default_stats(&self) -> EnemyStats {
        match self {
            Self::Drone => EnemyStats {
                hp: 10.,
                speed: 32.,
            }
        }
    }

    pub const fn get_tiles(&self) -> &[TILE] {
        match self {
            Self::Drone => &CASH_KNIGHT,
        }
    }
}

#[derive(Component)]
pub struct Enemy {
    class: Enemies,
    stats: EnemyStats,
}

pub fn spawn_enemy(
    class: Enemies,
    position: Vec2,
    atlas: &Handle<TextureAtlas>,
    mut commands: Commands,
) {
    commands.spawn(Enemy {
        class, stats: class.get_default_stats().clone()
    }).insert(
        MainBundle::from_xyz(position.x, position.y,util::z_pos::ENEMIES)
    ).with_children(|builder|
        sprite_from_tile(builder, class.get_tiles(), atlas, DEFAULT_PALETTE.into(), 0.))
    ;
}