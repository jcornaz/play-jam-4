#![cfg_attr(not(feature = "std"), no_std)]

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Aabb {
    x: Range,
    y: Range,
}

impl Aabb {
    pub fn from_min_max(min: impl Into<[f32; 2]>, max: impl Into<[f32; 2]>) -> Self {
        let min = min.into();
        let max = max.into();
        let x = Range::from_min_max(min[0], max[0]);
        let y = Range::from_min_max(min[1], max[1]);
        Self { x, y }
    }

    /// Returns by how much [`self`] should be moved in order to resolve penetration with [other]
    ///
    /// Returns `None` if the two shape are not collided
    #[cfg(any(feature = "std", feature = "libm"))]
    pub fn penetration(self, other: Self) -> Option<[f32; 2]> {
        let x = self.x.penetration(other.x)?;
        let y = self.y.penetration(other.y)?;
        Some(if abs(x) < abs(y) { [x, 0.] } else { [0., y] })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Range {
    min: f32,
    max: f32,
}

impl Range {
    fn from_min_max(min: f32, max: f32) -> Self {
        Self {
            min: min.min(max),
            max: min.max(max),
        }
    }

    #[cfg(any(feature = "std", feature = "libm"))]
    fn penetration(self, other: Self) -> Option<f32> {
        let p1 = Some(other.min - self.max).filter(|p| *p < 0.)?;
        let p2 = Some(other.max - self.min).filter(|p| *p > 0.)?;
        Some(if abs(p1) < abs(p2) { p1 } else { p2 })
    }
}

#[cfg(feature = "std")]
fn abs(v: f32) -> f32 {
    v.abs()
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
fn abs(v: f32) -> f32 {
    libm::absf(v)
}

#[cfg(all(test, any(feature = "std", feature = "libm")))]
mod tests {
    use super::*;

    #[test]
    fn range_should_penetrate_self() {
        let range = Range::from_min_max(0., 1.);
        assert_eq!(range.penetration(range).map(abs), Some(1.0));
    }
}