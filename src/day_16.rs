use crate::geometry::Point;

use std::cmp;

use std::collections::HashSet;
use std::collections::VecDeque;

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

fn dot(p: Point, q: Point) -> i64 {
    return p.x * q.x + p.y * q.y;
}

// Compute the next position(s) and direction(s) of a beam entering tile |p| with direction |d| and the tile has symbol |c|.
fn compute_beam_continuation((position, dir): (Point, Point), c: char) -> Vec<(Point, Point)> {
    match c {
        '.' => {
            return vec![(position + dir, dir)];
        }
        '|' | '-' => {
            // |c| is a splitter.
            let mirror_dir = match c {
                '|' => DIR_DOWN,
                '-' => DIR_RIGHT,
                _ => panic!("impossible"),
            };
            if dot(dir, mirror_dir) != 0 {
                // Beam and mirror align - no effect.
                return vec![(position + dir, dir)];
            }
            // The new directions are +/- transpose(dir).
            let dp = Point { x: dir.y, y: dir.x };
            return vec![(position + dp, dp), (position - dp, -dp)];
        }
        '/' => {
            // Mirroring in '/' is just transposing.
            let dp = Point { x: dir.y, y: dir.x };
            return vec![(position + dp, dp)];
        }
        '\\' => {
            // Mirroring in '\\' is negated transposing.
            let dp = -Point { x: dir.y, y: dir.x };
            return vec![(position + dp, dp)];
        }
        _ => {
            panic!("bad mirror symbol");
        }
    }
}

fn compute_num_energized(map: &Vec<Vec<char>>, initial_beam_head: (Point, Point)) -> usize {
    let height = map.len() as i64;
    let width = map[0].len() as i64;
    let is_within_bounds = |p: Point| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height;

    // BFS from initial beam until all beam heads have left the map or entered a cycle.
    let mut beam_heads: VecDeque<(Point, Point)> = VecDeque::from([initial_beam_head; 1]);
    let mut visited: HashSet<(Point, Point)> = HashSet::new();
    while let Some((p, d)) = beam_heads.pop_front() {
        visited.insert((p, d));
        let new_heads = compute_beam_continuation((p, d), map[p.y as usize][p.x as usize]);
        for head in new_heads {
            if is_within_bounds(head.0) && !visited.contains(&head) {
                beam_heads.push_back(head);
            }
        }
    }

    let mut energized_tiles = visited
        .iter()
        .map(|(position, _)| position)
        .collect::<Vec<&Point>>();
    energized_tiles.sort_unstable_by(|p, q| {
        if p.x != q.x {
            p.x.cmp(&q.x)
        } else {
            p.y.cmp(&q.y)
        }
    });
    energized_tiles.dedup();
    return energized_tiles.len();
}

pub fn solve_part_1(text: &String) -> () {
    let map = parse_map(text);

    // We start with a single beam in the upper left corner of the map, heading rightward.
    let top_left_map_corner = Point {
        x: 0,
        y: map.len() as i64 - 1,
    };
    let initial_beam_head = (top_left_map_corner, DIR_RIGHT);
    let num_energized_tiles = compute_num_energized(&map, initial_beam_head);

    println!("Number of energized tiles: {num_energized_tiles}");
    println!("Expected puzzle answer:    7477");
}

pub fn solve_part_2(text: &String) -> () {
    let map = parse_map(text);
    let height = map.len() as i64;
    let width = map[0].len() as i64;

    // Loop over all incoming beam positions and directions.
    let mut max_num_energized_tiles = 0;
    for row in 0..height {
        let head = (Point { x: 0, y: row }, DIR_RIGHT);
        max_num_energized_tiles =
            cmp::max(max_num_energized_tiles, compute_num_energized(&map, head));
        let head = (
            Point {
                x: width - 1,
                y: row,
            },
            DIR_LEFT,
        );
        max_num_energized_tiles =
            cmp::max(max_num_energized_tiles, compute_num_energized(&map, head));
    }
    for col in 0..width {
        let head = (Point { x: col, y: 0 }, DIR_UP);
        max_num_energized_tiles =
            cmp::max(max_num_energized_tiles, compute_num_energized(&map, head));
        let head = (
            Point {
                x: col,
                y: height - 1,
            },
            DIR_DOWN,
        );
        max_num_energized_tiles =
            cmp::max(max_num_energized_tiles, compute_num_energized(&map, head));
    }

    println!("Maximum number of energized tiles: {max_num_energized_tiles}");
    println!("Expected puzzle answer:            7853");
}
