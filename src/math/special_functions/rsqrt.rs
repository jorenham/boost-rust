//! boost/math/special_functions/rsqrt.hpp

use crate::ffi;

/// Square root *√x*
///
/// Corresponds to `boost::math::sqrt(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/rsqrt.html>
pub fn sqrt(x: f64) -> f64 {
    unsafe { ffi::math_sqrt(x) }
}

/// Reciprocal square root *1/√x*
///
/// Corresponds to `boost::math::rsqrt(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/rsqrt.html>
pub fn rsqrt(x: f64) -> f64 {
    unsafe { ffi::math_rsqrt(x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INF: f64 = f64::INFINITY;
    const NAN: f64 = f64::NAN;

    #[test]
    fn test_sqrt() {
        assert!(sqrt(NAN).is_nan());
        assert!(sqrt(-1.0).is_nan());
        assert_eq!(sqrt(1.0), 1.0);
        assert_eq!(sqrt(4.0), 2.0);
        assert_eq!(sqrt(0.0), 0.0);
        assert_eq!(sqrt(INF), INF);
    }

    #[test]
    fn test_rsqrt() {
        assert!(rsqrt(NAN).is_nan());
        assert!(rsqrt(-1.0).is_nan());
        assert_eq!(rsqrt(1.0), 1.0);
        assert_eq!(rsqrt(4.0), 0.5);
        assert_eq!(rsqrt(0.0), INF);
        assert_eq!(rsqrt(INF), 0.0);
    }
}
