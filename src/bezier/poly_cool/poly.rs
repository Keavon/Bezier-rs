// This file is inlined from https://github.com/jneem/poly-cool with permission of its author, Joe Neeman.

//! Polynomials of dynamic (run-time) degree.

use super::{different_signs, Cubic, InputError, TerminationCondition};

/// A polynomial of dynamic degree.
///
/// It would be nice to have polynomials of type-level degree,
/// but that's a bit awkward without const generic expressions
/// (e.g. to express the type of the derivative). It could be
/// done with `typenum` and `generic_array`...
#[derive(Clone, Debug)]
pub struct Poly {
    /// Coefficients in increasing order of degree.
    ///
    /// For example, `coeffs[0]` is the constant term.
    coeffs: Vec<f64>,
}

impl<'a> std::ops::Mul<&'a Poly> for &'a Poly {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Poly {
        let mut coeffs = vec![0.0; (self.coeffs.len() + rhs.coeffs.len()).saturating_sub(1)];

        for (i, c) in self.coeffs.iter().enumerate() {
            for (j, d) in rhs.coeffs.iter().enumerate() {
                coeffs[i + j] += c * d;
            }
        }
        Poly { coeffs }
    }
}

impl std::ops::Mul<&Poly> for Poly {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Poly {
        (&self) * rhs
    }
}

impl Poly {
    /// Constructs a new polynomial from coefficients.
    ///
    /// The first coefficient provided will be the constant term, the second will
    /// be the linear term, and so on.
    pub fn new(coeffs: impl IntoIterator<Item = f64>) -> Self {
        Poly {
            coeffs: coeffs.into_iter().collect(),
        }
    }

    fn is_finite(&self) -> bool {
        self.coeffs.iter().all(|c| c.is_finite())
    }

    /// Returns the polynomial that's the derivative of this polynomial.
    pub fn deriv(&self) -> Poly {
        let mut coeffs = Vec::with_capacity(self.coeffs.len() - 1);
        // If we're empty (meaning that we're the constant zero polynomial),
        // this will just return the zero polynomial again: no need for a
        // special case.
        for (i, c) in self.coeffs.iter().enumerate().skip(1) {
            coeffs.push(c * (i as f64));
        }
        Poly { coeffs }
    }

    /// Evaluates this polynomial at a point.
    pub fn eval(&self, x: f64) -> f64 {
        let mut ret = 0.0;
        let mut x_pow = 1.0;
        for &c in &self.coeffs {
            ret += c * x_pow;
            x_pow *= x;
        }
        ret
    }

    /// The degree of this polynomial.
    ///
    /// This function only looks at the *presence* of coefficients, not their
    /// value. If you construct a polynomial with three coefficients, this
    /// method will say that it has degree 2 even if all of those coefficients
    /// are zero.
    ///
    /// A polynomial with no coefficients will give zero as its degree, as will
    /// a polynomial with one coefficient.
    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }

    /// If this polynomial has degree 3 or less, converts it to a [cubic](crate::Cubic).
    fn to_cubic(&self) -> Option<Cubic> {
        if self.degree() <= 3 {
            Some(Cubic {
                c0: self.coeffs.first().copied().unwrap_or(0.0),
                c1: self.coeffs.get(1).copied().unwrap_or(0.0),
                c2: self.coeffs.get(2).copied().unwrap_or(0.0),
                c3: self.coeffs.get(3).copied().unwrap_or(0.0),
            })
        } else {
            None
        }
    }

    fn one_root<Term: TerminationCondition>(
        &self,
        deriv: &Poly,
        mut lower: f64,
        mut upper: f64,
        val_lower: f64,
        val_upper: f64,
        term: Term,
    ) -> f64 {
        if !val_lower.is_finite() || !val_upper.is_finite() || !deriv.is_finite() {
            return f64::NAN;
        }
        debug_assert!(different_signs(val_lower, val_upper));

        let mut x = lower + (upper - lower) / 2.0;
        let mut val_x = self.eval(x);
        let mut step = (upper - lower) / 2.0;

        while x.is_finite() && !term.stop(step, val_x) {
            let root_in_first_half = different_signs(val_lower, val_x);
            if root_in_first_half {
                upper = x;
            } else {
                lower = x;
            }

            let deriv_x = self.deriv().eval(x);
            debug_assert!(deriv_x.is_finite());
            debug_assert!(val_x.is_finite());

            step = -val_x / deriv_x;
            let mut new_x = x + step;

            if new_x <= lower || new_x >= upper {
                new_x = lower + (upper - lower) / 2.0;

                if new_x == upper || new_x == lower {
                    // This should be rare, but it happens if they ask for more
                    // accuracy than is reasonable. For example, suppse (because
                    // of large coefficients) the output value jumps from -1.0
                    // to 1.0 between adjacent floats and they ask for an output
                    // error of smaller than 0.5. Then we'll eventually shrink
                    // the search interval to a pair of adjacent floats and hit
                    // this case.
                    return new_x;
                }
            }
            step = new_x - x;
            x = new_x;
            val_x = self.eval(x);
        }
        x
    }

    /// Finds all the roots in an interval, using Yuksel's algorithm.
    ///
    /// This is a numerical, iterative method. It first constructs critical
    /// points to find bracketing intervals (intervals `[x0, x1]` where
    /// `self.eval(x0)` and `self.eval(x1)` have different signs). Then it uses
    /// a kind of modified Newton method to find a root on each bracketing
    /// interval. It has a few limitations:
    ///
    /// - if there is only a small interval where the polynomial changes sign,
    ///   it can miss roots. For example, when two roots are very close together
    ///   it can miss them both.
    /// - run time is quadratic in the degree. However, it is often very fast
    ///   in practice for polynomials of low degree, especially if the interval
    ///   `[lower, upper]` contains few roots.
    pub fn roots_between(&self, lower: f64, upper: f64, x_error: f64) -> Vec<f64> {
        let mut ret = Vec::new();
        let mut scratch = Vec::new();
        self.roots_between_with_buffer(lower, upper, x_error, &mut ret, &mut scratch);
        ret
    }

    /// Finds all the roots in an interval, using Yuksel's algorithm.
    ///
    /// See [`Poly::roots_between`] for more details. This method differs from that
    /// one in that it performs fewer allocations: you provide an `out` buffer
    /// for the result and a `scratch` buffer for intermediate computations.
    pub fn roots_between_with_buffer(
        &self,
        lower: f64,
        upper: f64,
        x_error: f64,
        out: &mut Vec<f64>,
        scratch: &mut Vec<f64>,
    ) {
        out.clear();
        scratch.clear();

        if let Some(c) = self.to_cubic() {
            out.extend(c.roots_between(lower, upper, x_error));
            return;
        }

        let deriv = self.deriv();
        deriv.roots_between_with_buffer(lower, upper, x_error, scratch, out);
        scratch.push(upper);
        out.clear();
        let mut last = lower;
        let mut last_val = self.eval(last);

        // `scratch` now contains all the critical points (in increasing order)
        // and the upper endpoint of the interval. If we throw away all the
        // critical points that are outside of (lower, upper), the things
        // remaining in `scratch` are the endpoints of the potential bracketing
        // intervals of our polynomial. So by filtering out uninteresting
        // critical points, this loop is iterating over potential bracketing
        // intervals.
        for &mut x in scratch {
            if x > last && x <= upper {
                let val = self.eval(x);
                if different_signs(last_val, val) {
                    out.push(self.one_root(&deriv, last, x, last_val, val, InputError(x_error)));
                }

                last = x;
                last_val = val;
            }
        }
    }
}
