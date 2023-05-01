use bevy::asset::Handle;
use bevy::hierarchy::{ChildBuilder, DespawnRecursiveExt};
use bevy::input::Input;
use bevy::math::{Vec2, Vec3Swizzles};
use bevy::prelude::{Commands, Component, Entity, MouseButton, Query, Res, ResMut, Transform, Window};
use bevy::sprite::TextureAtlas;
use enum_derived::Rand;
use rand::RngCore;

use crate::{graphics, util};
use crate::battle::Money;
use crate::graphics::sprites::TILE;
use crate::util::{battle_z_from_y, is_in, z_pos};
use crate::util::size::tile_to_f32;

#[derive(Component)]
pub struct ClickablePackage;

#[derive(Rand, Copy, Clone)]
pub enum PackageKind {
    #[weight(20)]
    Common,
    #[weight(2)]
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
            i, offset.x, offset.y, z_pos::ATTACHED_PACKAGE_OFFSET,
            bg.into(), fg.into(), f, r, atlas.clone(),
        ))
        .insert(package)
    ;
}

pub fn collect_package(
    mut commands: Commands,
    packages: Query<(&Package, &Transform, Entity)>,
    windows: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
    mut money: ResMut<Money>,
) {
    let Some(cursor_pos) = util::cursor_pos(windows) else { return; };
    if !mouse.just_pressed(MouseButton::Left) { return; }
    for (p, t, id) in &packages {
        // Click on package
        if is_in(cursor_pos, t.translation.xy(), Vec2::new(tile_to_f32(1), tile_to_f32(1))) {
            commands.entity(id).despawn_recursive();
            match p.kind {
                PackageKind::Common => { money.0 += util::package::MONEY_SMALL; }
                PackageKind::Money => { money.0 += util::package::MONEY_BIG; }
                PackageKind::Coffee => {}
                PackageKind::Cursed => {}
                PackageKind::Omega => {}
            }
        }
    }
}