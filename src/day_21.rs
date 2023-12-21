use crate::geometry::Point;

use std::cmp;
use std::collections::HashMap;
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

pub fn solve_part_1(text: &String) -> () {
    let map = parse_map(text);

    let height = map.len() as i64;
    let width = map[0].len() as i64;
    let is_within_bounds = |p: Point| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height;
    let mut start = Point { x: 0, y: 0 };
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = Point {
                    x: x as i64,
                    y: y as i64,
                };
            }
        }
    }
    let mut positions: HashSet<Point> = HashSet::from([start]);
    for _ in 0..64 {
        let mut new_positions = HashSet::new();
        for p in positions {
            for d in [DIR_LEFT, DIR_UP, DIR_RIGHT, DIR_DOWN] {
                let np = p + d;
                if is_within_bounds(np) && map[np.y as usize][np.x as usize] != '#' {
                    new_positions.insert(np);
                }
            }
        }
        positions = new_positions;
    }

    let num_reachable = positions.len();

    println!("Tiles reachable in 64 steps: {num_reachable}");
    println!("Expected puzzle answer:      3782");
}

fn count_reachable_with_steps(
    map: &Vec<Vec<char>>,
    start: Point,
    num_steps: i64,
    count_even: bool,
) -> i64 {
    let height = map.len() as i64;
    let width = map[0].len() as i64;
    let is_within_bounds = |p: Point| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height;
    let mut visited: HashSet<Point> = HashSet::new();
    let mut even_visited: HashSet<Point> = HashSet::new();
    let mut positions: HashSet<Point> = HashSet::from([start]);
    let mut steps = 0;
    while positions.len() > 0 && steps <= num_steps {
        let mut new_positions = HashSet::new();
        for p in positions {
            visited.insert(p);
            if steps % 2 == 0 {
                even_visited.insert(p);
            }
            for d in [DIR_LEFT, DIR_UP, DIR_RIGHT, DIR_DOWN] {
                let np = p + d;
                if !visited.contains(&np)
                    && is_within_bounds(np)
                    && map[np.y as usize][np.x as usize] != '#'
                {
                    new_positions.insert(np);
                }
            }
        }
        positions = new_positions;
        steps += 1;
    }

    if count_even {
        return even_visited.len() as i64;
    } else {
        return (visited.len() - even_visited.len()) as i64;
    }
}

pub fn solve_part_2(text: &String) -> () {
    // This solution is a mess. Can be significantly cleaned up and improved wrt performance.
    // Basically:
    // - The input has a very special format: (frustratingly _not_ all shared with the example input)
    //   - The starting point is right in the center
    //   - The map is square, with odd side length
    //   - There is a free path from the center to the boundary, and all along the boundary
    //   - There is no "unexpectedly long" path created by #-characters to reach any point in the map from any point on the boundary
    // - All copies of the map that are reachable within somewhat less than 26501365 steps can be completely explored
    //   - The total step count of these copies is just the number of copies times the number of reachable endpoints within
    // - All remaining copies are along the boundary of the "rhombus" at 26501365 steps from the starting point
    //   - The copies are only entered from a few boundary points and with a certain number of remaining steps
    //     - We can again just multiply the number of each configuration times the number of reachable endpoints within
    //     - ...or in this code's less thought-through implementation, use a cache to avoid recomputing the endpoint count

    let map = parse_map(text);
    let height = map.len() as i64;
    let width = map[0].len() as i64;
    assert!(height == width);

    let mut start = Point { x: 0, y: 0 };
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = Point {
                    x: x as i64,
                    y: y as i64,
                };
            }
        }
    }

    assert!(start.x % 2 == 1);
    assert!(start.y % 2 == 1);
    let even_visited =
        count_reachable_with_steps(&map, start, width * width, /*count_even*/ true);
    let odd_visited =
        count_reachable_with_steps(&map, start, width * width, /*count_even*/ false);

    let num_steps = 26501365;
    // We can reach and fully flood any map tile index (nx,ny) where:
    // width * (|nx| + |ny| + 1) - 1 <= num_steps
    let nx_ny_sum: i64 = cmp::max(0, (num_steps + 1) / width - 1);
    let num_fully_visited_tiles = (nx_ny_sum + 1)
        * (nx_ny_sum + 1)
        * (match (num_steps + nx_ny_sum) % 2 {
            0 => even_visited,
            _ => odd_visited,
        })
        + nx_ny_sum
            * nx_ny_sum
            * (match (num_steps + nx_ny_sum) % 2 {
                1 => even_visited,
                _ => odd_visited,
            });

    // What remains is the contribution from not-necessarily-fully traversed tiles.
    // These tiles have a sum |nx|+|ny| greater than nx_ny_sum.
    // We start from those, and flood-fill our way from there.
    // Cache pairs of entry points and remaining steps to save compute.

    let mut nxny_visited: HashSet<Point> = HashSet::new();
    let scalar_sign = |a: i64| match a {
        0 => 0,
        _ => a / a.abs(),
    };
    let sign = |p: &Point| Point {
        x: scalar_sign(p.x),
        y: scalar_sign(p.y),
    };
    let multiply_vector = |d: Point, n: i64| Point {
        x: d.x * n,
        y: d.y * n,
    };
    let used_steps_1d = |a: i64| match a {
        0 => 0,
        _ => (width + 1) / 2 + (a.abs() - 1) * width,
    };
    let remaining_steps_from =
        |nn: Point, n_steps: i64| n_steps - used_steps_1d(nn.x) - used_steps_1d(nn.y);
    // Cache: Map starting point within tile + number of steps within tile to the number of tiles visitable.
    let mut tile_flood_cache: HashMap<(Point, i64), i64> = HashMap::new();
    let mut to_visit: VecDeque<Point> = VecDeque::new();
    for n2 in 0..nx_ny_sum + 1 {
        let n1 = nx_ny_sum + 1 - n2;
        to_visit.push_back(Point { x: n1, y: n2 });
        to_visit.push_back(Point { x: -n2, y: n1 });
        to_visit.push_back(Point { x: -n1, y: -n2 });
        to_visit.push_back(Point { x: n2, y: -n1 });
    }
    let mut num_partial_visited = 0;
    while let Some(nn) = to_visit.pop_front() {
        if nn.x.abs() + nn.y.abs() <= nx_ny_sum || nxny_visited.contains(&nn) {
            // Already visited.
            continue;
        }
        nxny_visited.insert(nn);
        // Given nx,ny, compute closest entry point:
        let nn_start = start - multiply_vector(sign(&nn), (width - 1) / 2);
        // Compute number of remaining steps:
        let remaining_steps = remaining_steps_from(nn, num_steps);
        let count_even = remaining_steps % 2 == 0;
        let num_new_visited: i64;
        match tile_flood_cache.get(&(nn_start, remaining_steps)) {
            Some(num) => {
                num_new_visited = *num;
            }
            None => {
                let num = count_reachable_with_steps(&map, nn_start, remaining_steps, count_even);
                tile_flood_cache.insert((nn_start, remaining_steps), num);
                num_new_visited = num;
            }
        };
        num_partial_visited += num_new_visited;
        for dir in [DIR_LEFT, DIR_RIGHT, DIR_UP, DIR_DOWN] {
            let nnn = nn + dir;
            if remaining_steps_from(nnn, num_steps) >= 0 {
                to_visit.push_back(nnn);
            }
        }
    }

    let num_visited = num_fully_visited_tiles + num_partial_visited;

    println!("Tiles reachable after many steps: {num_visited}");
    println!("Expected puzzle answer:           630661863455116");
}
