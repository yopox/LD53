use bevy::app::{App, Plugin};
use bevy::hierarchy::HierarchyQueryExt;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::collide_aabb;
use bevy::utils::default;
use bevy_text_mode::TextModeTextureAtlasSprite;
use strum::IntoEnumIterator;

use crate::enemy::Enemies;
use crate::graphics::palette::Palette;
use crate::graphics::sprites::TILE;
use crate::shot::Shots;
use crate::util::size;

/// Handles collisions.
///
/// In order to get a collision we need:
/// - [SolidBody] on the parent with its size (translation + size / 2. = center)
/// - [TextModeTextureAtlasSprite] on the children entities, and [HitBox::for_tile] returning a hitbox
/// A [Contact] event will be sent after the collision.
/// To pause collisions momentarily, add an [Invincible] component with the desired cooldown.
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Contact>()
            .add_systems((collide, update_invincible));
    }
}

/// Takes entity into account for collision detection. Entity children should have a [HitBox].
/// [body_type] is used to perform collision detection against the right bodies.
/// [width] and [height] describe a rectangle containing all the children [HitBox]-es.
#[derive(Component)]
pub struct SolidBody {
    pub body_type: BodyType,
    pub width: f32,
    pub height: f32,
    pub bottom_right_anchor: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BodyType {
    Enemy,
    ShipShot,
}

impl BodyType {
    fn can_collide(&self, other: &BodyType) -> bool {
        match (self, other) {
            (BodyType::Enemy, BodyType::ShipShot) | (BodyType::ShipShot, BodyType::Enemy) => true,
            _ => false
        }
    }
}

pub struct Contact(pub (BodyType, Entity), pub (BodyType, Entity));

/// Excludes the entity from collision detection.
#[derive(Component, Debug)]
pub struct Invincible(pub usize);

#[derive(Component, Default, Clone, Copy)]
pub struct HitBox {
    pub dx: f32,
    pub dy: f32,
    pub width: f32,
    pub height: f32,
}

impl HitBox {
    /// Returns the collider for the tile [index]
    pub fn for_tile(index: usize, transparent_bg: bool) -> Option<HitBox> {
        match (index, transparent_bg) {
            // Dash: full width, 1px height, 4px dy
            (877, _) => Some(HitBox { width: 8.0, height: 1.0, dy: 4.0, ..default() }),
            // Laser: half height
            (336, _) => Some(HitBox { width: 8.0, height: 4.0, dy: 2.0, ..default() }),
            // Empty tile, transparent: no hitbox
            (0, true) => None,
            // Enemy details, no hitbox
            (619, _) | (268, _) | (487, _) | (777, _) | (967, _)
            | (463, _) | (397, _) | (643, _) => None,
            // Ship border, no hitbox
            (56, _) | (59, _) | (231, _) => None,
            // Default case: whole box
            _ => Some(HitBox { width: 8.0, height: 8.0, ..default() }),
        }
    }

    // TODO: formula to update hitbox based on the tile [flip] and [rotation] properties.
    pub fn with_flip_and_rotation(&self, flip: bool, rotation: u8) -> Self {
        HitBox {
            dx: self.dx,
            dy: self.dy,
            width: self.width,
            height: self.height,
        }
    }
}

pub fn body_size(sprite: &[TILE]) -> Vec2 {
    let x = *sprite.iter().map(|(x, _, _, _, _, _, _)| x).max().unwrap_or(&0);
    let y = *sprite.iter().map(|(_, y, _, _, _, _, _)| y).max().unwrap_or(&0);
    return vec2(size::tile_to_f32(x + 1), size::tile_to_f32(y + 1));
}

pub fn update_invincible(
    mut commands: Commands,
    mut invincible: Query<(&mut Invincible, &mut Visibility, Entity)>,
) {
    for (mut inv, mut visibility, id) in invincible.iter_mut() {
        if inv.0 == 0 { commands.entity(id).remove::<Invincible>(); } else {
            inv.0 -= 1;
            visibility.set_if_neq(if (inv.0 / 20) % 2 == 0 { Visibility::Inherited } else { Visibility::Hidden });
        }
    }
}

pub fn collide(
    colliders: Query<(&SolidBody, &Transform, Entity), Without<Invincible>>,
    children_query: Query<&Children>,
    hitboxes: Query<(&TextModeTextureAtlasSprite, &Transform), Without<SolidBody>>,
    mut contact: EventWriter<Contact>,
) {
    let bodies = &colliders.iter().collect::<Vec<(&SolidBody, &Transform, Entity)>>();
    for (i, &(body1, pos1, id1)) in bodies.iter().enumerate() {
        'for_body: for &(body2, pos2, id2) in bodies.iter().skip(i) {
            if !body1.body_type.can_collide(&body2.body_type) { continue; }

            // Collide outer bounds first to avoid complex computations
            // collide 1/3 args must be the center of the rectangle, 2/4 args are the rectangle size
            if collide_aabb::collide(
                vec3(pos1.translation.x + if body1.bottom_right_anchor { -body1.width / 2. } else { body1.width / 2. },
                     pos1.translation.y + body1.height / 2., 0.),
                vec2(body1.width, body1.height),
                vec3(pos2.translation.x + if body2.bottom_right_anchor { -body2.width / 2. } else { body2.width / 2. },
                     pos2.translation.y + body2.height / 2., 0.),
                vec2(body2.width, body2.height),
            ).is_none() { continue; }

            // Collide entity 1 children with entity 2 children
            let transparent: Color = Palette::Transparent.into();
            for child1 in children_query.iter_descendants(id1) {
                let Ok((sprite1, cpos1)) = hitboxes.get(child1) else { continue; };
                let Some(mut hitbox1) = HitBox::for_tile(sprite1.index, sprite1.bg == transparent) else { continue; };
                hitbox1 = hitbox1.with_flip_and_rotation(sprite1.flip_x, sprite1.rotation);

                for child2 in children_query.iter_descendants(id2) {
                    let Ok((sprite2, cpos2)) = hitboxes.get(child2) else { continue; };
                    let Some(mut hitbox2) = HitBox::for_tile(sprite2.index, sprite2.bg == transparent) else { continue; };
                    hitbox2 = hitbox2.with_flip_and_rotation(sprite2.flip_x, sprite2.rotation);

                    if collide_aabb::collide(
                        vec3(pos1.translation.x + cpos1.translation.x + hitbox1.dx + hitbox1.width / 2.,
                             pos1.translation.y + cpos1.translation.y + hitbox1.dy + hitbox1.height / 2., 0.),
                        vec2(hitbox1.width, hitbox1.height),
                        vec3(pos2.translation.x + cpos2.translation.x + hitbox2.dx + hitbox2.width / 2.,
                             pos2.translation.y + cpos2.translation.y + hitbox2.dy + hitbox2.height / 2., 0.),
                        vec2(hitbox2.width, hitbox2.height),
                    ).is_some() {
                        contact.send(Contact((body1.body_type, id1), (body2.body_type, id2)));
                        break 'for_body;
                    }
                }
            }
        }
    }
}

#[test]
fn sprites_have_hitbox() {
    let has_hitbox = |sprite: &[TILE]| {
        sprite
            .iter()
            .find(|(_, _, index, bg, _, _, _)| HitBox::for_tile(*index, *bg == 0).is_some())
            .is_some()
    };

    for enemy in Enemies::iter() {
        assert!(has_hitbox(enemy.get_tiles()), "The monster {:?} has no hitbox!", enemy)
    }

    for shot in Shots::iter() {
        assert!(has_hitbox(shot.get_tiles()), "The weapon {:?} has no hitbox!", shot);
    }
}