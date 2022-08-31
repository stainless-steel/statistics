use std::ops::{Add, Div, Mul, Sub};

/// A real number.
pub trait Real: Copy + Add<Output=Self> + Div<Output=Self> + Mul<Output=Self> + Sub<Output=Self> {
    /// Return the unity.
    fn one() -> Self;

    /// Return the zero.
    fn zero() -> Self;

    /// Create a real number from a natural one.
    fn from_natural(usize) -> Self;

    /// Wrapper for sqrt on the proper fp type
    fn sqrt(number: Self) -> Self;
}

/// A means of converting an arbitrary quantity to a real number.
pub trait ToReal<T: Real> {
    /// Convert to a real number.
    fn to_real(&self) -> T;
}

macro_rules! implement {
    ($($kind:ty),*) => (
        $(
            impl Real for $kind {
                #[inline(always)]
                fn one() -> Self { 1.0 }

                #[inline(always)]
                fn zero() -> Self { 0.0 }

                #[inline(always)]
                fn from_natural(number: usize) -> Self { number as $kind }

                #[inline(always)]
                fn sqrt(number: Self) -> Self { number.sqrt() }
            }
        )*
    );
}

implement!(f32, f64);

macro_rules! implement {
    ($($kind:ty => $real:ty,)*) => (
        $(
            impl ToReal<$real> for $kind {
                #[inline(always)]
                fn to_real(&self) -> $real { *self as $real }
            }
        )*
    );
}

implement! {
    u8 => f32,
    u8 => f64,

    u16 => f32,
    u16 => f64,

    u32 => f32,
    u32 => f64,

    u64 => f32,
    u64 => f64,

    i8 => f32,
    i8 => f64,

    i16 => f32,
    i16 => f64,

    i32 => f32,
    i32 => f64,

    i64 => f32,
    i64 => f64,

    f32 => f32,
    f64 => f64,

    isize => f32,
    isize => f64,

    usize => f32,
    usize => f64,
}
