use std::cmp::{max, min};

use bevy::prelude::*;

use crate::{GameState, logic, util};
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
    let points = vec![
        Vec2::new(0., 7.),
        Vec2::new(3., 7.),
        Vec2::new(3., 3.),
        Vec2::new(8., 3.),
        Vec2::new(8., 7.),
        Vec2::new(11., 7.),
        Vec2::new(11., 3.),
        Vec2::new(16., 3.),
        Vec2::new(16., 7.),
        Vec2::new(20., 7.),
    ];
    let path = logic::path::Path::from_points(points.clone());

    // Draw grass
    for x in 0..util::size::WIDTH {
        for y in 0..util::size::HEIGHT {
            let tile = sprite(
                0, x, y, util::z_pos::GRID,
                Palette::G, Palette::Transparent,
                false, 0, textures.mrmotext.clone(),
            );
            commands
                .spawn(tile)
                .insert(GridUI);
        }
    }

    // Draw road
    for i in 0..points.len() - 1 {
        let (p1, p2) = (points[i], points[i + 1]);
        if p1.x == p2.x {
            let (y1, y2) = (p1.y as usize, p2.y as usize);
            let (y1, y2) = (min(y1, y2), max(y1, y2));
            for y in y1 + 1..y2 {
                draw_road_tiles(p1.x as usize, y, &mut commands, &textures.mrmotext);
            }
        } else {
            let (x1, x2) = (p1.x as usize, p2.x as usize);
            let (x1, x2) = (min(x1, x2), max(x1, x2));
            for x in x1 + 1..x2 {
                draw_road_tiles(x, p1.y as usize, &mut commands, &textures.mrmotext);
            }
        }
    }
    for point in &points {
        draw_road_tiles(point.x as usize, point.y as usize, &mut commands, &textures.mrmotext);
    }
}

fn draw_road_tiles(x: usize, y: usize, commands: &mut Commands, atlas: &Handle<TextureAtlas>) {
    for x in [2 * x, 2 * x + 1] {
        for y in [2 * y, 2 * y + 1] {
            let tile = sprite(
                0, x, y, util::z_pos::ROAD,
                Palette::D, Palette::Transparent,
                false, 0, atlas.clone(),
            );
            commands
                .spawn(tile)
                .insert(GridUI);
        }
    }
}

fn cleanup(
    mut commands: Commands,
    entities: Query<Entity, With<GridUI>>,
) {
    for id in &entities {
        commands.entity(id).despawn_recursive();
    }
}