//! boost/math/special_functions/polygamma.hpp

use crate::ffi;
use core::ffi::c_int;

/// Polygamma function *𝟊<sup>(n)</sup>(x)*
///
/// For *n > 0*, Polygamma is defined as the *n*<sup>th</sup> derivative of the
/// [`digamma`](crate::math::digamma) function *𝟊(x)*.
///
/// Special cases:
/// - For `n = 0`, this is equivalent to the [`digamma`](crate::math::digamma) function *𝟊(x)*
/// - For `n = 1`, this is equivalent to the [`trigamma`](crate::math::trigamma) function
///   *𝟊<sup>(1)</sup>(x)*
/// - For `n = -1` and `x > 0`, this is equivalent to the [`lgamma`](crate::math::lgamma) function
///   *ln |Γ(x)|*
///
/// Corresponds to `boost::math::polygamma(x)` in C++, with some minor corrections related to
/// infinities and negative integers.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/polygamma.html>.
pub fn polygamma(n: i32, x: f64) -> f64 {
    if n < -1 {
        todo!("polygamma not implemented for n < -1");
    }
    unsafe { ffi::math_polygamma(n as c_int, x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RTOL: f64 = 1e-15;
    const ZETA_2: f64 = 1.644_934_066_848_226_4; // ζ(2) = π²/6

    fn is_pzero(x: f64) -> bool {
        x == 0.0 && x.is_sign_positive()
    }

    fn is_nzero(x: f64) -> bool {
        x == 0.0 && x.is_sign_negative()
    }

    fn is_pinf(x: f64) -> bool {
        x.is_infinite() && x.is_sign_positive()
    }

    #[test]
    fn test_polygamma_0() {
        // Same as digamma tests
        assert_relative_eq!(polygamma(0, 1.461_632_144_968_362_3), 0.0, epsilon = RTOL);
        assert_relative_eq!(polygamma(0, -0.504_083_008_264_455_4), 0.0, epsilon = RTOL);
        assert_relative_eq!(polygamma(0, -1.573_498_473_162_390_5), 0.0, epsilon = RTOL);
    }

    #[test]
    fn test_polygamma_1() {
        // Same as trigamma tests
        assert_abs_diff_eq!(polygamma(1, 1.0), ZETA_2, epsilon = RTOL);
        assert_abs_diff_eq!(polygamma(1, 2.0), ZETA_2 - 1.0, epsilon = RTOL);
        assert_abs_diff_eq!(polygamma(1, 3.0), ZETA_2 - 1.25, epsilon = RTOL);

        assert_abs_diff_eq!(polygamma(1, -1.5), 3.0 * ZETA_2 + 4.0 / 0.9, epsilon = RTOL);
        assert_abs_diff_eq!(polygamma(1, -0.5), 3.0 * ZETA_2 + 4.0, epsilon = RTOL);
        assert_abs_diff_eq!(polygamma(1, 0.5), 3.0 * ZETA_2, epsilon = RTOL);
        assert_abs_diff_eq!(polygamma(1, 1.5), 3.0 * ZETA_2 - 4.0, epsilon = RTOL);
        assert_abs_diff_eq!(polygamma(1, 2.5), 3.0 * ZETA_2 - 4.0 / 0.9, epsilon = RTOL);
    }

    #[test]
    fn test_polygamma_singularities() {
        for n in 0..=3 {
            assert!(polygamma(n, f64::NAN).is_nan());
            assert!(polygamma(n, -f64::INFINITY).is_nan());
        }

        assert!(
            is_pinf(polygamma(0, f64::INFINITY)),
            "{} != +∞",
            polygamma(0, f64::INFINITY)
        ); // +∞
        assert!(
            is_pzero(polygamma(1, f64::INFINITY)),
            "{} != +0.0",
            polygamma(1, f64::INFINITY)
        ); // +0
        assert!(
            is_nzero(polygamma(2, f64::INFINITY)),
            "{} != -0.0",
            polygamma(2, f64::INFINITY)
        ); // -0
        assert!(
            is_pzero(polygamma(3, f64::INFINITY)),
            "{} != +0.0",
            polygamma(3, f64::INFINITY)
        ); // +0

        for x in [0.0, -1.0, -2.0] {
            assert!(polygamma(0, x).is_nan()); // indeterminate
            assert!(is_pinf(polygamma(1, x))); // +∞
            assert!(polygamma(2, 0.0).is_nan()); // indeterminate
            assert!(is_pinf(polygamma(3, x))); // +∞
        }
    }
}
