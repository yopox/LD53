use bevy::prelude::*;

use crate::{GameState, util};
use crate::graphics::loading::Textures;
use crate::graphics::palette::Palette;
use crate::graphics::sprite;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                setup.in_schedule(OnEnter(GameState::Main))
            )
            .add_system(
                cleanup.in_schedule(OnExit(GameState::Main))
            )
        ;
    }
}

#[derive(Component)]
struct GridUI;

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    for x in 0..util::size::WIDTH {
        for y in 0..util::size::HEIGHT {
            let tile = sprite(
                0, x, y, util::z_pos::GRID,
                if (x / 2 + y / 2) % 2 == 0 { Palette::F } else { Palette::G }, Palette::Transparent,
                false, 0, textures.mrmotext.clone()
            );
            commands
                .spawn(tile)
                .insert(GridUI);
        }
    }
}

fn cleanup(
    mut commands: Commands,
    entities: Query<Entity, With<GridUI>>
) {
    for id in &entities {
        commands.entity(id).despawn_recursive();
    }
}