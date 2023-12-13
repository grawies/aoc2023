use nom::{
    bytes::complete::tag, character::complete::i64, combinator::all_consuming, error::Error,
    multi::separated_list1,
};

use std::cmp;
use std::collections::HashMap;

// Reads a comma-separated list of integers into a vector.
fn parse_list(input: &str) -> Vec<i64> {
    match all_consuming(separated_list1(tag(","), i64::<_, Error<_>>))(input) {
        Err(e) => {
            panic!("bad input: {}", e);
        }
        Ok((_, numbers)) => {
            return numbers;
        }
    }
}

fn count_placements(s: &str, v: Vec<i64>, cache: &mut HashMap<(String, Vec<i64>), i64>) -> i64 {
    if v.len() == 0 {
        if s.find('#').is_none() {
            return 1;
        } else {
            return 0;
        }
    }
    if s.len() == 0 {
        return 0;
    }
    let mut sum = 0;
    let min_len = v.iter().sum::<i64>() as usize + v.len() - 1;
    // We cannot start after a '#', need to match all.
    // Also don't even try starting if all of v doesn't fit in the string.
    let max_start_index = cmp::min(
        s.chars().position(|c| c == '#').unwrap_or_else(|| s.len()),
        s.len() - min_len,
    );
    for i in 0..max_start_index + 1 {
        // Try to place the next entry in v at index i.
        let j = i + v[0] as usize;
        if s[i..j].find('.').is_none() && (s.len() == j || s.chars().nth(j).unwrap() != '#') {
            let v_remainder = v[1..].iter().cloned().collect::<Vec<i64>>();
            if s.len() == j {
                sum += count_placements("", v_remainder, cache);
            } else {
                let str_remainder = &s[j + 1..];
                let num_arrangements: i64;
                match cache.get(&(str_remainder.to_string(), v_remainder.clone())) {
                    Some(n) => {
                        num_arrangements = *n;
                    }
                    None => {
                        num_arrangements =
                            count_placements(str_remainder, v_remainder.clone(), cache);
                        cache.insert(
                            (str_remainder.to_string(), v_remainder.clone()),
                            num_arrangements,
                        );
                    }
                }
                sum += num_arrangements;
            }
        }
    }
    return sum;
}

fn count_arrangements(text: &String, row_multiplier: usize) -> i64 {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut cache: HashMap<(String, Vec<i64>), i64> = HashMap::new();
    let mut sum = 0;
    for line in lines {
        let space_index = line.find(' ').unwrap();
        let input_string = &line[..space_index];
        let s = std::iter::repeat(input_string.to_string())
            .take(row_multiplier)
            .collect::<Vec<String>>()
            .join("?");
        let input_vector = parse_list(&line[space_index + 1..]);
        let v = input_vector
            .iter()
            .cycle()
            .take(input_vector.len() * row_multiplier)
            .cloned()
            .collect::<Vec<i64>>();
        let num_arrangements = count_placements(&s, v.clone(), &mut cache);
        sum += num_arrangements;
    }
    return sum;
}

pub fn solve_part_1(text: &String) -> () {
    let answer = count_arrangements(text, /*row_multiplier*/ 1);

    println!("Number of solutions:    {answer}");
    println!("Expected puzzle answer: 7236");
}

pub fn solve_part_2(text: &String) -> () {
    let answer = count_arrangements(text, /*row_multiplier*/ 5);

    println!("Number of solutions:    {answer}");
    println!("Expected puzzle answer: 11607695322318");
}
