use std::collections::HashSet;

use shared::{load_file_str, Vec2d};

fn main() {
    let input = load_file_str("./input.txt");

    let result1 = puzzle1(&input);
    println!("Puzzle 1: {}", result1);
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn puzzle1(input: &String) -> usize {
    let mut map = parse(input);

    let (mut guard_x, mut guard_y) = match map.find_first('^') {
        Some((x, y)) => {
            println!("Found '^' at position ({}, {})", x, y);
            (x, y)
        }
        None => {
            panic!("'^' not found in the map");
        }
    };
    let _ = map.set(guard_x, guard_y, '.');

    let mut guard_direction = Direction::Up;
    let mut dx: i32;
    let mut dy: i32;
    let mut unique_positions = HashSet::new();
    unique_positions.insert((guard_x as i32, guard_y as i32));
    loop {
        match guard_direction {
            Direction::Up => {
                dx = 0;
                dy = -1;
            }
            Direction::Down => {
                dx = 0;
                dy = 1;
            }
            Direction::Left => {
                dx = -1;
                dy = 0;
            }
            Direction::Right => {
                dx = 1;
                dy = 0;
            }
        }

        let proposed_x = guard_x as i32 + dx;
        let proposed_y = guard_y as i32 + dy;

        if is_out_of_bounds(proposed_x, proposed_y, &map) {
            break;
        }

        let tile = map.get(proposed_x as usize, proposed_y as usize);
        if let Some(&'#') = tile {
            guard_direction = guard_direction.turn_right();
        } else {
            guard_x = proposed_x as usize;
            guard_y = proposed_y as usize;
            unique_positions.insert((proposed_x, proposed_y));
        }
    }

    unique_positions.len()
}

fn is_out_of_bounds(x: i32, y: i32, map: &Vec2d<char>) -> bool {
    if x < 0 || y < 0 || x as usize >= map.get_width() || y as usize >= map.get_height() {
        true
    } else {
        false
    }
}

fn parse(input: &str) -> Vec2d<char> {
    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for line in input.lines() {
        width = line.len();
        for ch in line.chars() {
            data.push(ch);
        }
        height += 1
    }

    Vec2d::new(data, width, height)
}
