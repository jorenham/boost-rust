//! boost/math/special_functions/expm1.hpp

use crate::ffi;

/// Computes *e<sup>x</sup> - 1*
///
/// See [`log1p`](crate::math::log1p) for the inverse function.
///
/// Corresponds to `boost::math::expm1(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/expm1.html>
pub fn expm1(x: f64) -> f64 {
    unsafe { ffi::math_expm1(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::expm1;
    use core::f64::consts;

    #[test]
    fn test_expm1() {
        assert!(expm1(f64::NAN).is_nan());
        assert_eq!(expm1(f64::INFINITY), f64::INFINITY);
        assert_eq!(expm1(f64::NEG_INFINITY), -1.0);
        assert_eq!(expm1(0.0), 0.0);
        assert_relative_eq!(expm1(-1e-15), -1e-15);
        assert_relative_eq!(expm1(1e-15), 1e-15);
        assert_relative_eq!(expm1(1.0), consts::E - 1.0);
    }
}
