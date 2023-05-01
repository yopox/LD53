use std::cmp::{max, min};
use std::collections::HashSet;

use bevy::prelude::*;
use enum_derived::Rand;
use rand::{RngCore, thread_rng};

use crate::{GameState, logic};
use crate::graphics::loading::Textures;
use crate::graphics::sprite;
use crate::graphics::sprites::TILE;
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
    const GRID_WIDTH: f32 = 20.;
    let points = vec![
        Vec2::new(1., 5.),
        Vec2::new(3., 5.),
        Vec2::new(3., 3.),
        Vec2::new(8., 3.),
        Vec2::new(8., 5.),
        Vec2::new(11., 5.),
        Vec2::new(11., 3.),
        Vec2::new(16., 3.),
        Vec2::new(16., 5.),
        Vec2::new(GRID_WIDTH, 5.),
    ];
    let path = logic::path::Path::from_points(points.clone());
    commands.insert_resource(CurrentPath(path));

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

impl RoadElement {
    /// Returns tiles for a road element with horizontal orientation
    fn get_tiles(&self) -> [TILE; 4] {
        match self {
            RoadElement::Road => [
                (0, 1, 416, 4, 16, false, 0),
                (1, 1, 416, 4, 16, false, 0),
                (0, 0, 416, 4, 16, false, 0),
                (1, 0, 416, 4, 16, false, 0),
            ],
            RoadElement::Plain => [
                (0, 1, 64 + thread_rng().next_u32() as usize % 21, 0, 3, false, 0),
                (1, 1, 64 + thread_rng().next_u32() as usize % 21, 0, 3, false, 0),
                (0, 0, 64 + thread_rng().next_u32() as usize % 21, 0, 3, false, 0),
                (1, 0, 64 + thread_rng().next_u32() as usize % 21, 0, 3, false, 0),
            ],
            RoadElement::Rock => [
                (0, 1, 64 + thread_rng().next_u32() as usize % 21, 3, 4, false, 0),
                (1, 1, 64 + thread_rng().next_u32() as usize % 21, 3, 4, false, 0),
                (0, 0, 64 + thread_rng().next_u32() as usize % 21, 3, 4, false, 0),
                (1, 0, 64 + thread_rng().next_u32() as usize % 21, 3, 4, false, 0),
            ],
        }
    }
}

fn draw_road_tiles(grid: &Vec<Vec<RoadElement>>, commands: &mut Commands, atlas: &Handle<TextureAtlas>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            for (dx, dy, i, bg, fg, f, r) in grid[y][x].get_tiles() {
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