// This file is inlined from https://github.com/jneem/poly-cool with permission of its author, Joe Neeman.

use super::{different_signs, InputError, Quadratic, TerminationCondition, ValueError};

#[derive(Debug, Copy, Clone)]
pub struct Cubic {
    pub c0: f64,
    pub c1: f64,
    pub c2: f64,
    pub c3: f64,
}

impl std::ops::Div<f64> for Cubic {
    type Output = Cubic;

    fn div(self, rhs: f64) -> Cubic {
        Cubic {
            c0: self.c0 / rhs,
            c1: self.c1 / rhs,
            c2: self.c2 / rhs,
            c3: self.c3 / rhs,
        }
    }
}

impl std::ops::Mul<f64> for Cubic {
    type Output = Cubic;

    fn mul(self, rhs: f64) -> Cubic {
        Cubic {
            c0: self.c0 * rhs,
            c1: self.c1 * rhs,
            c2: self.c2 * rhs,
            c3: self.c3 * rhs,
        }
    }
}

impl Cubic {
    pub fn eval(&self, x: f64) -> f64 {
        let xx = x * x;
        let xxx = xx * x;
        self.c0 + self.c1 * x + self.c2 * xx + self.c3 * xxx
    }

    pub fn deriv(&self) -> Quadratic {
        Quadratic {
            c0: self.c1,
            c1: 2.0 * self.c2,
            c2: 3.0 * self.c3,
        }
    }

    pub fn max_coeff(&self) -> f64 {
        self.c0
            .abs()
            .max(self.c1.abs())
            .max(self.c2.abs())
            .max(self.c3.abs())
    }

    fn deflate(&self, root: f64) -> Quadratic {
        let a = self.c3;
        let b = self.c2 + root * a;
        let c = self.c1 + root * b;
        Quadratic {
            c2: a,
            c1: b,
            c0: c,
        }
    }

    /// Computes the critical points of this cubic, as long
    /// as the discriminant of the derivative is positive.
    /// The return values are in increasing order.
    ///
    /// Some corner cases worth noting:
    ///   - If the discriminant is zero, returns nothing. That is,
    ///     we don't find double-roots of the derivative.
    ///   - If the derivative is linear or close to it, we might
    ///     return +/- infinity as one of the roots.
    ///   - Unless some input is NaN, we don't return NaN.
    fn critical_points(&self) -> Option<(f64, f64)> {
        let a = 3.0 * self.c3;
        let b_2 = self.c2;
        let c = self.c1;
        let disc_4 = b_2 * b_2 - a * c;

        if !disc_4.is_finite() {
            return self.rescaled_critical_points();
        }

        if disc_4 > 0.0 {
            let q = -(b_2 + disc_4.sqrt().copysign(b_2));
            let r0 = q / a;
            let r1 = c / q;
            Some((r0.min(r1), r0.max(r1)))
        } else {
            None
        }
    }

    #[cold]
    fn rescaled_critical_points(&self) -> Option<(f64, f64)> {
        let scale = 2.0f64.powi(-515);
        (*self * scale).critical_points()
    }

    fn one_root<Term: TerminationCondition>(
        &self,
        mut lower: f64,
        mut upper: f64,
        term: Term,
    ) -> f64 {
        let val_lower = self.eval(lower);
        let val_upper = self.eval(upper);
        if !val_lower.is_finite() || !val_upper.is_finite() || !self.deriv().is_finite() {
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

    fn one_root_precomputed<Term: TerminationCondition>(
        &self,
        mut lower: f64,
        mut upper: f64,
        val_lower: f64,
        val_upper: f64,
        term: Term,
    ) -> f64 {
        if !val_lower.is_finite() || !val_upper.is_finite() || !self.deriv().is_finite() {
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

    // This has to be pub because we're benchmarking it right now.
    #[doc(hidden)]
    pub fn root_between_with_output_error(self, lower: f64, upper: f64, y_error: f64) -> f64 {
        self.one_root(lower, upper, ValueError(y_error))
    }

    // This has to be pub because we're benchmarking it right now.
    #[doc(hidden)]
    pub fn root_between(self, lower: f64, upper: f64, x_error: f64) -> f64 {
        self.one_root(lower, upper, InputError(x_error))
    }

    fn first_root<Term: TerminationCondition>(
        self,
        lower: f64,
        upper: f64,
        term: Term,
    ) -> Option<f64> {
        if let Some((x0, x1)) = self.critical_points() {
            let possible_endpoints: [f64; 3] = [x0, x1, upper];
            let mut last = lower;
            let mut last_val = self.eval(last);
            for x in possible_endpoints {
                if x > last && x <= upper {
                    let val = self.eval(x);
                    if different_signs(last_val, val) {
                        return Some(self.one_root_precomputed(last, x, last_val, val, term));
                    }

                    last = x;
                    last_val = val;
                }
            }
            None
        } else {
            let lower_val = self.eval(lower);
            let upper_val = self.eval(upper);
            if different_signs(lower_val, upper_val) {
                Some(self.one_root_precomputed(lower, upper, lower_val, upper_val, term))
            } else {
                None
            }
        }
    }

    fn all_roots_term<Term: TerminationCondition>(
        self,
        lower: f64,
        upper: f64,
        term: Term,
    ) -> Vec<f64> {
        let mut ret = Vec::new();
        if let Some(r) = self.first_root(lower, upper, term) {
            ret.push(r);
            let quad = self.deflate(r);
            if let Some((x0, x1)) = quad.positive_discriminant_roots() {
                if lower <= x0 && x0 <= upper {
                    ret.push(x0);
                }
                if lower <= x1 && x1 <= upper {
                    ret.push(x1);
                }
            }
        }
        ret
    }

    /// Computes all roots between `lower` and `upper`, to the desired accuracy.
    ///
    /// We make no guarantees about multiplicity. In fact, if there's a
    /// double-root that isn't a triple-root (and therefore has no sign change
    /// nearby) then there's a good chance we miss it altogether. This is
    /// fine if you're using this root-finding to optimize a quartic, because
    /// double-roots of the derivative aren't local extrema.
    pub fn roots_between(self, lower: f64, upper: f64, x_error: f64) -> Vec<f64> {
        self.all_roots_term(lower, upper, InputError(x_error))
    }

    /// Computes all roots between `lower` and `upper`, to the desired accuracy.
    ///
    /// "Accuracy" is measured with respect to the cubic's value: if this cubic
    /// is called `f` and we find some `x` with `|f(x)| < accuracy` (and `x` is
    /// contained between two endpoints where `f` has opposite signs) then we'll
    /// call `x` a root.
    ///
    /// We make no guarantees about multiplicity. In fact, if there's a
    /// double-root that isn't a triple-root (and therefore has no sign change
    /// nearby) then there's a good chance we miss it altogether. This is
    /// fine if you're using this root-finding to optimize a quartic, because
    /// double-roots of the derivative aren't local extrema.
    pub fn roots_between_with_output_error(self, lower: f64, upper: f64, y_error: f64) -> Vec<f64> {
        self.all_roots_term(lower, upper, ValueError(y_error))
    }

    /// Computes all roots between `lower` and `upper`, to the desired accuracy.
    ///
    /// "Accuracy" is measured with respect to the cubic's value: if this cubic
    /// is called `f` and we find some `x` with `|f(x)| < accuracy` (and `x` is
    /// contained between two endpoints where `f` has opposite signs) then we'll
    /// call `x` a root.
    ///
    /// We make no guarantees about multiplicity. In fact, if there's a
    /// double-root that isn't a triple-root (and therefore has no sign change
    /// nearby) then there's a good chance we miss it altogether. This is
    /// fine if you're using this root-finding to optimize a quartic, because
    /// double-roots of the derivative aren't local extrema.
    // This has to be pub because we're benchmarking it right now.
    //
    // We don't really want this public because it appears to be slower than
    // the other method. This one does a Newton search for each bracketing
    // interval, while the other one just does a single Newton search
    // and then deflates to find the other two roots.
    #[doc(hidden)]
    pub fn roots_between_with_output_error_multiple_searches(
        self,
        lower: f64,
        upper: f64,
        y_error: f64,
    ) -> Vec<f64> {
        let mut possible_endpoints = Vec::<f64>::new();
        if let Some((x0, x1)) = self.critical_points() {
            possible_endpoints.push(x0);
            possible_endpoints.push(x1);
        }
        possible_endpoints.push(upper);

        let mut last = lower;
        let mut last_val = self.eval(last);
        let mut ret = Vec::new();

        for x in possible_endpoints {
            if x > last && x <= upper {
                let val = self.eval(x);
                if different_signs(last_val, val) {
                    ret.push(self.root_between_with_output_error(last, x, y_error));
                }

                last = x;
                last_val = val;
            }
        }
        ret
    }

    #[cold]
    fn roots_blinn_renormalized(&self) -> Vec<f64> {
        if !self.max_coeff().is_finite() {
            Vec::new()
        } else {
            (*self / 2.0f64.powi(128)).roots_blinn()
        }
    }

    pub fn roots_blinn(&self) -> Vec<f64> {
        let mut ret = Vec::new();
        let a = self.c3;
        let b = self.c2 * (1.0 / 3.0);
        let c = self.c1 * (1.0 / 3.0);
        let d = self.c0;

        let delta_1 = a * c - b * b;
        let delta_2 = a * d - b * c;
        let delta_3 = b * d - c * c;
        let disc = 4.0 * delta_1 * delta_3 - delta_2 * delta_2;

        // TODO: what about disc = 0?
        if disc < 0.0 {
            dbg!(disc);
            let (tilde_a, tilde_c, tilde_d) = if b * b * b * d >= a * c * c * c {
                (a, delta_1, -2.0 * b * delta_1 + a * delta_2)
            } else {
                (d, delta_3, -d * delta_2 + 2.0 * c * delta_3)
            };
            let t_0 = -tilde_a.copysign(tilde_d) * (-disc).sqrt();
            let t_1 = -tilde_d + t_0;
            let p = (t_1 / 2.0).cbrt();

            let q = if t_0 == t_1 { -p } else { -tilde_c / p };
            let tilde_x = if tilde_c <= 0.0 {
                p + q
            } else {
                -tilde_d / (p * p + q * q + tilde_c)
            };

            let (x, w) = if b * b * b * d >= a * c * c * c {
                (tilde_x - b, a)
            } else {
                (-d, tilde_x + c)
            };

            if !x.is_finite() || !w.is_finite() {
                return self.roots_blinn_renormalized();
            }

            ret.push(x / w);
        } else {
            dbg!(disc);
            fn one_root(a_or_d: f64, disc: f64, bar_c: f64, bar_d: f64) -> (f64, f64) {
                let sqrt_c = (-bar_c).sqrt();
                let theta = (1.0 / 3.0) * (a_or_d * disc.sqrt()).atan2(-bar_d).abs();
                let (sin_theta, cos_theta) = theta.sin_cos();
                dbg!(theta, cos_theta);
                let tilde_x_1 = 2.0 * sqrt_c * cos_theta;
                let tilde_x_3 = sqrt_c * (-theta.cos() - 3.0f64.sqrt() * sin_theta);
                (tilde_x_1, tilde_x_3)
            }

            let bar_c_a = delta_1;
            let bar_d_a = -2.0 * b * delta_1 + a * delta_2;
            let (tilde_x_1_a, tilde_x_3_a) = one_root(a, disc, bar_c_a, bar_d_a);

            let bar_c_d = delta_3;
            let bar_d_d = -d * delta_2 + 2.0 * c * delta_3;
            let (tilde_x_1_d, tilde_x_3_d) = one_root(d, disc, bar_c_d, bar_d_d);

            let tilde_x_l = if tilde_x_1_a + tilde_x_3_a > 2.0 * b {
                tilde_x_1_a
            } else {
                tilde_x_3_a
            };
            let tilde_x_s = if tilde_x_1_d + tilde_x_3_d < 2.0 * c {
                tilde_x_1_d
            } else {
                tilde_x_3_d
            };

            let (x_l, w_l) = (tilde_x_l - b, a);
            let (x_s, w_s) = (-d, tilde_x_s + c);

            let e = w_l * w_s;
            let f = -x_l * w_s - w_l * x_s;
            let g = x_l * x_s;

            let (x_m, w_m) = (c * f - b * g, c * e - b * f);

            // TODO: check finiteness
            ret.push(x_l / w_l);
            ret.push(x_s / w_s);
            ret.push(x_m / w_m);
        }
        ret
    }
}
