use std::cmp::{max, min};

use bevy::prelude::*;
use enum_derived::Rand;
use rand::RngCore;

use crate::{GameState, logic, util};
use crate::graphics::{sprite, sprites};
use crate::graphics::loading::Textures;

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

#[derive(Resource)]
pub struct CurrentPath(pub logic::path::Path);

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    let points = vec![
        Vec2::new(0., 6.),
        Vec2::new(3., 6.),
        Vec2::new(3., 4.),
        Vec2::new(8., 4.),
        Vec2::new(8., 6.),
        Vec2::new(11., 6.),
        Vec2::new(11., 4.),
        Vec2::new(16., 4.),
        Vec2::new(16., 6.),
        Vec2::new(20., 6.),
    ];
    let path = logic::path::Path::from_points(points.clone());
    commands.insert_resource(CurrentPath(path));

    let mut grid = vec![vec![RoadElement::Plain; util::size::WIDTH]; util::size::HEIGHT];

    let is_oob = |x: isize, y: isize| { x < 0 || y < 0 || x >= util::size::WIDTH as isize || y >= util::size::HEIGHT as isize };

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

    for y in 0..util::size::HEIGHT {
        for x in 0..util::size::WIDTH {
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

    draw_road_tiles(&grid, &mut commands, &textures.mrmotext);
}

#[derive(Rand, Copy, Clone, PartialEq)]
enum RoadElement {
    Plain,
    Road,
    Rock,
}

// TODO: Types of elements (4 tiles, 1 tile random pos, 1 tile repeated)
impl RoadElement {
    /// Returns tiles for a road element with horizontal orientation
    fn get_tiles(&self) -> (sprites::INDEX, sprites::BG, sprites::FG, sprites::FLIP, sprites::ROTATION) {
        match self {
            RoadElement::Plain | RoadElement::Road => (0, 4, 3, false, 0),
            RoadElement::Rock => ((202 + rand::thread_rng().next_u32() % 3) as usize, 4, 3, false, 0),
        }
    }
}

fn draw_road_tiles(grid: &Vec<Vec<RoadElement>>, commands: &mut Commands, atlas: &Handle<TextureAtlas>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let (i, bg, fg, f, r) = grid[y][x].get_tiles();
            let tile = sprite(
                i, x, y, util::z_pos::ROAD,
                bg.into(), fg.into(), f, r, atlas.clone(),
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