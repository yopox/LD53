use bevy::asset::Handle;
use bevy::hierarchy::ChildBuilder;
use bevy::math::Vec2;
use bevy::prelude::Component;
use bevy::sprite::TextureAtlas;
use enum_derived::Rand;
use rand::RngCore;

use crate::{graphics, util};
use crate::graphics::sprites::TILE;

#[derive(Component)]
pub struct ClickablePackage;

#[derive(Rand, Copy, Clone)]
pub enum PackageKind {
    #[weight(20)]
    Common,
    #[weight(10)]
    Money,
    #[weight(2)]
    Coffee,
    #[weight(2)]
    Cursed,
    Omega,
}

impl PackageKind {
    fn get_tile(&self) -> usize {
        return match self {
            PackageKind::Common => 897 + rand::thread_rng().next_u32() % 27,
            PackageKind::Money => 938,
            PackageKind::Coffee => 616,
            PackageKind::Cursed => 355,
            PackageKind::Omega => 987,
        } as usize;
    }
}

#[derive(Component, Clone)]
pub struct Package {
    kind: PackageKind,
}

impl Package {
    pub fn new() -> Self {
        Package { kind: PackageKind::rand() }
    }

    pub fn tile(&self) -> TILE {
        (0, 0, self.kind.get_tile(), 14, 11, false, 0)
    }
}

pub fn spawn(builder: &mut ChildBuilder, offset: Vec2, atlas: &Handle<TextureAtlas>) {
    let package = Package::new();
    let (_, _, i, bg, fg, f, r) = package.tile();
    builder
        .spawn(graphics::sprite_f32(
            i, offset.x, offset.y, util::z_pos::PACKAGES - util::z_pos::ENEMIES,
            bg.into(), fg.into(), f, r, atlas.clone(),
        ))
        .insert(package)
    ;
}