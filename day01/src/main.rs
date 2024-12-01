use std::{collections::HashMap, fs};

fn main() {
    let result1 = puzzle1();
    println!("Puzzle 1: {}", result1);

    let result2 = puzzle2();
    println!("Puzzle 2: {}", result2);
}

fn puzzle1() -> i32 {
    let (mut left, mut right) = get_lists();

    left.sort();
    right.sort();

    let mut sum_of_differences = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        let diff = (l - r).abs();
        sum_of_differences += diff;
    }

    sum_of_differences
}

fn puzzle2() -> i32 {
    let (left, right) = get_lists();
    let mut right_count: HashMap<i32, i32> = HashMap::new();

    for r in right {
        let count = right_count.entry(r).or_insert(0);
        *count += 1;
    }

    let mut similarity_score = 0;
    for l in left {
        if let Some(count) = right_count.get(&l) {
            similarity_score += l * count;
        }
    }

    similarity_score
}

fn get_lists() -> (Vec<i32>, Vec<i32>) {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let lines_iter = content.split("\n");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines_iter {
        let elements: Vec<&str> = line.split_whitespace().collect();
        if let Some(first) = elements.first() {
            left.push(first.parse().expect("Failed to parse first number"));
        }
        if let Some(last) = elements.last() {
            right.push(last.parse().expect("Failed to parse last number"));
        }
    }

    (left, right)
}
