use std::collections::HashSet;

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
    pub offset: Vec2,
    pub single_hit: bool,
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
    mut colliders: Query<(&HitBox, &Transform, Entity), Without<Invincible>>,
    mut contact: EventWriter<Contact>,
) {
    let mut unique = HashSet::new();
    let bodies = colliders.iter_mut().collect::<Vec<(&HitBox, &Transform, Entity)>>();
    for b1 in 0..bodies.len() {
        'next_body: for b2 in b1 + 1..bodies.len() {
            let (body1, pos1, id1) = bodies[b1];
            let (body2, pos2, id2) = bodies[b2];

            // Check if bodies can collide
            if !body1.body_type.can_collide(&body2.body_type) { continue; }

            // Collide outer bounds
            // collide 1/3 args must be the center of the rectangle, 2/4 args are the rectangle size
            if collide_aabb::collide(
                vec3(
                    pos1.translation.x + body1.width / 2. + body1.offset.x,
                    pos1.translation.y + body1.height / 2. + body1.offset.y,
                    0.,
                ),
                vec2(body1.width, body1.height),
                vec3(
                    pos2.translation.x + body2.width / 2. + body2.offset.x,
                    pos2.translation.y + body2.height / 2. + body2.offset.y,
                    0.,
                ),
                vec2(body2.width, body2.height),
            ).is_none() { continue; }

            // Check if bodies have single_hit
            for (b, id) in [(body1, id1), (body2, id2)] {
                if b.single_hit {
                    if unique.contains(&id.index()) { continue 'next_body; } else { unique.insert(id.index()); }
                }
            }

            // Send a contact event
            contact.send(Contact(
                (body1.body_type, id1),
                (body2.body_type, id2),
            ));
        }
    }
}