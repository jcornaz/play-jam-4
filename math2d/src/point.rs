use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::vector::Vector;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Point<T = f32> {
    pub x: T,
    pub y: T,
}

impl Point<f32> {
    pub const ORIGIN: Self = Self::new(0., 0.);

    pub fn as_point_i32(self) -> Point<i32> {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl Point<i32> {
    pub const ORIGIN: Self = Self::new(0, 0);

    pub fn as_point_f32(self) -> Point<f32> {
        Point {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<Vector<T>> for Point<T> {
    fn from(Vector { x, y }: Vector<T>) -> Self {
        Self { x, y }
    }
}

impl<T> From<Point<T>> for Vector<T> {
    fn from(Point { x, y }: Point<T>) -> Self {
        Self { x, y }
    }
}

impl<T> From<[T; 2]> for Point<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self { x, y }
    }
}

impl<T> From<Point<T>> for [T; 2] {
    fn from(Point { x, y }: Point<T>) -> Self {
        [x, y]
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(Point { x, y }: Point<T>) -> Self {
        (x, y)
    }
}

impl<T> AddAssign<Vector<T>> for Point<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Vector<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Add<Vector<T>> for Point<T>
where
    T: Add,
{
    type Output = Point<T::Output>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> SubAssign<Vector<T>> for Point<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Vector<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Sub<Vector<T>> for Point<T>
where
    T: Sub,
{
    type Output = Point<T::Output>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Sub<Point<T>> for Point<T>
where
    T: Sub,
{
    type Output = Vector<T::Output>;
    fn sub(self, rhs: Point<T>) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Point::new(1, 2), Vector::new(3, 4), Point::new(4, 6))]
    fn test_add(
        #[case] lhs: Point<i32>,
        #[case] rhs: Vector<i32>,
        #[case] expected_sum: Point<i32>,
    ) {
        assert_eq!(lhs + rhs, expected_sum);
        let mut sum = lhs;
        sum += rhs;
        assert_eq!(sum, expected_sum);
    }

    #[rstest]
    #[case(Point::new(6, 5), Vector::<i32>::new(3, 1), Point::<i32>::new(3, 4))]
    fn test_sub(
        #[case] lhs: Point<i32>,
        #[case] rhs: Vector<i32>,
        #[case] expected_sum: Point<i32>,
    ) {
        assert_eq!(lhs - rhs, expected_sum);
        let mut diff = lhs;
        diff -= rhs;
        assert_eq!(diff, expected_sum);
    }
}
