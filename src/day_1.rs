use std::collections::HashMap;

pub fn solve_part_1(text: &String) -> () {
    let calibration_sum: i32 = solve_either(text, /*include_words=*/ false);
    println!("Calibration value sum:  {}", calibration_sum);
    println!("Expected puzzle answer: 53194");
}

pub fn solve_part_2(text: &String) -> () {
    let calibration_sum: i32 = solve_either(text, /*include_words=*/ true);
    println!("Calibration value sum:  {}", calibration_sum);
    println!("Expected puzzle answer: 54249");
}

fn solve_either(text: &String, include_words: bool) -> i32 {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut str_to_digit: HashMap<&str, i32> = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    if include_words {
        for (word, value) in [
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ] {
            str_to_digit.insert(word, value);
        }
    }
    let mut calibration_values: Vec<i32> = Vec::new();
    for line in lines {
        let mut first: Option<i32> = None;
        let mut last: Option<i32> = None;
        for (i, _) in line.chars().enumerate() {
            for (s, v) in &str_to_digit {
                if line[i..].starts_with(s) {
                    match first {
                        Some(_) => {}
                        None => {
                            first = Some(*v);
                        }
                    }
                    last = Some(*v);
                }
            }
        }
        assert!(first.is_some());
        assert!(last.is_some());
        match (first, last) {
            (Some(x), Some(y)) => {
                calibration_values.push(10 * x + y);
            }
            _ => {}
        }
    }
    let calibration_sum: i32 = calibration_values.iter().sum();
    return calibration_sum;
}
