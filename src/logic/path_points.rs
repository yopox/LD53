use bevy::math::{vec2, Vec2};

const GRID_WIDTH: f32 = 20.;

pub const PATH_0: [Vec2; 10] = [
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

const PATH_1: [Vec2; 11] = [
    vec2(0., 2.),
    vec2(2., 2.),
    vec2(2., 6.),
    vec2(6., 6.),
    vec2(6., 2.),
    vec2(10., 2.),
    vec2(10., 6.),
    vec2(14., 6.),
    vec2(14., 2.),
    vec2(18., 2.),
    vec2(GRID_WIDTH, 2.),
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
    vec2(15., 7.),
    vec2(15., 6.),
    vec2(15., 5.),
    vec2(14., 5.),
    vec2(14., 4.),
    vec2(13., 4.),
    vec2(13., 3.),
    vec2(12., 3.),
    vec2(12., 2.),
    vec2(11., 2.),
    vec2(11., 1.),
    vec2(GRID_WIDTH, 1.),
];

const PATH_6: [Vec2; 2] = [
    vec2(0., 4.),
    vec2(GRID_WIDTH, 4.),
];

pub fn path_of_level_n(level: u8) -> Vec<Vec2> {
    match level {
        0 => PATH_0.to_vec(),
        1 => PATH_1.to_vec(),
        2 => PATH_2.to_vec(),
        3 => PATH_3.to_vec(),
        4 => PATH_4.to_vec(),
        5 => PATH_5.to_vec(),
        _ => PATH_6.to_vec(),
    }
}