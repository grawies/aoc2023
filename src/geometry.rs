use auto_ops::impl_op_ex;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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
