//! boost/math/special_functions/log1p.hpp

use crate::ffi;

/// Computes *ln(x + 1)*
///
/// See [`expm1`](crate::math::expm1) for the inverse function.
///
/// Corresponds to `boost::math::log1p(x, y)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/log1p.html>
pub fn log1p(x: f64) -> f64 {
    unsafe { ffi::math_log1p(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::log1p;
    use core::f64::consts;

    #[test]
    fn test_log1p() {
        assert!(log1p(f64::NAN).is_nan());
        assert_eq!(log1p(f64::INFINITY), f64::INFINITY);
        assert_eq!(log1p(-1.0), f64::NEG_INFINITY);
        assert!(log1p(-2.0).is_nan());
        assert_eq!(log1p(0.0), 0.0);
        assert_relative_eq!(log1p(1e-15), 1e-15);
        assert_relative_eq!(log1p(-1e-15), -1e-15);
        assert_relative_eq!(log1p(1.0), consts::LN_2);
    }
}
