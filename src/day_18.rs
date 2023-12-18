use crate::geometry::Point;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, hex_digit1, i64, line_ending, space1},
    combinator::all_consuming,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
};

const DIR_LEFT: Point = Point { x: -1, y: 0 };
const DIR_RIGHT: Point = Point { x: 1, y: 0 };
const DIR_UP: Point = Point { x: 0, y: 1 };
const DIR_DOWN: Point = Point { x: 0, y: -1 };

pub fn solve_part_1(text: &String) -> () {
    let multiply_vector = |d: Point, n: i64| Point {
        x: d.x * n,
        y: d.y * n,
    };
    let trench_sides: Vec<Point> = parse_plan(text)
        .iter()
        .map(|(c, n, _)| multiply_vector(char_to_dir(*c), *n))
        .collect::<Vec<Point>>();
    let volume = compute_lagoon_volume(trench_sides);

    println!("Trench volume in m3:    {volume}");
    println!("Expected puzzle answer: 40714");
}

pub fn solve_part_2(text: &String) -> () {
    let trench_sides: Vec<Point> = parse_plan(text)
        .iter()
        .map(|(_, _, h)| get_hex_vector(h))
        .collect::<Vec<Point>>();
    let volume = compute_lagoon_volume(trench_sides);

    println!("Trench volume in m3:    {volume}");
    println!("Expected puzzle answer: 129849166997110");
}

fn parse_plan(input: &str) -> Vec<(char, i64, &str)> {
    match all_consuming(separated_list1(
        line_ending,
        tuple((
            anychar::<_, Error<_>>,
            preceded(space1, i64),
            delimited(tag(" (#"), hex_digit1, tag(")")),
        )),
    ))(input)
    {
        Err(e) => {
            panic!("bad input: {}", e);
        }
        Ok((_, result)) => {
            return result;
        }
    }
}

fn char_to_dir(c: char) -> Point {
    match c {
        'U' => DIR_UP,
        'D' => DIR_DOWN,
        'L' => DIR_LEFT,
        'R' => DIR_RIGHT,
        _ => {
            panic!("unexpected direction char: {c}");
        }
    }
}

// Parses a dig instruction in the hex format for part 2.
fn get_hex_vector(s: &str) -> Point {
    let num_steps = i64::from_str_radix(&s[0..s.len() - 1], 16).unwrap();
    let c = s.chars().last().unwrap();
    let dir = match c {
        '0' => DIR_RIGHT,
        '1' => DIR_DOWN,
        '2' => DIR_LEFT,
        '3' => DIR_UP,
        _ => {
            panic!("unexpected direction char: {c}");
        }
    };
    return Point {
        x: dir.x * num_steps,
        y: dir.y * num_steps,
    };
}

// Uses the Shoelace formula to compute the area of the polygon specified by the trench sides.
// https://en.wikipedia.org/wiki/Shoelace_formula
fn compute_lagoon_volume(trench_sides: Vec<Point>) -> i64 {
    // Dig out the trench.
    let mut pos = Point { x: 0, y: 0 };
    let mut polygon: Vec<Point> = Vec::new();
    polygon.push(pos);
    for diff_vector in trench_sides {
        pos = pos + diff_vector;
        polygon.push(pos);
    }
    // Close the polygon with a duplicate of the starting point, out of convenience.
    polygon.push(Point { x: 0, y: 0 });

    // Compute area using the shoelace formula with trapezoids.
    let mut polygon_area = 0;
    // We use the midpoints of the trench tiles, which doesn't quite include the full lagoon polygon.
    // We have to correct by the extra area extending 0.5 tiles out around the boundary.
    // This maths out to half the boundary length plus one.
    let mut boundary_length = 0;
    // We do everything doubled, and halve the result, to keep things integer.
    for i in 0..polygon.len() - 1 {
        // Add signed trapezoid area.
        let p = polygon[i];
        let q = polygon[i + 1];
        polygon_area += (q.x - p.x) * (q.y + p.y);
        // Add boundary length.
        let diff_vector = q - p;
        boundary_length += diff_vector.x.abs() + diff_vector.y.abs();
    }
    assert!(polygon_area % 2 == 0);
    assert!(boundary_length % 2 == 0);

    // We don't know if the polygon is specified in positive or negative direction, account for that.
    polygon_area = polygon_area.abs();

    return polygon_area / 2 + boundary_length / 2 + 1;
}
