use regex::Regex;
use shared::load_file_str;

fn main() {
    let input = load_file_str("./input.txt");

    let result1 = puzzle1(&input);
    println!("Puzzle 1: {}", result1);

    let result2 = puzzle2(&input);
    println!("Puzzle 2: {}", result2);
}

fn puzzle1(memory: &String) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut result = 0;
    for mat in re.find_iter(memory) {
        if let Some((left, right)) = parse(mat.as_str()) {
            result += left * right;
        } else {
            panic!("parsing didn't work, mate")
        }
    }
    result
}

fn puzzle2(memory: &String) -> i32 {
    let re = Regex::new(r"don't\(\)|do\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut result = 0;
    let mut enabled = true;
    for mat in re.find_iter(memory) {
        match mat.as_str() {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if enabled {
                    if let Some((left, right)) = parse(mat.as_str()) {
                        result += left * right;
                    }
                }
            }
        }
    }

    result
}

fn parse(stmt: &str) -> Option<(i32, i32)> {
    let trimmed = &stmt[4..stmt.len() - 1];
    let (left, right) = trimmed.split_once(",")?;
    let left_num: i32 = left.trim().parse().ok()?;
    let right_num: i32 = right.trim().parse().ok()?;
    Some((left_num, right_num))
}
