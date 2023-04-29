use bevy::prelude::*;
use crate::shot::Shot;

#[derive(Component)]
pub struct Tower {
    reloading_delay: f32,
    shot: Shot,
}