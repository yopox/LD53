use bevy::math::Vec2;

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