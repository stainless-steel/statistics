//! Statistics toolbox.

#[cfg(test)]
extern crate assert;

mod moment;

pub use moment::{mean, variance};
