//! Statistics toolbox.

#[cfg(test)]
extern crate assert;

use std::ops::{Add, Div, Mul, Sub};

mod moment;

pub use moment::{mean, variance};

/// A real number.
pub trait Real: Copy + Add<Output=Self> + Div<Output=Self> + Mul<Output=Self> + Sub<Output=Self> {
    /// Return the unity.
    fn one() -> Self;

    /// Return the zero.
    fn zero() -> Self;

    /// Create a real number from a natural one.
    fn from_natural(usize) -> Self;
}

/// A means of converting an arbitrary quantity to a real number.
pub trait ToReal<T: Real> {
    /// Convert to a real number.
    fn to_real(&self) -> T;
}

macro_rules! real(
    ($kind:ty) => (
        impl Real for $kind {
            #[inline(always)]
            fn one() -> Self {
                1.0
            }

            #[inline(always)]
            fn zero() -> Self {
                0.0
            }

            #[inline(always)]
            fn from_natural(number: usize) -> Self {
                number as $kind
            }
        }
    );
);

real!(f32);
real!(f64);

macro_rules! to_real(
    ($kind:ty, $real:ty) => (
        impl ToReal<$real> for $kind {
            #[inline(always)]
            fn to_real(&self) -> $real {
                *self as $real
            }
        }
    );
);

to_real!(u8, f32);
to_real!(u8, f64);

to_real!(u16, f32);
to_real!(u16, f64);

to_real!(u32, f32);
to_real!(u32, f64);

to_real!(u64, f32);
to_real!(u64, f64);

to_real!(i8, f32);
to_real!(i8, f64);

to_real!(i16, f32);
to_real!(i16, f64);

to_real!(i32, f32);
to_real!(i32, f64);

to_real!(i64, f32);
to_real!(i64, f64);

to_real!(f32, f32);
to_real!(f64, f64);

to_real!(isize, f32);
to_real!(isize, f64);

to_real!(usize, f32);
to_real!(usize, f64);
