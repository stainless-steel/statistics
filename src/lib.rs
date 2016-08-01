//! Statistics toolbox.

#[cfg(test)]
extern crate assert;

mod moment;
mod real;

pub use moment::{mean, variance};
pub use real::{Real, ToReal};
