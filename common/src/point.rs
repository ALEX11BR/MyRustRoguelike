use std::ops::Add;

use crate::{LEVEL_HEIGHT, LEVEL_WIDTH};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn in_bounds(self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < LEVEL_WIDTH as i32 && self.y < LEVEL_HEIGHT as i32
    }
    // Doesn't include diagonal neighbors
    pub fn is_neighboring(self, other: Point) -> bool {
        (self.x == other.x && self.y.abs_diff(other.y) <= 1)
            || (self.y == other.y && self.x.abs_diff(other.x) <= 1)
    }
    pub fn iter_neighbors(self) -> impl Iterator<Item = Self> {
        vec![
            Point::new(0, 1),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(-1, 0),
        ]
        .into_iter()
        .map(move |p| self + p)
    }
}
impl Into<(isize, isize)> for Point {
    fn into(self) -> (isize, isize) {
        (self.y as isize, self.x as isize)
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
