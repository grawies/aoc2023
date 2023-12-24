use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending, space0},
    combinator::all_consuming,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
};

use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;

type Point3 = (i64, i64, i64);
type BigInt3 = (BigInt, BigInt, BigInt);

fn fstone(p: &BigInt3, v: &BigInt3) -> String {
    return format!("{} {} {} @ {} {} {}", p.0, p.1, p.2, v.0, v.1, v.2);
}

pub fn solve_part_1(text: &String) -> () {
    let paths = parse_trajectories(text);
    let min_isct: BigInt;
    let max_isct: BigInt;
    if paths.len() > 10 {
        min_isct = (200000000000000 as i64).to_bigint().unwrap();
        max_isct = (400000000000000 as i64).to_bigint().unwrap();
    } else {
        min_isct = 7.to_bigint().unwrap();
        max_isct = 27.to_bigint().unwrap();
    }
    let mut num_xy_intersecting = 0;
    let sign = |v: &BigInt| {
        if v.clone() < Zero::zero() {
            return -1;
        }
        return 1;
    };
    let to_bigint = |p: &Point3| {
        (
            p.0.to_bigint().unwrap(),
            p.1.to_bigint().unwrap(),
            p.2.to_bigint().unwrap(),
        )
    };

    for (i, (p0_i64, v0_i64)) in paths.iter().enumerate() {
        for (p1_i64, v1_i64) in paths.iter().skip(i + 1) {
            let p0 = to_bigint(p0_i64);
            let v0 = to_bigint(v0_i64);
            let p1 = to_bigint(p1_i64);
            let v1 = to_bigint(v1_i64);

            let intersection: bool;

            // Solve system:
            // [v0 -v1] t = p1 - p0    (1)
            let det = -v0.0.clone() * v1.1.clone() + v0.1.clone() * v1.0.clone();
            if det.is_zero() {
                // Degenerate case, 0 or infinite solutions depending on RHS.
                // For now, let's assume that there aren't infinite solutions.

                intersection = false;
            } else {
                let adj = [[-v1.1.clone(), v1.0.clone()], [-v0.1.clone(), v0.0.clone()]];
                // adjA * A == det * I   (2)
                // Combining (1) and (2), we get:
                // detA * t = adjA * (p1 - p0).
                let dp = (p1.0.clone() - p0.0.clone(), p1.1.clone() - p0.1.clone());
                let t_0_det_s: BigInt = sign(&det)
                    * (adj[0][0].clone() * dp.0.clone() + adj[0][1].clone() * dp.1.clone());
                let t_1_det_s: BigInt = sign(&det)
                    * (adj[1][0].clone() * dp.0.clone() + adj[1][1].clone() * dp.1.clone());

                let x_det_s =
                    p0.0.clone() * det.clone() * sign(&det) + v0.0.clone() * t_0_det_s.clone();
                let y_det_s =
                    p0.1.clone() * det.clone() * sign(&det) + v0.1.clone() * t_0_det_s.clone();

                let min_isct_det_s = min_isct.clone() * det.clone() * sign(&det);
                let max_isct_det_s = max_isct.clone() * det.clone() * sign(&det);
                if
                // check intersection at positive time
                t_0_det_s.clone() >= Zero::zero() && t_1_det_s.clone() >= Zero::zero()
                // check intersection within bounds
                && min_isct_det_s <= x_det_s
                    && x_det_s <= max_isct_det_s
                    && min_isct_det_s <= y_det_s
                    && y_det_s <= max_isct_det_s
                {
                    intersection = true;
                } else {
                    intersection = false;
                }
            }

            if !intersection {
                continue;
            }
            // Intersection!
            num_xy_intersecting += 1;
        }
    }

    println!("Num intersections in XY plane: {num_xy_intersecting}");
    println!("Expected puzzle answer:        14046");
}

fn parse_trajectories(input: &str) -> Vec<(Point3, Point3)> {
    let commaspace = || terminated(tag(","), space0);
    return all_consuming(separated_list1(
        line_ending::<_, Error<_>>,
        separated_pair(
            tuple((
                i64,
                preceded(commaspace(), i64),
                preceded(commaspace(), i64),
            )),
            delimited(space0, tag("@"), space0),
            tuple((
                i64,
                preceded(commaspace(), i64),
                preceded(commaspace(), i64),
            )),
        ),
    ))(input)
    .unwrap()
    .1;
}
