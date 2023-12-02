mod day_1;

use std::env;

struct Config {
    days: Vec<usize>,
    part1: bool,
    part2: bool,
    input_file: String,
}

// Somewhere out there are at least five well-established libraries that provide flag parsing.
fn parse_flags() -> Config {
    let mut config = Config {
        days: (1..25).collect(),
        part1: false,
        part2: false,
        input_file: "input.txt".to_string(),
    };
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        if arg.starts_with("--days=") {
            let arg_value = &arg[7..].to_string();
            let days = arg_value
                .split(',')
                .map(|s| {
                    s.parse::<usize>()
                        .expect("--days must be a comma-separated list of integers")
                })
                .filter(|i| *i >= 1 && *i <= 25);
            config.days = days.collect();
        } else if arg.starts_with("--part") {
            let arg_value = &arg[6..].to_string();
            let part = arg_value
                .parse::<i32>()
                .expect("--partN must specify an integer N");
            match part {
                1 => {
                    config.part1 = true;
                }
                2 => {
                    config.part2 = true;
                }
                _ => {
                    panic!("unexpected format of flag --part: {}", arg);
                }
            }
        } else if arg.starts_with("--input_file=") {
            config.input_file = arg[13..].to_string();
        } else {
            panic!("Unrecognized command line flag: {}", arg);
        }
    }
    // If neither --part1 nor --part2 were specified, run both parts.
    if !config.part1 && !config.part2 {
        config.part1 = true;
        config.part2 = true;
    }
    return config;
}

type Solution = fn(&String) -> ();

fn solutions() -> [Vec<Solution>; 25] {
    return [
        vec![day_1::solve_part_1, day_1::solve_part_2],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
}

/* Usage:
 * aoc2023 [--days=<n,m,..>] [--part1] [--part2] [--input_file=<path>]
 * Runs all solutions on the correponsing input.txt files in data/.
 *
 * If --days is specified, executes just the given days.
 * If --partN is specified, executes just that part of the problem.
 * If flag --input_file is specified, replaces the default input.txt in data/.
*/
fn main() {
    let config = parse_flags();
    let solutions = solutions();
    for day in config.days {
        let solution = &solutions[day - 1];
        if solution.len() == 0 {
            continue;
        }
        println!(" --- Day {}", day);
        if config.part1 {
            println!(" ------ Part One");
            solution[0](&config.input_file);
        }
        if config.part2 && solution.len() >= 2 {
            println!(" ------ Part Two");
            solution[1](&config.input_file);
        }
    }
}
