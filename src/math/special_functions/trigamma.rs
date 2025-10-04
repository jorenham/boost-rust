//! boost/math/special_functions/trigamma.hpp

use crate::ffi;

/// Trigamma function *𝟊<sup>(1)</sup>(x)*
///
/// Trigamma is defined as the derivative of the [`digamma`](crate::math::digamma) function *𝟊(x)*,
/// and a special case of the [`polygamma`](crate::math::polygamma) function *𝟊<sup>(n)</sup>(x)*
/// for *n = 1*.
///
/// Corresponds to `boost::math::trigamma(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/trigamma.html>
pub fn trigamma(x: f64) -> f64 {
    unsafe { ffi::math_trigamma(x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RTOL: f64 = 1e-15;
    const ZETA_2: f64 = 1.644_934_066_848_226_4; // ζ(2) = π²/6

    #[test]
    fn test_trigamma() {
        assert_abs_diff_eq!(trigamma(1.0), ZETA_2, epsilon = RTOL);
        assert_abs_diff_eq!(trigamma(2.0), ZETA_2 - 1.0, epsilon = RTOL);
        assert_abs_diff_eq!(trigamma(3.0), ZETA_2 - 1.25, epsilon = RTOL);

        assert_abs_diff_eq!(trigamma(-1.5), 3.0 * ZETA_2 + 4.0 / 0.9, epsilon = RTOL);
        assert_abs_diff_eq!(trigamma(-0.5), 3.0 * ZETA_2 + 4.0, epsilon = RTOL);
        assert_abs_diff_eq!(trigamma(0.5), 3.0 * ZETA_2, epsilon = RTOL);
        assert_abs_diff_eq!(trigamma(1.5), 3.0 * ZETA_2 - 4.0, epsilon = RTOL);
        assert_abs_diff_eq!(trigamma(2.5), 3.0 * ZETA_2 - 4.0 / 0.9, epsilon = RTOL);
    }

    #[test]
    fn test_trigamma_singularities() {
        assert!(trigamma(f64::NAN).is_nan());
        assert_eq!(trigamma(f64::INFINITY), 0.0);
        assert_eq!(trigamma(0.0), f64::INFINITY);
        assert_eq!(trigamma(-1.0), f64::INFINITY);
        assert_eq!(trigamma(-2.0), f64::INFINITY);
        assert!(trigamma(-f64::INFINITY).is_nan());
    }
}
