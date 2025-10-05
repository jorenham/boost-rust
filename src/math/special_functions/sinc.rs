//! boost/math/special_functions/sin_pi.hpp

use crate::ffi;

/// Computes *sin(x) / x*
///
/// See also [`sinhc_pi`][crate::math::sinhc_pi].
///
/// Corresponds to Boost's `boost::math::sinc_pi`.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sinc/sinc_pi.html>
pub fn sinc_pi(x: f64) -> f64 {
    unsafe { ffi::math_sinc_pi(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::sinc_pi;
    use core::f64::consts::PI;

    const ATOL: f64 = f64::EPSILON;

    #[test]
    fn test_sinc_pi() {
        assert_abs_diff_eq!(sinc_pi(f64::INFINITY), 0.0, epsilon = ATOL);
        assert_abs_diff_eq!(sinc_pi(PI), 0.0, epsilon = ATOL);
        assert_abs_diff_eq!(sinc_pi(0.0), 1.0, epsilon = ATOL);
        assert_abs_diff_eq!(sinc_pi(-PI), 0.0, epsilon = ATOL);
        assert_abs_diff_eq!(sinc_pi(-f64::INFINITY), 0.0, epsilon = ATOL);
    }
}
