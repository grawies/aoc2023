use crate::geometry::Point;

use std::collections::HashMap;

const DIR_LEFT: Point = Point { x: -1, y: 0 };
const DIR_RIGHT: Point = Point { x: 1, y: 0 };
const DIR_UP: Point = Point { x: 0, y: 1 };
const DIR_DOWN: Point = Point { x: 0, y: -1 };

// Turns a string of newline-separated rows into a 2D matrix.
// Reverses the y-axis, to convert from top-down text to "up is positive" y-axis.
fn parse_map(input: &str) -> Vec<Vec<char>> {
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in input.split("\n") {
        rows.push(line.chars().collect::<Vec<char>>());
    }
    rows.reverse();
    return rows;
}

fn tilt(map: &mut Vec<Vec<char>>, dir: Point) -> () {
    let height = map.len() as i64;
    let width = map[0].len() as i64;
    // Start at closest row in tilt direction.
    let mut start = Point { x: 0, y: 0 };
    let bounds = Point {
        x: width,
        y: height,
    };
    let orthogonal_dir = Point { x: dir.y, y: dir.x };
    if dir.x + dir.y > 0 {
        // Reverse direction.
        start = Point {
            x: bounds.x - 1,
            y: bounds.y - 1,
        };
    }
    let is_within_bounds = |p: Point| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height;
    // Loop over all point at fixed dir-distance from edge, moving each step-by-step until we hit another rock or the edge.
    while is_within_bounds(start) {
        let mut p = start;
        while is_within_bounds(p) {
            if map[p.y as usize][p.x as usize] != 'O' {
                p = p - dir;
                continue;
            }
            let mut q = p.clone();
            while is_within_bounds(q) {
                let nq = q + dir;
                if !is_within_bounds(nq) || map[nq.y as usize][nq.x as usize] != '.' {
                    break;
                }
                // There is an 'O' at q and a '.' in direction |dir|.
                map[nq.y as usize][nq.x as usize] = 'O';
                map[q.y as usize][q.x as usize] = '.';
                q = nq;
            }
            p = p - dir;
        }

        start = start - orthogonal_dir;
    }
}

fn compute_weight(map: Vec<Vec<char>>) -> usize {
    let mut total_load = 0;
    for (i, row) in map.iter().enumerate() {
        for (_, c) in row.iter().enumerate() {
            if *c == 'O' {
                total_load += i + 1;
            }
        }
    }
    return total_load;
}

pub fn solve_part_1(text: &String) -> () {
    let mut map = parse_map(text);
    tilt(&mut map, DIR_UP);

    let total_load = compute_weight(map);

    println!("Total rock load after tilt North: {total_load}");
    println!("Expected puzzle answer:           110565");
}

pub fn solve_part_2(text: &String) -> () {
    let mut map = parse_map(text);
    let mut cache: HashMap<Vec<Vec<char>>, i64> = HashMap::new();
    let mut i = 0;
    let cycle_count = 1_000_000_000;
    while i < cycle_count {
        tilt(&mut map, DIR_UP);
        tilt(&mut map, DIR_LEFT);
        tilt(&mut map, DIR_DOWN);
        tilt(&mut map, DIR_RIGHT);
        if let Some(j) = cache.get(&map) {
            // We have detected a cycle, and can fast forward an integer number of those.
            let short_circuit_cycle_count = (cycle_count - i) / (i - j);
            i += short_circuit_cycle_count * (i - j);
        }
        cache.insert(map.clone(), i);
        i += 1;
    }

    let total_load = compute_weight(map);

    println!("Total rock load after spin cycling: {total_load}");
    println!("Expected puzzle answer:             89845");
}
