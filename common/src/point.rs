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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_new() {
        let point = Point::new(2, 1);
        assert_eq!(point.x, 2);
        assert_eq!(point.y, 1);
    }
    #[test]
    fn point_eq() {
        let point = Point::new(3, 2);

        assert_eq!(point, Point::new(3, 2));

        assert_ne!(point, Point::new(2, 3));
        assert_ne!(point, Point::new(1, 4));
    }
    #[test]
    fn point_in_bounds() {
        assert!(Point::new(0, 0).in_bounds());
        assert!(Point::new(5, 6).in_bounds());
        assert!(Point::new(50, 10).in_bounds());
        assert!(Point::new(LEVEL_WIDTH - 1, LEVEL_HEIGHT - 1).in_bounds());

        assert!(!Point::new(10, 50).in_bounds());
        assert!(!Point::new(0, -5).in_bounds());
        assert!(!Point::new(-1, 0).in_bounds());
        assert!(!Point::new(-4, -5).in_bounds());
        assert!(!Point::new(90, 10).in_bounds());
        assert!(!Point::new(90, 50).in_bounds());
        assert!(!Point::new(LEVEL_WIDTH, 0).in_bounds());
        assert!(!Point::new(0, LEVEL_HEIGHT).in_bounds());
        assert!(!Point::new(LEVEL_WIDTH, LEVEL_HEIGHT).in_bounds());
    }
    #[test]
    fn point_is_neighboring() {
        let point = Point::new(2, 1);

        assert!(point.is_neighboring(Point::new(2, 0)));
        assert!(point.is_neighboring(Point::new(2, 2)));
        assert!(point.is_neighboring(Point::new(1, 1)));
        assert!(point.is_neighboring(Point::new(3, 1)));

        assert!(!point.is_neighboring(Point::new(2, 3)));
        assert!(!point.is_neighboring(Point::new(0, 3)));
        assert!(!point.is_neighboring(Point::new(20, 13)));
    }
    #[test]
    fn point_iter_neighbors() {
        let point = Point::new(1, 2);

        assert_eq!(point.iter_neighbors().count(), 4);

        assert!(point.iter_neighbors().any(|p| p == Point::new(1, 3)));
        assert!(point.iter_neighbors().any(|p| p == Point::new(1, 1)));
        assert!(point.iter_neighbors().any(|p| p == Point::new(0, 2)));
        assert!(point.iter_neighbors().any(|p| p == Point::new(2, 2)));
    }
    #[test]
    fn point_into_isize_isize() {
        let point = Point::new(4, 2);
        let point_isize_isize: (isize, isize) = point.into();

        assert_eq!(point_isize_isize, (2, 4));
    }
    #[test]
    fn point_add() {
        let point = Point::new(4, 1);

        assert_eq!(point + Point::new(0, 0), point);
        assert_eq!(point + Point::new(0, 1), Point::new(4, 2));
        assert_eq!(point + Point::new(0, -1), Point::new(4, 0));
        assert_eq!(point + Point::new(-1, 0), Point::new(3, 1));
        assert_eq!(point + Point::new(1, 0), Point::new(5, 1));
    }
}
