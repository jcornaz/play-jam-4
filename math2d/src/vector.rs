use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::angle::Angle;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Vector<T = f32> {
    /// X-axis component
    pub x: T,
    /// X-axis component
    pub y: T,
}

impl<T> Vector<T> {
    /// Create a new vector instance
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Vector<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    /// Returns the dot product of the two vector (aka projection)
    #[must_use]
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Squared magnitude (aka squared length) of the vector
    ///
    /// This is usually faster than getting the actual magnitude as it doesn't require
    /// a square-root operation
    #[must_use]
    pub fn magnitude_squared(self) -> T {
        self.dot(self)
    }
}

impl Vector<f32> {
    /// Zero vector
    pub const ZERO: Self = Self::new(0., 0.);

    /// Unit vector pointing in direction of the x axis
    pub const X: Self = Self::new(1., 0.);

    /// Unit vector pointing in direction of the y axis
    pub const Y: Self = Self::new(0., 1.);

    /// Rotate the vector by the given angle
    #[must_use]
    pub fn rotate(self, angle: impl Angle) -> Self {
        let (cos, sin) = Vector::from(angle).into();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    /// Magnitude (aka length) of the vector
    ///
    /// Consider using [`Self::magnitude_squared`] instead if possible to avoid having to perform
    /// a square root operation
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))]
    pub fn magnitude(self) -> f32 {
        crate::sqrt(self.magnitude_squared())
    }

    /// Return the normalized version of the vector if possible
    ///
    /// If the vector cannot be normalized, returns `None` (e.g. if the vector has a magnitude of zero)
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))]
    pub fn normalize(self) -> Option<Self> {
        let recip = self.magnitude().recip();
        if recip.is_finite() && recip > 0.0 {
            Some(self * recip)
        } else {
            None
        }
    }

    /// cast into a [`Vector<i32>`]
    #[must_use]
    pub fn as_vector_i32(self) -> Vector<i32> {
        Vector {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl Vector<i32> {
    /// Zero vector
    pub const ZERO: Self = Self::new(0, 0);

    /// Unit vector pointing in direction of the x axis
    pub const X: Self = Self::new(1, 0);

    /// Unit vector pointing in direction of the y axis
    pub const Y: Self = Self::new(0, 1);

    /// cast into a [`Vector<f32>`]
    #[must_use]
    pub fn as_vector_f32(self) -> Vector<f32> {
        Vector {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl<T> From<Vector<T>> for [T; 2] {
    fn from(Vector { x, y }: Vector<T>) -> Self {
        [x, y]
    }
}

impl<T> From<[T; 2]> for Vector<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Vector<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<Vector<T>> for (T, T) {
    fn from(Vector { x, y }: Vector<T>) -> Self {
        (x, y)
    }
}

impl<A> From<A> for Vector<f32>
where
    A: Angle,
{
    fn from(value: A) -> Self {
        Self {
            x: value.cos(),
            y: value.sin(),
        }
    }
}

impl<T> AddAssign for Vector<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Add for Vector<T>
where
    T: Add,
{
    type Output = Vector<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> SubAssign for Vector<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub,
{
    type Output = Vector<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Neg for Vector<T>
where
    T: Neg,
{
    type Output = Vector<T::Output>;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> MulAssign<T> for Vector<T>
where
    T: Copy + MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Copy + Mul<T>,
{
    type Output = Vector<T::Output>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector<T>
where
    T: Copy + DivAssign<T>,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Copy + Div<T>,
{
    type Output = Vector<T::Output>;
    fn div(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[cfg(any(feature = "std", feature = "libm"))]
    use crate::angle::{Degrees, Radians};

    use super::*;

    #[rstest]
    #[case(Vector::<i32>::ZERO, Vector::<i32>::ZERO, Vector::<i32>::ZERO)]
    #[case(Vector::<i32>::ZERO, Vector::<i32>::X, Vector::<i32>::X)]
    #[case(Vector::new(1, 2), Vector::new(3, 4), Vector::new(4, 6))]
    fn test_add(
        #[case] lhs: Vector<i32>,
        #[case] rhs: Vector<i32>,
        #[case] expected_sum: Vector<i32>,
    ) {
        assert_eq!(lhs + rhs, expected_sum);
        let mut sum = lhs;
        sum += rhs;
        assert_eq!(sum, expected_sum);
    }

    #[rstest]
    #[case(Vector::<i32>::ZERO, Vector::<i32>::ZERO, Vector::<i32>::ZERO)]
    #[case(Vector::<i32>::Y, Vector::<i32>::Y, Vector::<i32>::ZERO)]
    #[case(Vector::new(6, 5), Vector::<i32>::new(3, 1), Vector::<i32>::new(3, 4))]
    fn test_sub(
        #[case] lhs: Vector<i32>,
        #[case] rhs: Vector<i32>,
        #[case] expected_sum: Vector<i32>,
    ) {
        assert_eq!(lhs - rhs, expected_sum);
        let mut diff = lhs;
        diff -= rhs;
        assert_eq!(diff, expected_sum);
    }

    #[rstest]
    #[case(Vector::<f32>::ZERO, Degrees(90.), Vector::<f32>::ZERO)]
    #[case(Vector::<f32>::X, Degrees(0.), Vector::<f32>::X)]
    #[case(Vector::<f32>::X, Degrees(360.), Vector::<f32>::X)]
    #[case(Vector::<f32>::X, Radians(0.), Vector::<f32>::X)]
    #[case(Vector::<f32>::X, Radians(core::f32::consts::PI * 2.), Vector::<f32>::X)]
    #[case(Vector::<f32>::X, Degrees(90.), Vector::<f32>::Y)]
    #[case(Vector::<f32>::X, Degrees(180.), -Vector::<f32>::X)]
    #[case(Vector::<f32>::X, Radians(core::f32::consts::PI), -Vector::<f32>::X)]
    #[case(Vector::<f32>::X, Degrees(270.), -Vector::<f32>::Y)]
    #[case(Vector::<f32>::X, Degrees(45.), Vector::<f32>::new(1./2_f32.sqrt(), 1./2_f32.sqrt()))]
    #[cfg(any(feature = "std", feature = "libm"))]
    fn test_rotate(
        #[case] source: Vector,
        #[case] angle: impl Angle,
        #[case] expected_result: Vector,
    ) {
        let result = source.rotate(angle);
        assert!((result.x - expected_result.x).abs() < 0.000001);
        assert!((result.y - expected_result.y).abs() < 0.000001);
    }

    #[rstest]
    #[case(Vector::<f32>::X, Vector::<f32>::X)]
    #[case(Vector::<f32>::Y, Vector::<f32>::Y)]
    #[case(Vector::new(2., 0.), Vector::<f32>::X)]
    #[cfg(any(feature = "std", feature = "libm"))]
    fn normalize_should_succeed(#[case] vector: Vector, #[case] expected_normal: Vector) {
        let normal = vector.normalize().unwrap();
        assert!((normal.x - expected_normal.x).abs() < 0.000001);
        assert!((normal.y - expected_normal.y).abs() < 0.000001);
    }

    #[rstest]
    #[cfg(any(feature = "std", feature = "libm"))]
    fn normalize_should_returns_none_if_cannot_be_normalized(
        #[values(Vector::<f32>::ZERO, Vector::new(f32::INFINITY, 0.), Vector::new(f32::NAN, 0.))]
        vector: Vector,
    ) {
        assert_eq!(vector.normalize(), None);
    }
}
