use bevy::app::{App, Plugin};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::collide_aabb;

use crate::graphics::sprites::TILE;
use crate::util::size;

/// Handles collisions.
///
/// In order to get a collision we need:
/// - [Hitbox] on the entity with its size (translation + size / 2. = center)
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

/// Takes entity into account for collision detection.
/// [body_type] is used to perform collision detection against the right bodies.
#[derive(Component, Clone)]
pub struct HitBox {
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
    colliders: Query<(&HitBox, &Transform, Entity), Without<Invincible>>,
    mut contact: EventWriter<Contact>,
) {
    let bodies = &colliders.iter().collect::<Vec<(&HitBox, &Transform, Entity)>>();
    for (i, &(body1, pos1, id1)) in bodies.iter().enumerate() {
        for &(body2, pos2, id2) in bodies.iter().skip(i) {
            if !body1.body_type.can_collide(&body2.body_type) { continue; }

            // Collide outer bounds
            // collide 1/3 args must be the center of the rectangle, 2/4 args are the rectangle size
            if collide_aabb::collide(
                vec3(pos1.translation.x + if body1.bottom_right_anchor { -body1.width / 2. } else { body1.width / 2. },
                     pos1.translation.y + body1.height / 2., 0.),
                vec2(body1.width, body1.height),
                vec3(pos2.translation.x + if body2.bottom_right_anchor { -body2.width / 2. } else { body2.width / 2. },
                     pos2.translation.y + body2.height / 2., 0.),
                vec2(body2.width, body2.height),
            ).is_none() { continue; }

            contact.send(Contact((body1.body_type, id1), (body2.body_type, id2)));
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

    for enemy in Drones::iter() {
        assert!(has_hitbox(enemy.get_tiles()), "The monster {:?} has no hitbox!", enemy)
    }

    for shot in Shots::iter() {
        assert!(has_hitbox(shot.get_tiles()), "The weapon {:?} has no hitbox!", shot);
    }
}