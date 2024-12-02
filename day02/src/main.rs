use shared::load_file_lines;

fn main() {
    let result1 = puzzle1();
    println!("Puzzle 1: {}", result1);

    let result2 = puzzle2();
    println!("Puzzle 2: {}", result2);
}

fn puzzle1() -> i32 {
    let lines = load_file_lines("./input.txt");
    let reports = get_reports(lines);

    let mut count_safe = 0;
    for report in reports {
        let gradient = get_gradient(report.clone());
        let is_safe = check_safety(gradient.clone());
        if is_safe {
            count_safe += 1;
        }
    }

    count_safe
}

fn puzzle2() -> i32 {
    let lines = load_file_lines("./input.txt");
    let reports = get_reports(lines);

    let mut count_safe = 0;
    for report in reports {
        let gradient = get_gradient(report.clone());
        let is_safe = check_safety(gradient.clone());
        if is_safe {
            count_safe += 1;
        } else {
            // mutate the report
            for idx in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(idx);
                if check_safety(get_gradient(new_report)) {
                    count_safe += 1;
                    break;
                }
            }
        }
    }

    count_safe
}

fn check_safety(gradients: Vec<i32>) -> bool {
    let is_monotonous = check_monotony(gradients.clone());

    let is_withing_bounds = gradients.iter().all(|&v| v.abs() >= 1 && v.abs() <= 3);

    is_monotonous && is_withing_bounds
}

fn check_monotony(gradients: Vec<i32>) -> bool {
    if gradients.iter().map(|&v| v < 0).all(|d| d == true) {
        true
    } else if gradients.iter().map(|&v| v > 0).all(|d| d == true) {
        true
    } else {
        false
    }
}

fn get_gradient(report: Vec<i32>) -> Vec<i32> {
    let mut gradient: Vec<i32> = Vec::new();
    for idx in 1..report.len() {
        let diff = report[idx - 1] - report[idx];
        gradient.push(diff);
    }
    gradient
}

fn get_reports(lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let mut report: Vec<i32> = Vec::new();
        for number_str in line.split_whitespace() {
            let number: i32 = number_str.parse().unwrap(); // can panic
            report.push(number);
        }
        reports.push(report);
    }
    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_negative_gradient() {
        let gradient = get_gradient([1, 2, 3].to_vec());
        assert_eq!(gradient, [-1, -1].to_vec())
    }

    #[test]
    fn test_get_positive_gradient() {
        let gradient = get_gradient([3, 2, 1].to_vec());
        assert_eq!(gradient, [1, 1].to_vec())
    }

    #[test]
    fn test_get_flat_gradient() {
        let gradient = get_gradient([1, 1, 1].to_vec());
        assert_eq!(gradient, [0, 0].to_vec())
    }

    #[test]
    fn test_get_gradient_from_negative() {
        let gradient = get_gradient([-1, -2, -3].to_vec());
        assert_eq!(gradient, [1, 1].to_vec())
    }

    #[test]
    fn test_check_monotony_negative() {
        let monotony = check_monotony([-1, -99, -3].to_vec());
        assert_eq!(monotony, true)
    }

    #[test]
    fn test_check_monotony_positive() {
        let monotony = check_monotony([1, 99, 3].to_vec());
        assert_eq!(monotony, true)
    }

    #[test]
    fn test_check_monotony_false() {
        let monotony = check_monotony([1, -1, 3].to_vec());
        assert_eq!(monotony, false)
    }

    #[test]
    fn test_check_monotony_zero() {
        let monotony = check_monotony([1, 0, 3].to_vec());
        assert_eq!(monotony, false)
    }
}
