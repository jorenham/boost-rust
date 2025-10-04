//! boost/math/special_functions/rsqrt.hpp
//! boost/math/special_functions/sqrt1pm1.hpp

use crate::ffi;

/// Square root *√x*
///
/// Corresponds to `std::sqrt` in C++.
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

/// Computes `sqrt(1 + x) - 1`
///
/// This function is useful when you need the difference between `sqrt(x)` and `1`, when `x` is
/// itself close to `1`.
///
/// Corresponds to `boost::math::sqrt1pm1(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/sqrt1pm1.html>
pub fn sqrt1pm1(x: f64) -> f64 {
    unsafe { ffi::math_sqrt1pm1(x) }
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

    #[test]
    fn test_sqrt1pm1() {
        assert!(sqrt1pm1(NAN).is_nan());
        assert!(sqrt1pm1(-2.0).is_nan());
        assert_eq!(sqrt1pm1(-1.0), -1.0);
        assert_eq!(sqrt1pm1(0.0), 0.0);
        assert_eq!(sqrt1pm1(3.0), 1.0);
        assert_eq!(sqrt1pm1(INF), INF);
    }
}
