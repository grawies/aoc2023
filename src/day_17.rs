use crate::geometry::Point;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

const DIR_LEFT: Point = Point { x: -1, y: 0 };
const DIR_RIGHT: Point = Point { x: 1, y: 0 };
const DIR_UP: Point = Point { x: 0, y: 1 };
const DIR_DOWN: Point = Point { x: 0, y: -1 };

// Turns a string of newline-separated rows into a 2D matrix.
// Reverses the y-axis, to convert from top-down text to "up is positive" y-axis.
fn parse_map(input: &str) -> Vec<Vec<i64>> {
    let mut rows: Vec<Vec<i64>> = Vec::new();
    for line in input.split("\n") {
        rows.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>(),
        );
    }
    rows.reverse();
    return rows;
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct MoveState {
    pos: Point,
    dir: Point,
    straight_counter: i64,
}

fn nbrs(state: &MoveState, map: &Vec<Vec<i64>>, use_ultra_crucible: bool) -> Vec<MoveState> {
    let height = map.len() as i64;
    let width = map[0].len() as i64;
    let is_within_bounds = |p: Point| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height;
    let mut ns: Vec<MoveState> = Vec::new();
    // We may move in 4 directions.
    for new_dir in [DIR_LEFT, DIR_RIGHT, DIR_UP, DIR_DOWN] {
        // Don't revisit the position we came from, heat loss can only increase.
        if new_dir == -state.dir {
            continue;
        }
        // Crucible conditions on how many times we may move straight ahead.
        let new_straight_counter = match new_dir == state.dir {
            true => state.straight_counter + 1,
            false => 1,
        };
        if use_ultra_crucible {
            // Ultra crucibles must move at least 4 tiles before turning.
            if state.straight_counter > 0 && state.straight_counter < 4 && new_dir != state.dir {
                continue;
            }
            // Ultra crucibles cannot move more than 10 tiles in a straight line.
            if new_straight_counter > 10 {
                continue;
            }
        } else {
            // Normal crucibles cannot move more than 3 tiles in a straight line.
            if new_straight_counter > 3 {
                continue;
            }
        }
        let new_pos = state.pos + new_dir;
        // We cannot leave the map.
        if !is_within_bounds(new_pos) {
            continue;
        }
        let new_state = MoveState {
            pos: new_pos,
            dir: new_dir,
            straight_counter: new_straight_counter,
        };
        ns.push(new_state);
    }
    return ns;
}

#[derive(Eq, PartialEq)]
struct HeapState {
    heat_loss: i64,
    move_state: MoveState,
}

impl Ord for HeapState {
    fn cmp(&self, other: &Self) -> Ordering {
        // The lower the heat loss, the greater.
        // Secondary comparison on MoveState for consistency with Ord.
        // See: https://doc.rust-lang.org/std/collections/binary_heap/index.html
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| self.move_state.cmp(&other.move_state))
    }
}

impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn minimum_heat_loss_path(map: &Vec<Vec<i64>>, use_ultra_crucible: bool) -> i64 {
    let mut queue: BinaryHeap<HeapState> = BinaryHeap::new();
    let mut visited: HashSet<MoveState> = HashSet::new();
    let mut min_heat_loss: Option<i64> = None;
    let start_pos = Point {
        x: 0,
        y: map.len() as i64 - 1,
    };
    let target_pos = Point {
        x: map[0].len() as i64 - 1,
        y: 0,
    };
    queue.push(HeapState {
        heat_loss: 0,
        move_state: MoveState {
            pos: start_pos,
            dir: DIR_RIGHT,
            straight_counter: 0,
        },
    });
    while let Some(heap_state) = queue.pop() {
        if visited.contains(&heap_state.move_state) {
            continue;
        }
        if heap_state.move_state.pos == target_pos {
            min_heat_loss = Some(heap_state.heat_loss);
            break;
        }
        visited.insert(heap_state.move_state);
        // Once the crucible may turn, it is never better to enter a state later (=>at higher heat loss) with higher straight-movement counter (=> fewer future options for directions).
        // Hence we count all such state as visited together with the current one.
        let min_tiles_before_turning = match use_ultra_crucible {
            true => 4,
            false => 0,
        };
        if heap_state.move_state.straight_counter >= min_tiles_before_turning {
            for c in heap_state.move_state.straight_counter..11 {
                let visited_state = MoveState {
                    pos: heap_state.move_state.pos,
                    dir: heap_state.move_state.dir,
                    straight_counter: c,
                };
                visited.insert(visited_state);
            }
        }
        for neighbor in nbrs(&heap_state.move_state, &map, use_ultra_crucible) {
            if visited.contains(&neighbor) {
                continue;
            }
            let heat_loss =
                heap_state.heat_loss + map[neighbor.pos.y as usize][neighbor.pos.x as usize];
            queue.push(HeapState {
                heat_loss: heat_loss,
                move_state: neighbor,
            });
        }
    }

    return min_heat_loss.unwrap();
}

pub fn solve_part_1(text: &String) -> () {
    let map = parse_map(text);
    let min_heat_loss = minimum_heat_loss_path(&map, /*use_ultra_crucible*/ false);

    println!("Minimum heat loss:      {min_heat_loss}");
    println!("Expected puzzle answer: 1044");
}

pub fn solve_part_2(text: &String) -> () {
    let map = parse_map(text);
    let min_heat_loss = minimum_heat_loss_path(&map, /*use_ultra_crucible*/ true);

    println!("Minimum heat loss:      {min_heat_loss}");
    println!("Expected puzzle answer: 1227");
}
