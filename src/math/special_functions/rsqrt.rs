//! boost/math/special_functions/rsqrt.hpp

use crate::ffi;

/// Reciprocal square root *1/√x*
///
/// See [`boost::math::ccmath::sqrt`](crate::math::ccmath::sqrt) for the square root.
///
/// Corresponds to `boost::math::rsqrt(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/rsqrt.html>
pub fn rsqrt(x: f64) -> f64 {
    unsafe { ffi::math_rsqrt(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::rsqrt;

    #[test]
    fn test_rsqrt() {
        assert!(rsqrt(f64::NAN).is_nan());
        assert!(rsqrt(-1.0).is_nan());
        assert_eq!(rsqrt(1.0), 1.0);
        assert_eq!(rsqrt(4.0), 0.5);
        assert_eq!(rsqrt(0.0), f64::INFINITY);
        assert_eq!(rsqrt(f64::INFINITY), 0.0);
    }
}
