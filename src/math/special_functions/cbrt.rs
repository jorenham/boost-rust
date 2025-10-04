//! boost/math/special_functions/cbrt.hpp

use crate::ffi;

/// Cube root *∛x*
///
/// Implemented using Halley iteration.
///
/// Corresponds to `boost::math::cbrt(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/cbrt.html>
pub fn cbrt(x: f64) -> f64 {
    unsafe { ffi::math_cbrt(x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cbrt() {
        assert!(cbrt(f64::NAN).is_nan());
        assert_eq!(cbrt(f64::INFINITY), f64::INFINITY);
        assert_eq!(cbrt(f64::NEG_INFINITY), f64::NEG_INFINITY);
        assert_eq!(cbrt(1.0), 1.0);
        assert_eq!(cbrt(-1.0), -1.0);
        assert_eq!(cbrt(27.0), 3.0);
        assert_eq!(cbrt(-27.0), -3.0);
    }
}
