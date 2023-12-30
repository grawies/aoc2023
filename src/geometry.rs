use auto_ops::impl_op_ex;
use num_bigint::{BigInt, ToBigInt};

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl_op_ex!(+ |a: &Point, b: &Point| -> Point {
Point {
    x: a.x + b.x,
    y: a.y + b.y,
}});

impl_op_ex!(-|a: &Point, b: &Point| -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});

impl_op_ex!(-|a: &Point| -> Point { Point { x: -a.x, y: -a.y } });

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Point4D {
    pub x: BigInt,
    pub y: BigInt,
    pub z: BigInt,
    pub t: BigInt,
}

impl_op_ex!(+ |a: &Point4D, b: &Point4D| -> Point4D {
Point4D {
    x: a.x.clone() + b.x.clone(),
    y: a.y.clone() + b.y.clone(),
    z: a.z.clone() + b.z.clone(),
    t: a.t.clone() + b.t.clone(),
}});

impl_op_ex!(-|a: &Point4D, b: &Point4D| -> Point4D {
    Point4D {
        x: a.x.clone() - b.x.clone(),
        y: a.y.clone() - b.y.clone(),
        z: a.z.clone() - b.z.clone(),
        t: a.t.clone() - b.t.clone(),
    }
});

impl_op_ex!(-|a: &Point4D| -> Point4D {
    Point4D {
        x: -a.x.clone(),
        y: -a.y.clone(),
        z: -a.z.clone(),
        t: -a.t.clone(),
    }
});

impl From<Point4D> for Vec<BigInt> {
    fn from(p: Point4D) -> Self {
        return vec![p.x, p.y, p.z, p.t];
    }
}

// Returns the transpose of |a|.
// Clones liberally.
pub fn transpose<T: Clone>(a: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut b: Vec<Vec<T>> = Vec::new();
    for i in 0..a[0].len() {
        b.push(Vec::new());
        for j in 0..a.len() {
            b[i].push(a[j][i].clone());
        }
    }
    return b;
}
