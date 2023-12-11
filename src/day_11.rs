use std::cmp;

type Point = (i64, i64);

// Reads the coordinates of all '#' characters.
fn parse_positions(text: &String) -> Vec<Point> {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut ps = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                ps.push((x as i64, y as i64));
            }
        }
    }
    return ps;
}

// Returns a vector whose i'th element gives the number of gaps in |vs| up to coordinate i.
fn count_gaps(vs: Vec<i64>) -> Vec<i64> {
    let mut counter: Vec<i64> = Vec::new();
    let mut i: usize = 0;
    let mut coordinate: i64 = 0;
    let mut count: usize = 0;
    while i < vs.len() {
        counter.push(count as i64);
        coordinate += 1;
        let i_prev = i;
        while i < vs.len() && vs[i] < coordinate as i64 {
            i += 1;
        }
        if i == i_prev {
            count += 1;
        }
    }
    return counter;
}

fn galaxy_distance_sum(positions: Vec<Point>, galaxy_expansion_multiplier: i64) -> i64 {
    let mut xpos = positions.iter().map(|(x, _)| *x).collect::<Vec<i64>>();
    let mut ypos = positions.iter().map(|(_, y)| *y).collect::<Vec<i64>>();
    xpos.sort();
    ypos.sort();
    let x_gaps_csum = count_gaps(xpos);
    let y_gaps_csum = count_gaps(ypos);
    let mut distance_sum = 0;
    for (i, (px, py)) in positions.iter().enumerate() {
        for (qx, qy) in positions[..i].iter() {
            let naive_distance = (px - qx).abs() + (py - qy).abs();
            let extra_x_gap =
                x_gaps_csum[*cmp::max(px, qx) as usize] - x_gaps_csum[*cmp::min(px, qx) as usize];
            let extra_y_gap =
                y_gaps_csum[*cmp::max(py, qy) as usize] - y_gaps_csum[*cmp::min(py, qy) as usize];
            distance_sum +=
                naive_distance + (extra_x_gap + extra_y_gap) * (galaxy_expansion_multiplier - 1);
        }
    }
    return distance_sum;
}

pub fn solve_part_1(text: &String) -> () {
    let positions = parse_positions(text);
    let distance_sum = galaxy_distance_sum(positions, /*galaxy_expansion_multiplier*/ 2);

    println!("Sum of distances:       {distance_sum}");
    println!("Expected puzzle answer: 9418609");
}

pub fn solve_part_2(text: &String) -> () {
    let positions = parse_positions(text);
    let distance_sum =
        galaxy_distance_sum(positions, /*galaxy_expansion_multiplier*/ 1_000_000);
    // 82000210 too low

    println!("Sum of distances:       {distance_sum}");
    println!("Expected puzzle answer: 593821230983");
}
