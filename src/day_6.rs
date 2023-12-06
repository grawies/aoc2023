use nom::{
    error::Error,
    bytes::complete::tag,
    character::complete::{i64, space1},
    combinator::all_consuming,
    multi::many1,
    sequence::preceded,
};

use std::iter::zip;

fn parse_row(input: &str, tag_str: &str ) -> Vec<i64> {
    match all_consuming(preceded(tag(tag_str), many1(preceded(space1::<_, Error<_>>, i64))))(input) {
        Err(e) => {
            panic!("bad input: {}", e);
        }
        Ok((_, times)) => {
            return times;
        }
    }
}

pub fn solve_part_1(text: &String) -> () {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let times = parse_row(&lines[0], "Time:");
    let distances = parse_row(&lines[1], "Distance:");
    let mut time_product: i64 = 1;
    for (time, distance) in zip(times.iter(), distances.iter()) {
        for t in 1..*time {
            if t * (*time - t) > *distance {
                // This is the lowest time that beats the record.
                time_product *= *time - 2 * t + 1;
                break;
            }
        }
    }

    println!("Product of number of winning times: {}", time_product);
    println!("Expected puzzle answer:             633080");
}

pub fn solve_part_2(text: &String) -> () {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let times = parse_row(&lines[0], "Time:");
    let time = (times.into_iter().map(|x| x.to_string())).collect::<Vec<String>>().join("").parse::<i64>().expect("de-kerning failed");
    let distances = parse_row(&lines[1], "Distance:");
    let distance: i64 = (distances.into_iter().map(|x| x.to_string())).collect::<Vec<String>>().join("").parse::<i64>().expect("de-kerning failed");

    let mut num_winning_times: i64 = 0;
        for t in 1..time {
            if t * (time - t) > distance {
                // This is the lowest time that beats the record.
                num_winning_times = time - 2 * t + 1;
                break;
            }
    }

    println!("Number of winning times: {}", num_winning_times);
    println!("Expected puzzle answer:  20048741");
}

