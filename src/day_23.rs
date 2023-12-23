use crate::geometry::Point;

use std::cmp;

use std::collections::HashMap;
use std::collections::HashSet;

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

pub fn solve_part_1(text: &String) -> () {
    let map = parse_map(text);
    let height = map.len() as i64;
    let width = map[0].len() as i64;
    let is_within_bounds = |p: Point| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height;

    let mut start = Point {
        x: -1,
        y: height - 1,
    };
    let mut target = Point { x: -1, y: 0 };
    for x in 0..width {
        if map[height as usize - 1][x as usize] == '.' {
            start.x = x;
        }
        if map[0][x as usize] == '.' {
            target.x = x;
        }
    }
    assert!(start.x != -1);
    assert!(target.x != -1);

    let allowed_dirs = HashMap::from([
        ('.', vec![DIR_LEFT, DIR_UP, DIR_RIGHT, DIR_DOWN]),
        ('>', vec![DIR_RIGHT]),
        ('v', vec![DIR_DOWN]),
        ('^', vec![DIR_UP]),
        ('<', vec![DIR_LEFT]),
    ]);

    // DFS from the start, pruning any branch that reaches a point already visited by a longer path.
    let mut max_to_reach: HashMap<Point, i64> = HashMap::new();
    let mut stack: Vec<(Point, Point, i64)> = Vec::from([(start, start, 0)]);
    while let Some((pos, src, steps)) = stack.pop() {
        if max_to_reach.get(&pos).unwrap_or(&-1) >= &steps {
            // Already visited by a longer path - prune this branch.
            continue;
        }
        max_to_reach.insert(pos, steps);
        for d in allowed_dirs
            .get(&map[pos.y as usize][pos.x as usize])
            .unwrap()
        {
            let npos = pos + d;
            if npos != src && is_within_bounds(npos) && map[npos.y as usize][npos.x as usize] != '#'
            {
                stack.push((npos, pos, steps + 1));
            }
        }
    }

    let max_steps_to_target = max_to_reach.get(&target).unwrap();

    println!("Longest hike in steps:  {max_steps_to_target}");
    println!("Expected puzzle answer: 2094");
}

pub fn solve_part_2(text: &String) -> () {
    let map = parse_map(text);
    let height = map.len();
    let width = map[0].len();
    let is_within_bounds =
        |p: Point| p.x >= 0 && p.x < width as i64 && p.y >= 0 && p.y < height as i64;

    let mut start = Point {
        x: -1,
        y: height as i64 - 1,
    };
    let mut target = Point { x: -1, y: 0 };
    for x in 0..width {
        if map[height - 1][x] == '.' {
            start.x = x as i64;
        }
        if map[0][x] == '.' {
            target.x = x as i64;
        }
    }
    assert!(start.x != -1);
    assert!(target.x != -1);

    // I suspect this is an NP-complete problem (longest path), unless there is some special input structure I am missing.
    // Traversing the whole graph step-by-step is too slow (I tried).
    // Instead, first reduce the graph from a 2D grid to a graph of junctions (incl. start/target node), with edges weighted by the distance of segments between junctions.
    // Then do a DFS on this weighted graph.
    let mut edges: HashMap<Point, Vec<(Point, i64)>> = HashMap::new();
    let mut junctions_to_search: Vec<Point> = vec![start];
    let mut searched: HashSet<Point> = HashSet::from([start]);
    let nbrs = |p| {
        [DIR_LEFT, DIR_UP, DIR_RIGHT, DIR_DOWN]
            .iter()
            .filter(|d| {
                let np = p + **d;
                is_within_bounds(np) && map[np.y as usize][np.x as usize] != '#'
            })
            .copied()
            .collect::<Vec<Point>>()
    };
    while let Some(pos) = junctions_to_search.pop() {
        for mut dir in nbrs(pos) {
            let mut curr = pos;
            let mut steps = 0;
            loop {
                curr = curr + dir;
                steps += 1;
                let neighbor_dirs = nbrs(curr)
                    .into_iter()
                    .filter(|d| *d != -dir)
                    .collect::<Vec<Point>>();
                if neighbor_dirs.len() != 1 {
                    // We are at a new junction / leaf node.
                    // Store the edge weight and enqueue node.
                    edges
                        .entry(pos)
                        .and_modify(|v| v.push((curr, steps)))
                        .or_insert(vec![(curr, steps)]);
                    if !searched.contains(&curr) {
                        searched.insert(curr);
                        junctions_to_search.push(curr);
                    }
                    break;
                }
                dir = neighbor_dirs[0];
            }
        }
    }

    let mut max_to_reach: HashMap<Point, i64> = HashMap::new();
    let mut stack: Vec<(Point, i64, HashSet<Point>)> = vec![(start, 0, HashSet::new())];
    while let Some((pos, steps, mut visited)) = stack.pop() {
        visited.insert(pos);
        max_to_reach
            .entry(pos)
            .and_modify(|s| *s = cmp::max(steps, *s))
            .or_insert(steps);
        for (npos, cost) in edges.get(&pos).unwrap() {
            if !visited.contains(&npos) {
                stack.push((*npos, steps + cost, visited.clone()));
            }
        }
    }

    let max_steps_to_target = max_to_reach.get(&target).unwrap();

    println!("Longest hike in steps:  {max_steps_to_target}");
    println!("Expected puzzle answer: 6442");
}
