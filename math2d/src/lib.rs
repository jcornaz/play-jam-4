#![cfg_attr(not(feature = "std"), no_std)]

pub use angle::Angle;
#[cfg(feature = "point")]
pub use point::Point;
pub use vector::Vector;

mod angle;
#[cfg(feature = "point")]
mod point;
mod vector;

#[cfg(feature = "std")]
fn cos(v: f32) -> f32 {
    v.cos()
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
fn cos(v: f32) -> f32 {
    libm::cosf(v)
}

#[cfg(feature = "std")]
fn sin(v: f32) -> f32 {
    v.sin()
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
fn sin(v: f32) -> f32 {
    libm::sinf(v)
}

#[cfg(feature = "std")]
fn sqrt(v: f32) -> f32 {
    v.sqrt()
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
fn sqrt(v: f32) -> f32 {
    libm::sqrtf(v)
}
