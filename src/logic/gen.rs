use std::collections::HashSet;

use bevy::math::{vec2, Vec2};
use rand::{RngCore, thread_rng};

use crate::util;

pub fn gen_path() -> Vec<Vec2> {
    let mut current_x: u8 = 0;
    let mut current_y: u8 = (thread_rng().next_u32() % 5 + 3) as u8;

    let mut covered: HashSet<(u8, u8)> = HashSet::new();
    covered.insert((current_x, current_y));

    let mut path: Vec<Vec2> = Vec::new();
    path.push(vec2(current_x as f32, current_y as f32));

    let mut tries = 0;
    while current_x < 20 {
        tries += 1;
        if tries > 1000 { return gen_path(); }

        let amount = (thread_rng().next_u32() % 3 + 1) as u8;
        match thread_rng().next_u32() % 4 {
            0 => {
                // Move right
                let valid = ((current_x + 1)..=(current_x + amount)).all(|x| !covered.contains(&(x, current_y)));
                if !valid { continue; }

                ((current_x + 1)..=(current_x + amount)).for_each(|x| { covered.insert((x, current_y)); });
                current_x += amount;
            }
            1 => {
                // Move down
                let valid = current_y > amount
                    && ((current_y - amount)..(current_y)).all(|y| !covered.contains(&(current_x, y)));
                if !valid { continue; }

                ((current_y - amount)..(current_y)).for_each(|y| { covered.insert((current_x, y)); });
                current_y -= amount;
            }
            2 => {
                // Move left
                let valid = current_x > amount
                    && ((current_x - amount)..(current_x)).all(|x| !covered.contains(&(x, current_y)));
                if !valid { continue; }

                ((current_x - amount)..(current_x)).for_each(|x| { covered.insert((x, current_y)); });
                current_x -= amount;
            }
            _ => {
                // Move up
                let valid = current_y + amount + 1 < (util::size::HEIGHT - util::size::GUI_HEIGHT) as u8 / 2
                    && ((current_y + 1)..=(current_y + amount)).all(|y| !covered.contains(&(current_x, y)));
                if !valid { continue; }

                ((current_y + 1)..=(current_y + amount)).for_each(|y| { covered.insert((current_x, y)); });
                current_y += amount;
            }
        }
        path.push(vec2(current_x as f32, current_y as f32));
    }
    path
}