//! boost/math/special_functions/sinhc.hpp

use crate::ffi;

/// Computes *sinh(x) / x*
///
/// See also [`sinc_pi`][crate::math::sinc_pi].
///
/// Corresponds to Boost's `boost::math::sinhc_pi`.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sinc/sinhc_pi.html>
pub fn sinhc_pi(x: f64) -> f64 {
    unsafe { ffi::math_sinhc_pi(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::sinhc_pi;
    use core::f64::consts::E;

    #[test]
    fn test_sinhc_pi() {
        assert_eq!(sinhc_pi(f64::INFINITY), f64::INFINITY);
        assert_eq!(sinhc_pi(1.0), 0.5 * (E - 1.0 / E));
        assert_eq!(sinhc_pi(0.0), 1.0);
        assert_eq!(sinhc_pi(-1.0), 0.5 * (E - 1.0 / E));
        assert_eq!(sinhc_pi(-f64::INFINITY), f64::INFINITY);
    }
}
