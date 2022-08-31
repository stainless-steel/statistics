//! Statistics toolbox.

#[cfg(test)]
extern crate assert;

mod moment;
mod real;

pub use moment::{mean, variance, stddev};
pub use real::{Real, ToReal};
