//! boost/math/special_functions/logsumexp.hpp

use crate::ffi;

/// Log-sum-exp function of two arguments
///
/// *ln(e<sup>x</sup> + e<sup>y</sup>)*
///
/// See [`logsumexp`] for a more general version that accepts multiple arguments.
///
/// Corresponds to `boost::math::logaddexp` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/logaddexp.html>
pub fn logaddexp(x: f64, y: f64) -> f64 {
    unsafe { ffi::math_logaddexp(x, y) }
}

/// Log-sum-exp function for multiple arguments
///
/// *ln(e<sup>x<sub>0</sub></sup> + e<sup>x<sub>1</sub></sup> + ... + e<sup>x<sub>n-1</sub></sup>)*
///
/// See [`logaddexp`] for the two-argument version.
///
/// Corresponds to `boost::math::logsumexp` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/logaddexp.html>
#[doc(alias = "lse")]
pub fn logsumexp(x: &[f64]) -> f64 {
    if x.is_empty() {
        // special casing to avoid undefined behavior in Boost
        f64::NEG_INFINITY
    } else {
        unsafe { ffi::math_logsumexp(x.as_ptr(), x.len()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::LN_2;

    const RTOL: f64 = 2e-15;
    const LN_3: f64 = 1.098_612_288_668_109_8;

    #[test]
    fn test_logaddexp() {
        assert_relative_eq!(logaddexp(0.0, 0.0), LN_2, epsilon = RTOL);
        assert_relative_eq!(logaddexp(1.0, 1.0), LN_2 + 1.0, epsilon = RTOL);
        assert_relative_eq!(logaddexp(-1.0, -1.0), LN_2 - 1.0, epsilon = RTOL);
        assert_relative_eq!(logaddexp(100.0, 100.0), LN_2 + 100.0, epsilon = RTOL);
        assert_relative_eq!(logaddexp(-100.0, -100.0), LN_2 - 100.0, epsilon = RTOL);
        assert_relative_eq!(logaddexp(100.0, -100.0), 100.0, epsilon = RTOL);
        assert_relative_eq!(logaddexp(-100.0, 100.0), 100.0, epsilon = RTOL);
    }

    #[test]
    fn test_logsumexp_0() {
        assert_eq!(logsumexp(&[]), f64::NEG_INFINITY);
    }

    #[test]
    fn test_logsumexp_1() {
        assert_relative_eq!(logsumexp(&[0.0]), 0.0, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[1.0]), 1.0, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[-1.0]), -1.0, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[100.0]), 100.0, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[-100.0]), -100.0, epsilon = RTOL);
    }

    #[test]
    fn test_logsumexp_2() {
        assert_relative_eq!(logsumexp(&[0.0, 0.0]), LN_2, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[1.0, 1.0]), LN_2 + 1.0, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[-1.0, -1.0]), LN_2 - 1.0, epsilon = RTOL);

        assert_relative_eq!(logsumexp(&[100.0, 100.0]), LN_2 + 100.0, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[-100.0, -100.0]), LN_2 - 100.0, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[100.0, -100.0]), 100.0, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[-100.0, 100.0]), 100.0, epsilon = RTOL);
    }

    #[test]
    fn test_logsumexp_3() {
        assert_relative_eq!(logsumexp(&[0.0, 0.0, 0.0]), LN_3, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[1.0, 1.0, 1.0]), LN_3 + 1.0, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[-1.0, -1.0, -1.0]), LN_3 - 1.0, epsilon = RTOL);

        assert_relative_eq!(
            logsumexp(&[100.0, 100.0, 100.0]),
            LN_3 + 100.0,
            epsilon = RTOL
        );
        assert_relative_eq!(
            logsumexp(&[-100.0, -100.0, -100.0]),
            LN_3 - 100.0,
            epsilon = RTOL
        );
        assert_relative_eq!(logsumexp(&[100.0, -100.0, -100.0]), 100.0, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[-100.0, 100.0, -100.0]), 100.0, epsilon = RTOL);
        assert_relative_eq!(logsumexp(&[-100.0, -100.0, 100.0]), 100.0, epsilon = RTOL);
    }
}
