use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u64},
    combinator::all_consuming,
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
};

use std::cmp;

use std::collections::HashSet;

type Point3 = (usize, usize, usize);

fn parse_and_drop_bricks(text: &String) -> (Vec<usize>, Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let bricks = parse_bricks(text);
    let mut bricks_below: Vec<Vec<usize>> = vec![Vec::new(); bricks.len()];
    let mut bricks_on_top: Vec<Vec<usize>> = vec![Vec::new(); bricks.len()];
    let mut bricks_by_z_asc: Vec<usize> = (0..bricks.len()).collect();
    bricks_by_z_asc.sort_unstable_by_key(|i| cmp::min(bricks[*i].0 .2, bricks[*i].1 .2));

    let mut zbuf: [[usize; 10]; 10] = [[0; 10]; 10];
    let mut zbuf_idx: [[Option<usize>; 10]; 10] = [[None; 10]; 10];

    for i in &bricks_by_z_asc {
        let brick = bricks[*i];
        let brick_z_height =
            cmp::max(brick.0 .2, brick.1 .2) - cmp::min(brick.0 .2, brick.1 .2) + 1;
        let mut max_z_below = 0;
        for x in brick.0 .0..brick.1 .0 + 1 {
            for y in brick.0 .1..brick.1 .1 + 1 {
                max_z_below = cmp::max(max_z_below, zbuf[y][x]);
            }
        }
        let mut supports: Vec<usize> = Vec::new();
        for x in brick.0 .0..brick.1 .0 + 1 {
            for y in brick.0 .1..brick.1 .1 + 1 {
                if max_z_below > 0 && zbuf[y][x] == max_z_below {
                    let support_index = zbuf_idx[y][x].unwrap();
                    if supports.last() != Some(&support_index) {
                        supports.push(support_index);
                    }
                }
                zbuf[y][x] = max_z_below + brick_z_height;
                zbuf_idx[y][x] = Some(*i);
            }
        }
        for j in &supports {
            bricks_on_top[*j].push(*i);
        }
        bricks_below[*i] = supports;
    }
    return (bricks_by_z_asc, bricks_below, bricks_on_top);
}

pub fn solve_part_1(text: &String) -> () {
    let (_, bricks_below, bricks_on_top) = parse_and_drop_bricks(text);

    let answer = (0..bricks_on_top.len())
        .filter(|i| bricks_on_top[*i].iter().all(|j| bricks_below[*j].len() > 1))
        .count();

    println!("Free  bricks:           {answer}");
    println!("Expected puzzle answer: 413");
}

pub fn solve_part_2(text: &String) -> () {
    let (bricks_by_z_asc, bricks_below, _) = parse_and_drop_bricks(text);

    let mut support_closure: Vec<HashSet<usize>> = vec![HashSet::new(); bricks_by_z_asc.len()];
    let mut sum_num_fallen_bricks = 0;
    for i in bricks_by_z_asc.iter().rev() {
        let sc = support_closure[*i].clone();
        let mut fallen: HashSet<usize> = HashSet::from([*i]);
        // Iterating from below over the bricks above i, collect all the bricks that would fall by removing i.
        for j in bricks_by_z_asc.iter().filter(|j| sc.contains(j)) {
            // A brick falls if all the bricks it rests on fall.
            if bricks_below[*j].iter().all(|k| fallen.contains(k)) {
                fallen.insert(*j);
            }
        }
        sum_num_fallen_bricks += fallen.len() - 1;

        for j in &bricks_below[*i] {
            support_closure[*j].insert(*i);
            for k in &sc {
                support_closure[*j].insert(*k);
            }
        }
    }

    println!("Sum of number of bricks that can fall: {sum_num_fallen_bricks}");
    println!("Expected puzzle answer:                41610");
}

fn parse_bricks(input: &str) -> Vec<(Point3, Point3)> {
    return all_consuming(separated_list1(
        line_ending::<_, Error<_>>,
        separated_pair(
            tuple((u64, preceded(tag(","), u64), preceded(tag(","), u64))),
            tag("~"),
            tuple((u64, preceded(tag(","), u64), preceded(tag(","), u64))),
        ),
    ))(input)
    .unwrap()
    .1
    .iter()
    .map(|(p, q)| {
        (
            (p.0 as usize, p.1 as usize, p.2 as usize),
            (q.0 as usize, q.1 as usize, q.2 as usize),
        )
    })
    .collect();
}
