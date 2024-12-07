use itertools::Itertools;

use shared::load_file_str;

fn main() {
    let input = load_file_str("./input.txt");

    let result1 = puzzle1(&input);
    println!("Puzzle 1: {}", result1);

    let result2 = puzzle2(&input);
    println!("Puzzle 2: {}", result2);
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

fn puzzle1(input: &str) -> i64 {
    let equations = parse(input);

    let allowed_operators = vec![Operator::Add, Operator::Multiply];

    let mut sum: i64 = 0;
    for (test_number, operands) in equations {
        let operator_combinations = get_combinations(&allowed_operators, operands.len() - 1);
        for operators in operator_combinations {
            let result = parse_expression(&operands, operators);
            if result == test_number {
                sum += test_number;
                break;
            }
        }
    }

    sum
}

fn puzzle2(input: &str) -> i64 {
    let equations = parse(input);

    let allowed_operators = vec![Operator::Add, Operator::Multiply, Operator::Concat];

    let mut sum: i64 = 0;
    for (test_number, operands) in equations {
        let operator_combinations = get_combinations(&allowed_operators, operands.len() - 1);
        for operators in operator_combinations {
            let result = parse_expression(&operands, operators);
            if result == test_number {
                sum += test_number;
                break;
            }
        }
    }

    sum
}

fn get_combinations<T: Clone>(values: &Vec<T>, n: usize) -> Vec<Vec<T>> {
    (0..n)
        .map(|_| values.clone())
        .multi_cartesian_product()
        .collect_vec()
}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    let mut equations: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in input.lines() {
        let (test_number_str, operands_str) = line
            .split_once(": ")
            .expect("there to be a test value and operation numbers");
        let test_number: i64 = test_number_str.parse().expect("failed to parse str to i64");
        let mut operands: Vec<i64> = Vec::new();
        for operand_str in operands_str.split_whitespace() {
            let number = operand_str.parse().expect("");
            operands.push(number);
        }
        equations.push((test_number, operands));
    }
    equations
}

fn parse_expression(operands: &Vec<i64>, operators: Vec<Operator>) -> i64 {
    let mut previous_value = *operands.first().expect("there should be an operand");
    for (operand, operator) in operands.iter().skip(1).zip(operators) {
        match operator {
            Operator::Add => previous_value += operand,
            Operator::Multiply => previous_value *= operand,
            Operator::Concat => {
                previous_value = (previous_value.to_string() + &operand.to_string())
                    .parse()
                    .expect("failed to parse concatenated string to i64")
            }
        }
    }
    previous_value
}
