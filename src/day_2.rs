use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i32, space1},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
    Err, IResult,
};

use std::cmp;
use std::collections::HashMap;

// https://doc.rust-lang.org/reference/attributes/derive.html
// Needed to use the enum as a key type in HashMap.
#[derive(Eq, Hash, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn to_color(input: &str) -> Result<Color, Err<&str>> {
    match input {
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "blue" => Ok(Color::Blue),
        _ => Err(Err::Failure("boo")),
    }
}

// Example: "14 green" => (14, Color::Green)
fn parse_cube(input: &str) -> IResult<&str, (i32, Color)> {
    separated_pair(i32, space1, map_res(alpha1, to_color))(input)
}

// Example: "1 green, 2 blue; 5 red" => [(1, Color::Green), (2, Color::blue), (5, Color::Red)]
// This ignores the distinction between draws within and between sets of a game.
// The problem does not use this extra "substructure" anyway.
fn parse_game(input: &str) -> IResult<&str, Vec<(i32, Color)>> {
    separated_list1(alt((tag(", "), tag("; "))), parse_cube)(input)
}

// Example: "Game 10: 1 green, 2 blue; 5 red" => (10, [(1, Color::Green), (2, Color::blue), (5, Color::Red)])
fn parse_line(input: &str) -> (i32, Vec<(i32, Color)>) {
    match all_consuming(pair(delimited(tag("Game "), i32, tag(": ")), parse_game))(input) {
        Err(e) => {
            panic!("bad input: {}", e);
        }
        Ok((_, (id, draws))) => {
            return (id, draws);
        }
    }
}

pub fn solve_part_1(text: &String) -> () {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut id_sum = 0;
    for line in lines {
        let (id, draws) = parse_line(&line);
        let mut max_map: HashMap<Color, i32> = HashMap::new();
        for (num, color) in draws {
            max_map
                .entry(color)
                .and_modify(|v| *v = cmp::max(*v, num))
                .or_insert(num);
        }
        if max_map.get(&Color::Red).unwrap_or(&0) <= &12
            && max_map.get(&Color::Green).unwrap_or(&0) <= &13
            && max_map.get(&Color::Blue).unwrap_or(&0) <= &14
        {
            id_sum += id;
        }
    }

    println!("Sum of valid game IDs   {}", id_sum);
    println!("Expected puzzle answer: 2239");
}

pub fn solve_part_2(text: &String) -> () {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut power_sum = 0;
    for line in lines {
        let (_, draws) = parse_line(&line);
        let mut max_map: HashMap<Color, i32> = HashMap::new();
        for (num, color) in draws {
            max_map
                .entry(color)
                .and_modify(|v| *v = cmp::max(*v, num))
                .or_insert(num);
        }
        power_sum += max_map.get(&Color::Red).unwrap_or(&0)
            * max_map.get(&Color::Green).unwrap_or(&0)
            * max_map.get(&Color::Blue).unwrap_or(&0);
    }

    println!("Sum of valid game IDs   {}", power_sum);
    println!("Expected puzzle answer: 83435");
}
