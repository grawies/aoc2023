use crate::geometry::{transpose, Point4D};

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

// Returns the matrix a with row i and column j removed.
fn matrix_minor(a: &Vec<Vec<BigInt>>, i: usize, j: usize) -> Vec<Vec<BigInt>> {
    return a
        .iter()
        .enumerate()
        .filter(|(r, _)| *r != i)
        .map(|(_, row)| {
            row.iter()
                .enumerate()
                .filter(|(c, _)| *c != j)
                .map(|(_, x)| x.clone())
                .collect::<Vec<BigInt>>()
        })
        .collect::<Vec<Vec<BigInt>>>();
}

// Returns the determinant of the square matrix a.
fn determinant(a: Vec<Vec<BigInt>>) -> BigInt {
    let n = a.len();
    for row in &a {
        assert!(row.len() == n);
    }
    if n == 1 {
        return a[0][0].clone();
    }
    let mut det: BigInt = Zero::zero();
    let mut sgn = 1;
    for i in 0..n {
        det += sgn * a[i][0].clone() * determinant(matrix_minor(&a, i, 0));
        sgn = -sgn;
    }
    return det;
}

// Returns the adjugate of the square matrix a.
fn adjugate(a: Vec<Vec<BigInt>>) -> Vec<Vec<BigInt>> {
    let n = a.len();
    for row in &a {
        assert!(row.len() == n);
    }
    let det = determinant(a.clone());
    assert!(!det.is_zero());
    let mut adj: Vec<Vec<BigInt>> = Vec::new();
    for i in 0..n {
        adj.push(Vec::new());
        for j in 0..n {
            let sign = match (i + j) % 2 {
                0 => 1,
                1 => -1,
                _ => panic!("unexpected modulo 2 result"),
            };
            adj[i].push(sign * determinant(matrix_minor(&a, j, i)));
        }
    }
    return adj;
}

fn mul(p: &Point4D, a: &BigInt) -> Point4D {
    Point4D {
        x: a.clone() * p.x.clone(),
        y: a.clone() * p.y.clone(),
        z: a.clone() * p.z.clone(),
        t: a.clone() * p.t.clone(),
    }
}

fn checked_div(p: &Point4D, a: &BigInt) -> Point4D {
    assert!((p.x.clone() % a.clone()).is_zero());
    assert!((p.y.clone() % a.clone()).is_zero());
    assert!((p.z.clone() % a.clone()).is_zero());
    assert!((p.t.clone() % a.clone()).is_zero());
    Point4D {
        x: p.x.clone() / a.clone(),
        y: p.y.clone() / a.clone(),
        z: p.z.clone() / a.clone(),
        t: p.t.clone() / a.clone(),
    }
}

// Returns a point on a trajectory passing through each of the three input trajectories, in coordinates (x,y,z,time).
fn find_intersection_point(
    (p1, v1): (Point3, Point3),
    (p2, v2): (Point3, Point3),
    (p3, v3): (Point3, Point3),
) -> Point4D {
    // Most math here is done in 4D space, with coordinates x, y, z, and time.
    // Each trajectory (p,v) corresponds to a 1D line of points [0, p] + t*[1, v] in 4D space.
    let from3dto4d = |(x, y, z): Point3, t: i64| Point4D {
        x: x.to_bigint().unwrap(),
        y: y.to_bigint().unwrap(),
        z: z.to_bigint().unwrap(),
        t: t.to_bigint().unwrap(),
    };
    let p1 = from3dto4d(p1, 0);
    let p2 = from3dto4d(p2, 0);
    let p3 = from3dto4d(p3, 0);
    let v1 = from3dto4d(v1, 1);
    let v2 = from3dto4d(v2, 1);
    let v3 = from3dto4d(v3, 1);
    // We seek a line that passes through all three of our input lines.
    // Any such line lies in the subspace spanned by points on the two first lines, and can be parameterized as:
    //    p(s,u,w) = p1 + s*v1 + u*(p2+v2-p1) + w*(p2-p1)
    // We seek the intersection with the third line:
    //    p(s,u,w) = p3 + t * v3,
    // or:
    //    A * [s u w t]' = b,
    // where A is the 4x4 matrix [v1 p2+v2-p1 p2-p1 -v3]
    // and b = p3 - p1.
    let a = transpose(vec![
        Vec::from(v1.clone()),
        Vec::from(&p2 + &v2 - &p1),
        Vec::from(&p2 - &p1),
        Vec::from(-&v3),
    ]);
    let b = p3.clone() - p1.clone();
    // If A has non-zero determinant, there is only a single solution.
    let det = determinant(a.clone());
    assert!(!det.is_zero());
    let adj = adjugate(a);
    // Let the suffix _d denote that the object is scaled by a's determinant.
    let dot_product = |a: &Vec<BigInt>, b: &Point4D| {
        assert!(a.len() == 4);
        a[0].clone() * b.clone().x
            + a[1].clone() * b.y.clone()
            + a[2].clone() * b.z.clone()
            + a[3].clone() * b.t.clone()
    };
    let soln_d = vec![
        dot_product(&adj[0], &b),
        dot_product(&adj[1], &b),
        dot_product(&adj[2], &b),
        dot_product(&adj[3], &b),
    ];
    // Intersection point on the third line (p3,v3).
    let isct_3_d = mul(&p3, &det) + mul(&v3, &soln_d[3]);
    return checked_div(&isct_3_d, &det);
}

pub fn solve_part_2(text: &String) -> () {
    let paths = parse_trajectories(text);

    // We only need three trajectories to determine the (guaranteed to exist and be unique) common trajectory.
    let q1 = find_intersection_point(paths[0], paths[1], paths[2]);
    let q2 = find_intersection_point(paths[1], paths[2], paths[3]);
    let dq = q2 - q1.clone();

    // On the line q1 + t*(q2-q1), we seek the point such that the t-coordinate is 0.
    let p0 = checked_div(&(mul(&q1, &dq.t) - mul(&dq, &q1.t)), &dq.t);
    let p0s = p0.x + p0.y + p0.z;
    println!("Starting point sum:     {}", p0s);
    println!("Expected puzzle answer: 808107741406756");
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
