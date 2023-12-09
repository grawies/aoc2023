use nom::{
    bytes::complete::tag, character::complete::i64, combinator::all_consuming, error::Error,
    multi::separated_list1,
};

fn parse_list(input: &str) -> Vec<i64> {
    match all_consuming(separated_list1(tag(" "), i64::<_, Error<_>>))(input) {
        Err(e) => {
            panic!("bad input: {}", e);
        }
        Ok((_, numbers)) => {
            return numbers;
        }
    }
}

fn extrapolate_poly(numbers: &Vec<i64>, extrapolate_ending: bool) -> i64 {
    let mut values: Vec<i64> = numbers.clone();
    let mut stack: Vec<Vec<i64>> = Vec::new();
    stack.push(values.clone());
    while !values.iter().all(|x| *x == 0) {
        let mut new_values: Vec<i64> = Vec::new();
        for i in 0..values.len() - 1 {
            new_values.push(values[i + 1] - values[i]);
        }
        stack.push(new_values.clone());
        values = new_values;
    }
    let mut extrapolated_value: i64 = 0;
    match extrapolate_ending {
        true => {
            for &x in stack.iter().map(|v| v.last().unwrap()) {
                extrapolated_value += x;
            }
            return extrapolated_value;
        }
        false => {
            for &x in stack.iter().rev().map(|v| v.first().unwrap()) {
                extrapolated_value = x - extrapolated_value;
            }
            return extrapolated_value;
        }
    }
}

pub fn solve_part_1(text: &String) -> () {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut sum = 0;
    for line in lines {
        let numbers = parse_list(&line);
        sum += extrapolate_poly(&numbers, /*extrapolate_ending*/ true);
    }
    println!("Sum of predictions:     {sum}");
    println!("Expected puzzle answer: 2175229206");
}

pub fn solve_part_2(text: &String) -> () {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut sum = 0;
    for line in lines {
        let numbers = parse_list(&line);
        sum += extrapolate_poly(&numbers, /*extrapolate_ending*/ false);
    }
    println!("Sum of predictions:     {sum}");
    println!("Expected puzzle answer: 942");
}
