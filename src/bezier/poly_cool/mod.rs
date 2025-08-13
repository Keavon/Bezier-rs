// This file is inlined from https://github.com/jneem/poly-cool with permission of its author, Joe Neeman.

//! This is a crate for numerical polynomial root-finding.
//!
//! Currently, we implement a single solver: Yuksel's iterative algorithm
//! for finding roots in a bounded interval. We aspire to have
//! more, with thorough tests and benchmarks.

mod cubic;
mod poly;
mod quadratic;

// Cubic and Quadratic are used in benches so they have to be public.
// We haven't actually put any thought into their API yet, though.
#[doc(hidden)]
pub use cubic::Cubic;
#[doc(hidden)]
pub use quadratic::Quadratic;

pub use poly::Poly;

trait TerminationCondition {
    fn stop(&self, last_step: f64, value: f64) -> bool;
}

struct InputError(f64);

impl TerminationCondition for InputError {
    fn stop(&self, last_step: f64, _value: f64) -> bool {
        last_step.abs() <= self.0
    }
}

struct ValueError(f64);

impl TerminationCondition for ValueError {
    fn stop(&self, _last_step: f64, value: f64) -> bool {
        value.abs() <= self.0
    }
}

fn different_signs(x: f64, y: f64) -> bool {
    (x < 0.0) != (y < 0.0)
}
