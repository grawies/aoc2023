use std::ops::Add;
use std::ops::Sub;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

// TODO: Use auto_ops or impl_ops to add overloads for reference types.
// Or manually. About lifetime annotations:
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch19-02-advanced-lifetimes.html
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
