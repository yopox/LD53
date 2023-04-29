use std::cmp::{max, min};

use bevy::prelude::*;
use enum_derived::Rand;

use crate::{GameState, logic, util};
use crate::graphics::loading::Textures;
use crate::graphics::palette::Palette;
use crate::graphics::sprite;
use crate::graphics::sprites::TILE;

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
                draw_road_tiles(
                    RoadElement::rand(), false,
                    p1.x as usize, y,
                    &mut commands, &textures.mrmotext,
                );
            }
        } else {
            let (x1, x2) = (p1.x as usize, p2.x as usize);
            let (x1, x2) = (min(x1, x2), max(x1, x2));
            for x in x1 + 1..x2 {
                draw_road_tiles(
                    RoadElement::rand(), true,
                    x, p1.y as usize,
                    &mut commands, &textures.mrmotext,
                );
            }
        }
    }
    for point in &points {
        draw_road_tiles(RoadElement::Plain, true, point.x as usize, point.y as usize, &mut commands, &textures.mrmotext);
    }
}

#[derive(Rand)]
enum RoadElement {
    #[weight(10)]
    Plain,
    Crossing,
    #[weight(2)]
    Plant1,
    Crack,
    Paper,
}

// TODO: Types of elements (4 tiles, 1 tile random pos, 1 tile repeated)
impl RoadElement {
    /// Returns tiles for a road element with horizontal orientation
    fn get_tiles(&self, horizontal: bool) -> [TILE; 4] {
        match (self, horizontal) {
            (RoadElement::Plain, _) => [
                (0, 1, 0, 3, 16, false, 0),
                (1, 1, 0, 3, 16, false, 0),
                (0, 0, 0, 3, 16, false, 0),
                (1, 0, 0, 3, 16, false, 0),
            ],
            (RoadElement::Crossing, true) => [
                (0, 1, 460, 3, 0, false, 1),
                (1, 1, 460, 3, 0, false, 1),
                (0, 0, 460, 3, 0, false, 1),
                (1, 0, 460, 3, 0, false, 1),
            ],
            (RoadElement::Crossing, false) => [
                (0, 1, 460, 3, 0, false, 0),
                (1, 1, 460, 3, 0, false, 0),
                (0, 0, 460, 3, 0, false, 0),
                (1, 0, 460, 3, 0, false, 0),
            ],
            (RoadElement::Plant1, _) => [
                (0, 1, 462, 3, 7, true, 0),
                (1, 1, 0, 3, 7, true, 0),
                (0, 0, 0, 3, 7, false, 0),
                (1, 0, 0, 3, 7, false, 0),
            ],
            (RoadElement::Crack, true) => [
                (0, 1, 648, 3, 4, true, 0),
                (1, 1, 647, 3, 4, true, 0),
                (0, 0, 647, 3, 4, false, 0),
                (1, 0, 648, 3, 4, false, 0),
            ],
            (RoadElement::Crack, false) => [
                (0, 1, 647, 3, 4, false, 1),
                (1, 1, 648, 3, 4, true, 3),
                (0, 0, 648, 3, 4, false, 1),
                (1, 0, 647, 3, 4, true, 3),
            ],
            (RoadElement::Paper, _) => [
                (0, 1, 0, 3, 16, true, 0),
                (1, 1, 0, 3, 16, true, 0),
                (0, 0, 0, 3, 16, true, 0),
                (1, 0, 109, 3, 0, true, 0),
            ],
        }
    }
}

fn draw_road_tiles(element: RoadElement, horizontal: bool, x: usize, y: usize, commands: &mut Commands, atlas: &Handle<TextureAtlas>) {
    for (dx, dy, i, bg, fg, flip, rotation) in element.get_tiles(horizontal) {
        let tile = sprite(
            i, 2 * x + dx, 2 * y + dy, util::z_pos::ROAD,
            bg.into(), fg.into(),
            flip, rotation, atlas.clone(),
        );
        commands
            .spawn(tile)
            .insert(GridUI);
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