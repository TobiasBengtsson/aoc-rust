use std::ops::{Add, Sub};

#[cfg(test)]
use quickcheck::Arbitrary;

pub(crate) mod template;

type IntType = usize;

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct Point {
    pub x: IntType,
    pub y: IntType,
}

impl Point {
    pub fn new(x: IntType, y: IntType) -> Point {
        Point {x, y}
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x.wrapping_add(rhs.x), y: self.y.wrapping_add(rhs.y) }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point { x: self.x.wrapping_sub(rhs.x), y: self.y.wrapping_sub(rhs.y) }
    }
}

#[cfg(test)]
impl Arbitrary for Point {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Point { x: IntType::arbitrary(g), y: IntType::arbitrary(g) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_point_addition() {
        let p1 = Point::new(4, 5);
        let p2 = Point::new(5, 6);
        assert_eq!(Point::new(9, 11), p1 + p2);
    }

    #[test]
    fn test_point_subtraction() {
        let p1 = Point::new(5, 3);
        let p2 = Point::new(4, 2);
        assert_eq!(Point::new(1, 1), p1 - p2);
    }

    #[quickcheck]
    fn test_point_addition_associative(p1: Point, p2: Point, p3: Point) {
        assert_eq!((p1 + p2) + p3, p1 + (p2 + p3));
    }

    #[quickcheck]
    fn test_point_addition_commutative(p1: Point, p2: Point) {
        assert_eq!(p1 + p2, p2 + p1);
    }

    #[quickcheck]
    fn test_point_addition_default_is_identity(p1: Point) {
        assert_eq!(Point::default() + p1, p1);
        assert_eq!(p1 + Point::default(), p1);
    }

    #[quickcheck]
    fn test_point_subtraction_is_element_wise(p1: Point, p2: Point) {
        let p3 = p1 - p2;
        assert_eq!(p1.x.wrapping_sub(p2.x), p3.x);
        assert_eq!(p1.y.wrapping_sub(p2.y), p3.y);
    }

    #[quickcheck]
    fn test_point_subtraction_default_is_right_identity(p1: Point) {
        assert_eq!(p1 - Point::default(), p1);
    }

    #[quickcheck]
    fn test_subtraction_is_inverse_of_addition(p1: Point, p2: Point) {
        let p3 = p1 + p2;
        assert_eq!(p1, p3 - p2);
        assert_eq!(p2, p3 - p1);
    }
}
