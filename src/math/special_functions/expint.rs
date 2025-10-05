//! boost/math/special_functions/expint.hpp

use crate::ffi;

/// Exponential integral *Ei(x)*.
///
/// Corresponds to `boost::math::expint(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/expint/expint_i.html>
pub fn expint_i(x: f64) -> f64 {
    unsafe { ffi::math_expint_i(x) }
}

/// Generalized exponential integral *E<sub>n</sub>(x)*.
///
/// Corresponds to `boost::math::expint(n, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/expint/expint_n.html>
pub fn expint_n(n: u32, x: f64) -> f64 {
    unsafe { ffi::math_expint_n(n, x) }
}

#[cfg(test)]
mod tests {
    use crate::math::{expint_i, expint_n};
    use core::f64::consts::E;

    #[test]
    fn test_expint_i() {
        assert_relative_eq!(expint_i(1.0), 1.895_117_816_355_936_8);
        assert_relative_eq!(expint_i(-1.0), -0.219_383_934_395_520_29);
        assert_relative_eq!(expint_i(0.0), f64::NEG_INFINITY);
        assert_relative_eq!(expint_i(f64::NEG_INFINITY), 0.0);
        assert_relative_eq!(expint_i(f64::INFINITY), f64::INFINITY);
    }

    #[test]
    fn test_expint_n() {
        assert_relative_eq!(expint_n(0, 1.0), 1.0 / E);
        assert_relative_eq!(expint_n(0, 2.0), 1.0 / (2.0 * E * E));
        assert_relative_eq!(expint_n(0, f64::INFINITY), 0.0);
        assert!(expint_n(0, f64::NEG_INFINITY).is_nan());
    }
}
