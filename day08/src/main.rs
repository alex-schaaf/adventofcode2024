use std::collections::HashMap;

use itertools::Itertools;
use shared::load_file_str;

fn main() {
    let input = load_file_str("./input.txt");

    let result1 = puzzle1(&input);
    println!("Puzzle 1: {}", result1);
    
    // let result2 = puzzle2(&input);
    // println!("Puzzle 2: {}", result2);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    // Calculate a distance vector
    fn distance_vec(&self, other: &Point) -> (i32, i32) {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        return (dx, dy);
    }

    fn add_vec(&self, dx: i32, dy: i32) -> Point {
        Point::new(self.x + dx, self.y + dy)
    }

    fn subtract_vec(&self, dx: i32, dy: i32) -> Point {
        Point::new(self.x - dx, self.y - dy)
    }

    fn in_bounds(&self, width: i32, height: i32) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }
}

fn puzzle1(input: &str) -> usize {
    let (antennas, antennas_flat, antinodes, width, height) = parse(input);

    let mut confirmed_antinodes: HashMap<Point, &char> = HashMap::new();

    for (antenna_char, antenna_locs) in antennas.iter() {
        for combinations in antenna_locs.iter().combinations(2) {
            // println!("{:?}", combinations);
            let p1 = combinations[0];
            let p2 = combinations[1];
            let (dx, dy) = p1.distance_vec(p2);
            // println!("{}, {}", dx, dy);
            let a = p2.subtract_vec(dx, dy);
            let b = p1.add_vec(dx, dy);
            // println!("{:?}", p1);
            // println!("{:?}", p2);
            // println!("{:?}", a);
            // println!("{:?}", b);
            for p in vec![a, b].iter() {
                if p.in_bounds(width as i32, height as i32) {
                    if let Some(exists) = confirmed_antinodes.get(&p) {
                        // println!("{exists} at {:?}", a);
                        confirmed_antinodes.insert(*p, exists);
                    } else if let Some(exists) = antennas_flat.get(&p) {
                        // println!("{exists} at {:?}", a);
                        confirmed_antinodes.insert(*p, exists);
                    } else {
                        confirmed_antinodes.insert(*p, &'#');
                    }
                }

            }

        }
    }

    // println!("{:?}", confirmed_antinodes);

    for y in (0..height) {
        for x in (0..width) {
            let p = Point::new(x as i32, y as i32);
            if let Some(an) = confirmed_antinodes.get(&p) {
                print!("{}", '#')
            } else if let Some(an) = antennas_flat.get(&p) {
                print!("{}", an)
            } else {
                print!(".")
            }
        }
        print!("\n")
    }

    confirmed_antinodes.len()
}

fn parse(
    input: &str,
) -> (
    HashMap<char, Vec<Point>>,
    HashMap<Point, char>,
    HashMap<Point, char>,
    usize,
    usize,
) {
    let antenna_symbols: Vec<char> = ('0'..='9').chain('a'..='z').chain('A'..='Z').collect();
    let mut width = 0;
    let mut height: usize = 0;

    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antennas_flat: HashMap<Point, char> = HashMap::new();
    let mut antinodes = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        width = line.len();

        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => continue,
                '#' => {
                    antinodes.insert(Point::new(x as i32, y as i32), ch);
                }
                _ if antenna_symbols.contains(&ch) => {
                    let p = Point::new(x as i32, y as i32);
                    antennas.entry(ch).or_insert_with(Vec::new).push(p.clone());
                    antennas_flat.insert(p.clone(), ch);
                }
                _ => panic!("unexpected char"),
            }
        }

        height += 1;
    }

    (antennas, antennas_flat, antinodes, width, height)
}
