#![doc = include_str!("../README.md")]

pub(crate) mod compare;

mod bezier;
mod consts;
mod poisson_disk;
mod polynomial;
mod subpath;
mod utils;

pub use bezier::*;
pub use subpath::*;
pub use utils::{Cap, Join, SubpathTValue, TValue, TValueType};

// TODO: Remove this, it is kept for semver backwards compatibility but never should have been public to begin with.
#[deprecated]
#[derive(Debug, Clone)]
pub struct Bezier1d(pub Vec<f64>);
