use bevy::asset::Handle;
use bevy::hierarchy::{ChildBuilder, DespawnRecursiveExt};
use bevy::input::Input;
use bevy::math::{Vec2, Vec3Swizzles};
use bevy::prelude::{Commands, Component, Entity, EventWriter, MouseButton, Query, Res, ResMut, Transform, Window};
use bevy::sprite::TextureAtlas;
use enum_derived::Rand;

use crate::{graphics, shot, util};
use crate::battle::{CursorState, Money};
use crate::graphics::sprites::TILE;
use crate::logic::tower_stats::OMEGA_DAMAGES;
use crate::music::{PlaySfxEvent, SFX};
use crate::shot::spawn_bomb;
use crate::util::{is_in, z_pos};
use crate::util::size::battle::OMEGA_RANGE;
use crate::util::size::tile_to_f32;

#[derive(Component)]
pub struct ClickablePackage;

#[derive(Rand, Copy, Clone)]
pub enum PackageKind {
    #[weight(20)]
    Common,
    #[weight(2)]
    Money,
    // #[weight(0)]
    // Coffee,
    #[weight(2)]
    Cursed,
    Omega,
}

impl PackageKind {
    fn get_tile(&self) -> usize {
        return match self {
            PackageKind::Common => 393,
            PackageKind::Money => 395,
            // PackageKind::Coffee => 397,
            PackageKind::Cursed => 397,
            PackageKind::Omega => 401,
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
    mut sfx: EventWriter<PlaySfxEvent>,
    packages: Query<(&Package, &Transform, Entity)>,
    windows: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
    state: Option<Res<CursorState>>,
    mut money: ResMut<Money>,
) {
    let Some(cursor_pos) = util::cursor_pos(windows) else { return; };
    if !mouse.just_pressed(MouseButton::Left) { return; }
    let sell = state.is_some() && state.unwrap().eq(&CursorState::Sell);

    for (p, t, id) in &packages {
        // Click on package
        if is_in(cursor_pos, t.translation.xy(), Vec2::new(tile_to_f32(1), tile_to_f32(1))) {
            commands.entity(id).despawn_recursive();

            if sell {
                money.0 += util::package::MONEY_SELL;
                sfx.send(PlaySfxEvent(SFX::SellTower));
            } else {
                match p.kind {
                    PackageKind::Common => { money.0 += util::package::MONEY_SMALL; }
                    PackageKind::Money => { money.0 += util::package::MONEY_BIG; }
                    // PackageKind::Coffee => {}
                    PackageKind::Cursed => { if money.0 >= util::package::MONEY_CURSE { money.0 -= util::package::MONEY_CURSE; } else { money.0 = 0; } }
                    PackageKind::Omega => {
                        spawn_bomb(shot::Bomb::new(cursor_pos, OMEGA_RANGE, OMEGA_DAMAGES), &mut commands);
                    }
                }
                match p.kind {
                    PackageKind::Cursed => sfx.send(PlaySfxEvent(SFX::PackageMalus)),
                    _ => sfx.send(PlaySfxEvent(SFX::PackageBonus)),
                }
            }

        }
    }
}