use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct Shot {
    damages: f32,
}

pub enum Shots {
    Basic,
}

impl Shots {
    pub const fn get_default_damages(&self) -> f32 {
        match self {
            Self::Basic => 6.,
        }
    }
}