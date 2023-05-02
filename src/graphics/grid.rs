use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use enum_derived::Rand;
use rand::{RngCore, thread_rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{GameState, logic};
use crate::graphics::loading::Textures;
use crate::graphics::sprite;
use crate::graphics::sprites::TILE;
use crate::logic::path::path_of_level_n;
use crate::util::{battle_z_from_y, size, z_pos};
use crate::util::size::is_oob;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                setup.in_schedule(OnEnter(GameState::Battle))
            )
            .add_system(
                update_z.in_set(OnUpdate(GameState::Battle))
            )
            .add_system(
                cleanup.in_schedule(OnExit(GameState::Battle))
            )
        ;
    }
}

#[derive(Component)]
struct GridUI;

#[derive(Resource)]
pub struct CurrentPath(pub logic::path::Path);

/// BE CAREFUL THERE IS A FACTOR 2 BETWEEN GRID AND TILES, `GRID[.][.]` = 4 TILES
#[derive(Resource)]
pub struct Grid {
    pub elements: Vec<Vec<RoadElement>>,
    pub towers: HashSet<(usize, usize)>,
}

/// Add this to entities on the grid to set their z dynamically
#[derive(Component)]
pub struct GridElement;

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    let level: u8 = 0;
    let points = path_of_level_n(level);
    let path = logic::path::Path::from_points(points.clone());
    commands.insert_resource(CurrentPath(path));

    draw_road(&mut commands, &textures, points);
}

pub fn draw_road(mut commands: &mut Commands, textures: &Res<Textures>, points: Vec<Vec2>) {
    let mut grid = vec![vec![RoadElement::Plain; size::WIDTH]; size::HEIGHT];

    // Draw road
    for i in 0..points.len() - 1 {
        let (p1, p2) = (points[i], points[i + 1]);
        if p1.x == p2.x {
            let x = p1.x as usize;
            let (y1, y2) = (p1.y as usize, p2.y as usize);
            let (y1, y2) = (min(y1, y2), max(y1, y2));
            for y in y1..=y2 {
                if is_oob(x as isize, y as isize) { continue }
                grid[y][x] = RoadElement::Road;
            }
        } else {
            let y = p1.y as usize;
            let (x1, x2) = (p1.x as usize, p2.x as usize);
            let (x1, x2) = (min(x1, x2), max(x1, x2));
            for x in x1..=x2 {
                if is_oob(x as isize, y as isize) { continue }
                grid[y][x] = RoadElement::Road;
            }
        }
    }

    for y in 0..size::GRID_HEIGHT {
        for x in 0..size::WIDTH {
            if grid[y][x] == RoadElement::Road { continue }
            for (dx, dy) in [
                (0, -2),
                (-1, -1), (0, -1), (1, -1),
                (-2, 0), (-1, 0), (1, 0), (2, 0),
                (-1, 1), (0, 1), (1, 1),
                (0, 2),
            ] {
                if is_oob(x as isize + dx, y as isize + dy) { continue }
                if grid[(y as isize + dy) as usize][(x as isize + dx) as usize] == RoadElement::Road {
                    grid[y][x] = RoadElement::Rock;
                    continue
                }
            }
        }
    }

    draw_road_tiles(&grid, &mut commands, &textures.tileset);
    commands.insert_resource(Grid { elements: grid, towers: HashSet::new() });
}

#[derive(Rand, Copy, Clone, PartialEq)]
pub enum RoadElement {
    Plain,
    Road,
    Rock,
}

#[derive(PartialEq, EnumIter, Eq, Hash)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn get_offset(&self) -> (isize, isize) {
        match self {
            Direction::N => (0, 1),
            Direction::NE => (1, 1),
            Direction::E => (1, 0),
            Direction::SE => (1, -1),
            Direction::S => (0, -1),
            Direction::SW => (-1, -1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }
}

impl RoadElement {
    /// Returns tiles for a road element with horizontal orientation
    fn get_tiles(&self, adjacent: &HashMap<Direction, RoadElement>) -> [TILE; 4] {
        let mut tiles = match self {
            RoadElement::Road => [
                (0, 1, 416, 4, 16, false, 0),
                (1, 1, 416, 4, 16, false, 0),
                (0, 0, 416, 4, 16, false, 0),
                (1, 0, 416, 4, 16, false, 0),
            ],
            RoadElement::Plain => {
                let index = || {
                    match thread_rng().next_u32() % 3 {
                        0 | 1 => 0,
                        _ => 64 + thread_rng().next_u32() as usize % 21,
                    }
                };
                [
                    (0, 1, index(), 0, 3, false, 0),
                    (1, 1, index(), 0, 3, false, 0),
                    (0, 0, index(), 0, 3, false, 0),
                    (1, 0, index(), 0, 3, false, 0),
                ]
            },
            RoadElement::Rock => {
                let index = || {
                    match thread_rng().next_u32() % 3 {
                        0 => 0,
                        _ => 64 + thread_rng().next_u32() as usize % 21,
                    }
                };
                [
                    (0, 1, index(), 3, 4, false, 0),
                    (1, 1, index(), 3, 4, false, 0),
                    (0, 0, index(), 3, 4, false, 0),
                    (1, 0, index(), 3, 4, false, 0),
                ]
            },
        };

        let is = |opt: Option<&RoadElement>, r: RoadElement| { opt.is_some() && *opt.unwrap() == r };

        let n = adjacent.get(&Direction::N);
        let e = adjacent.get(&Direction::E);
        let s = adjacent.get(&Direction::S);
        let w = adjacent.get(&Direction::W);

        match self {
            RoadElement::Plain => {
                let (corner, bg, fg) = match thread_rng().next_u32() % 4 {
                    0 => (321, 3, 0),
                    _ => (322, 3, 0),
                };

                let mut corner_changed = false;

                if is(w, RoadElement::Rock) && is(s, RoadElement::Rock) {
                    corner_changed = true;
                    tiles[2] = (0, 0, corner, bg, fg, true, 3);
                } else if is(w, RoadElement::Rock) && is(n, RoadElement::Rock) {
                    corner_changed = true;
                    tiles[0] = (0, 1, corner, bg, fg, true, 2);
                } else if is(e, RoadElement::Rock) && is(s, RoadElement::Rock) {
                    corner_changed = true;
                    tiles[3] = (1, 0, corner, bg, fg, true, 0);
                } else if is(e, RoadElement::Rock) && is(n, RoadElement::Rock) {
                    corner_changed = true;
                    tiles[1] = (1, 1, corner, bg, fg, true, 1);
                }

                if !corner_changed && thread_rng().next_u32() % 5 == 0 {
                    let tile = (130 + thread_rng().next_u32() % 4) as usize;
                    tiles = [
                        (0, 1, tile, 0, 3, false, 0),
                        (1, 1, tile, 0, 3, false, 1),
                        (0, 0, tile, 0, 3, false, 3),
                        (1, 0, tile, 0, 3, false, 2),
                    ];
                }
            }
            RoadElement::Road => {
                if is(w, RoadElement::Road) && is(s, RoadElement::Road) && is(n, RoadElement::Rock) && is(e, RoadElement::Rock) {
                    tiles[1] = (1, 1, 323, 4, 3, true, 1);
                    tiles[3] = (1, 0, 291, 4, 3, true, 1);
                } else if is(w, RoadElement::Road) && is(n, RoadElement::Road) && is(s, RoadElement::Rock) && is(e, RoadElement::Rock) {
                    tiles[1] = (1, 1, 291, 4, 3, false, 3);
                    tiles[3] = (1, 0, 323, 4, 3, false, 3);
                } else if is(e, RoadElement::Road) && is(s, RoadElement::Road) && is(n, RoadElement::Rock) && is(w, RoadElement::Rock) {
                    tiles[0] = (0, 1, 323, 4, 3, false, 1);
                    tiles[2] = (0, 0, 291, 4, 3, false, 1);
                } else if is(e, RoadElement::Road) && is(n, RoadElement::Road) && is(s, RoadElement::Rock) && is(w, RoadElement::Rock) {
                    tiles[0] = (0, 1, 291, 4, 3, true, 3);
                    tiles[2] = (0, 0, 323, 4, 3, true, 3);
                }
            }
            RoadElement::Rock => {
                if is(w, RoadElement::Road) && is(s, RoadElement::Road) {
                    tiles[2] = (0, 0, 324, 4, 3, false, 0);
                } else if is(w, RoadElement::Road) && is(n, RoadElement::Road) {
                    tiles[0] = (0, 1, 324, 4, 3, true, 2);
                } else if is(e, RoadElement::Road) && is(s, RoadElement::Road) {
                    tiles[3] = (1, 0, 324, 4, 3, true, 0);
                } else if is(e, RoadElement::Road) && is(n, RoadElement::Road) {
                    tiles[1] = (1, 1, 324, 4, 3, false, 2);
                }

                if adjacent.values().filter(|e| **e == RoadElement::Plain).count() >= 3 {
                    for mut t in tiles.iter_mut() {
                        if thread_rng().next_u32() % 10 < 7 { t.4 = 0; }
                    }
                }

                let (corner, bg, fg, dr) = match thread_rng().next_u32() % 4 {
                    0 => (320, 3, 0, 0),
                    _ => (322, 3, 0, 2),
                };
                if is(w, RoadElement::Plain) && is(s, RoadElement::Plain) {
                    tiles[2] = (0, 0, corner, bg, fg, true, (3 + dr) % 4);
                } else if is(w, RoadElement::Plain) && is(n, RoadElement::Plain) {
                    tiles[0] = (0, 1, corner, bg, fg, true, (2 + dr) % 4);
                } else if is(e, RoadElement::Plain) && is(s, RoadElement::Plain) {
                    tiles[3] = (1, 0, corner, bg, fg, true, (0 + dr) % 4);
                } else if is(e, RoadElement::Plain) && is(n, RoadElement::Plain) {
                    tiles[1] = (1, 1, corner, bg, fg, true, (1 + dr) % 4);
                }
            }
        }

        tiles
    }
}

fn draw_road_tiles(grid: &Vec<Vec<RoadElement>>, commands: &mut Commands, atlas: &Handle<TextureAtlas>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let mut adjacent: HashMap<Direction, RoadElement> = HashMap::new();
            for dir in Direction::iter() {
                let (dx, dy) = dir.get_offset();
                if x as isize + dx < 0 || y as isize + dy < 0 { continue }
                let Some(vec) = grid.get((y as isize + dy) as usize) else { continue };
                let Some(elem) = vec.get((x as isize + dx) as usize) else { continue };
                adjacent.insert(dir, elem.clone());
            }

            for (dx, dy, i, bg, fg, f, r) in grid[y][x].get_tiles(&adjacent) {
                let tile = sprite(
                    i, 2 * x + dx, 2 * y + dy + size::GUI_HEIGHT, z_pos::ROAD,
                    bg.into(), fg.into(), f, r, atlas.clone(),
                );
                commands
                    .spawn(tile)
                    .insert(GridUI);
            }
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

pub fn update_z(
    mut query: Query<&mut Transform, (Or<(Changed<Transform>, Added<Transform>)>, With<GridElement>)>,
) {
    for mut pos in query.iter_mut() {
        pos.translation.z = battle_z_from_y(pos.translation.y);
    }
}