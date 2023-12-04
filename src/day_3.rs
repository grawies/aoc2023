use std::collections::HashMap;
use std::collections::HashSet;

struct Part {
    number: i64,
    adj_ps: Vec<(i32, i32)>,
}

fn parse_input(text: &String) -> (Vec<Part>, HashSet<(i32, i32)>, HashSet<(i32, i32)>) {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    // Contains a Part for each string of digits in the input.
    let mut parts: Vec<Part> = Vec::new();
    // Contains all positions of symbols.
    let mut symbol_ps: HashSet<(i32, i32)> = HashSet::new();
    // Contains all positions of gear symbols.
    let mut gear_symbol_ps: HashSet<(i32, i32)> = HashSet::new();

    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    for (y_idx, line) in lines.iter().enumerate() {
        let mut current_part: Option<Part> = None;
        let width = line.len();
        for (x_idx, c) in line.chars().enumerate() {
            let x = x_idx as i32;
            let y = y_idx as i32;
            let c_is_digit = digits.contains(&c);
            if c_is_digit {
                let d = c.to_digit(10).expect("unexpected non-digit digit char") as i64;
                if let Some(ref mut part) = current_part {
                    part.number = 10 * part.number + d;
                    part.adj_ps.push((x, y - 1));
                    part.adj_ps.push((x, y + 1));
                } else {
                    let mut part = Part {
                        number: d as i64,
                        adj_ps: Vec::new(),
                    };
                    part.adj_ps.push((x - 1, y - 1));
                    part.adj_ps.push((x - 1, y));
                    part.adj_ps.push((x - 1, y + 1));
                    part.adj_ps.push((x, y - 1));
                    part.adj_ps.push((x, y + 1));
                    current_part = Some(part);
                }
            }
            if !c_is_digit || x_idx == width - 1 {
                // Check if a part number ends here.
                match current_part {
                    Some(mut part) => {
                        part.adj_ps.push((x, y - 1));
                        part.adj_ps.push((x, y));
                        part.adj_ps.push((x, y + 1));
                        parts.push(part);
                        current_part = None;
                    }
                    _ => {}
                }
            }
            if !c_is_digit && c != '.' {
                symbol_ps.insert((x, y));
            }
            if c == '*' {
                gear_symbol_ps.insert((x, y));
            }
        }
    }
    return (parts, symbol_ps, gear_symbol_ps);
}

pub fn solve_part_1(text: &String) -> () {
    let (parts, symbol_ps, _) = parse_input(text);

    let mut part_number_sum: i64 = 0;
    for part in parts {
        let mut is_engine_part = false;
        for p in part.adj_ps {
            if symbol_ps.contains(&p) {
                is_engine_part = true;
                break;
            }
        }
        if is_engine_part {
            part_number_sum += part.number;
        }
    }

    println!("Engine part number sum: {}", part_number_sum);
    println!("Expected puzzle answer: 550064");
}

pub fn solve_part_2(text: &String) -> () {
    let (parts, _, gear_symbol_ps) = parse_input(text);

    // Map gear symbol positions to their adjacent part numbers.
    let mut gear_adj_nums: HashMap<(i32, i32), Vec<i64>> = HashMap::new();
    for part in parts {
        for p in part.adj_ps {
            if gear_symbol_ps.contains(&p) {
                gear_adj_nums
                    .entry(p)
                    .and_modify(|c| c.push(part.number))
                    .or_insert(Vec::from([part.number]));
            }
        }
    }

    let mut gear_ratio_sum: i64 = 0;
    for ns in gear_adj_nums.values() {
        if ns.len() != 2 {
            continue;
        }
        gear_ratio_sum += ns[0] * ns[1];
    }

    println!("Gear ratio sum:         {}", gear_ratio_sum);
    println!("Expected puzzle answer: 85010461");
}
