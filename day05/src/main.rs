use std::collections::HashMap;

use shared::load_file_str;

fn main() {
    let input = load_file_str("./input.txt");

    let result1 = puzzle1(&input);
    println!("Puzzle 1: {}", result1);
}

fn puzzle1(input: &str) -> i32 {
    let (ordering_rules, pages_to_produce) = parse(input);

    let mut middle_numbers = Vec::new();

    for pages in pages_to_produce {
        let mut rules = HashMap::new();
        for &(p1, p2) in &ordering_rules {
            if pages.contains(&p1) && pages.contains(&p2) {
                if rules.contains_key(&p1) {
                    rules.insert(p1, rules.get(&p1).expect("val") + 1);
                } else {
                    rules.insert(p1, 1);
                }
            }
        }

        for (idx, page_num) in pages.iter().enumerate() {
            if let Some(&count) = rules.get(page_num) {
                if count == pages.len() - (1 + idx) {
                    continue;
                } else {
                    break;
                }
            } else {
                // last page should not be in the rules :)
                if idx == pages.len() - 1 {
                    println!("correct: {:?}", pages);
                    let middle_num = *pages.get((pages.len() - 1) / 2).expect("number in middle");
                    middle_numbers.push(middle_num);
                } else {
                    break;
                }
            }
        }
    }
    println!("middles: {:?}", middle_numbers);

    middle_numbers.iter().sum()
}

fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (part1, part2) = input.split_once("\n\n").expect("two parts");
    let ordering_rules = parse_part1(part1);
    let pages_to_produce = parse_part2(part2);
    (ordering_rules, pages_to_produce)
}

fn parse_part1(part1: &str) -> Vec<(i32, i32)> {
    let mut values = Vec::new();
    for line in part1.lines() {
        let (left_str, right_str) = line.split_once("|").expect("two numbers");
        let left: i32 = left_str.parse().expect("i32");
        let right: i32 = right_str.parse().expect("i32");
        values.push((left, right));
    }
    values
}

fn parse_part2(part2: &str) -> Vec<Vec<i32>> {
    let mut values = Vec::new();

    for line in part2.lines() {
        let mut numbers = Vec::new();
        for num_str in line.split(",") {
            let num: i32 = num_str.parse().expect("i32");
            numbers.push(num);
        }
        values.push(numbers);
    }

    values
}