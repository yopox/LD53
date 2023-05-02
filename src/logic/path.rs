use bevy::math::vec2;
use bevy::prelude::*;

pub struct Path {
    /// Vec2(x, y) -> (1., 2.) is the center of the tile (1, 2)
    points: Vec<Vec2>,
    /// Total length of the path
    length: f32,
    /// Length of each segment
    segments: Vec<f32>,
    /// Combined segments length
    combined: Vec<f32>,
}

impl Path {
    pub fn from_points(points: Vec<Vec2>) -> Self {
        let mut length: f32 = 0.;
        let mut segments = vec![];
        let mut combined = vec![];


        for i in 0..points.len() - 1 {
            let (p1, p2) = (points[i], points[i + 1]);
            assert!(p1.x == p2.x || p1.y == p2.y);
            let distance = p1.distance(p2);
            length += distance;
            segments.push(distance);
            combined.push(length);
        }

        Path { points, length, segments, combined }
    }

    /// Returns the position on the path after walking [length] (1. = tile side).
    pub fn pos(&self, length: f32) -> Option<Vec2> {
        if length < 0. { return None; }
        if length >= self.length {
            return match self.points.last() {
                Some(pos) => Some(*pos),
                None => None,
            }
        }

        // Current segment nb
        let Some((i, _)) = self.combined.iter().enumerate().find(|(_, d)| **d > length) else { return None };

        // Proportion of the current segment
        let in_progress = length - if i > 0 { self.combined[i-1] } else { 0. };
        let proportion = in_progress / self.segments[i];

        // Compute position
        let Vec2 { x: x1, y: y1 } = self.points[i];
        let Vec2 { x: x2, y: y2 } = self.points[i + 1];
        let x = x1 + proportion * (x2 - x1);
        let y = y1 + proportion * (y2 - y1);

        return Some(Vec2::new(x, y));
    }

    pub fn drone_won(&self, advance: f32) -> bool {
        advance >= self.length
    }
}

#[test]
fn test_path_length() {
    let path = Path::from_points(vec![
        Vec2::new(0., 0.),
        Vec2::new(2., 0.),
    ]);
    assert_eq!(path.length, 2.);
}

#[test]
fn test_path_segments() {
    let path = Path::from_points(vec![
        Vec2::new(0., 0.),
        Vec2::new(2., 0.),
        Vec2::new(2., 4.),
    ]);
    assert_eq!(path.segments, vec![2., 4.,]);
    assert_eq!(path.combined, vec![2., 6.,]);
}

#[test]
fn test_path_position() {
    let path = Path::from_points(vec![
        Vec2::new(1., 0.),
        Vec2::new(2., 0.),
        Vec2::new(2., 4.),
    ]);
    assert_eq!(path.pos(-1.), None);
    assert_eq!(path.pos(0.), Some(Vec2::new(1., 0.)));
    assert_eq!(path.pos(0.5), Some(Vec2::new(1.5, 0.)));
    assert_eq!(path.pos(2.), Some(Vec2::new(2., 1.)));
    assert_eq!(path.pos(5.), Some(Vec2::new(2., 4.)));
}

#[test]
fn test_path_some() {
    let path = Path::from_points(vec![
        Vec2::new(1., 0.),
        Vec2::new(2., 0.),
        Vec2::new(2., 4.),
    ]);
    for i in 0..100000 {
        assert_ne!(path.pos(path.length * i as f32 / 100000.), None);
    }
}

const GRID_WIDTH: f32 = 20.;

pub const PATH_1: [Vec2; 10] = [
    vec2(0., 5.),
    vec2(3., 5.),
    vec2(3., 3.),
    vec2(8., 3.),
    vec2(8., 5.),
    vec2(11., 5.),
    vec2(11., 3.),
    vec2(16., 3.),
    vec2(16., 5.),
    vec2(GRID_WIDTH, 5.),
];

const PATH_2: [Vec2; 14] = [
    vec2(0., 7.),
    vec2(4., 7.),
    vec2(4., 6.),
    vec2(13., 6.),
    vec2(13., 7.),
    vec2(15., 7.),
    vec2(15., 4.),
    vec2(4., 4.),
    vec2(4., 1.),
    vec2(6., 1.),
    vec2(6., 2.),
    vec2(15., 2.),
    vec2(15., 1.),
    vec2(GRID_WIDTH, 1.),
];

const PATH_3: [Vec2; 10] = [
    vec2(10., 5.),
    vec2(7., 5.),
    vec2(7., 3.),
    vec2(13., 3.),
    vec2(13., 7.),
    vec2(5., 7.),
    vec2(5., 1.),
    vec2(15., 1.),
    vec2(15., 2.),
    vec2(GRID_WIDTH, 2.),
];

const PATH_4: [Vec2; 8] = [
    vec2(9., 0.),
    vec2(9., 1.),
    vec2(3., 1.),
    vec2(3., 7.),
    vec2(17., 7.),
    vec2(17., 1.),
    vec2(11., 1.),
    vec2(11., 0.),
];

const PATH_5: [Vec2; 13] = [
    vec2(0., 7.),
    vec2(12., 7.),
    vec2(12., 6.),
    vec2(12., 5.),
    vec2(11., 5.),
    vec2(11., 4.),
    vec2(10., 4.),
    vec2(10., 3.),
    vec2(9., 3.),
    vec2(9., 2.),
    vec2(8., 2.),
    vec2(8., 1.),
    vec2(GRID_WIDTH, 1.),
];

const PATH_6: [Vec2; 2] = [
    vec2(0., 4.),
    vec2(GRID_WIDTH, 4.),
];

pub fn path_of_level_n(level: u8) -> Vec<Vec2> {
    match level {
        1 => PATH_1.to_vec(),
        2 => PATH_2.to_vec(),
        3 => PATH_3.to_vec(),
        4 => PATH_4.to_vec(),
        5 => PATH_5.to_vec(),
        _ => PATH_6.to_vec(),
    }
}
