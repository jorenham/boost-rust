//! boost/math/special_functions/sqrt1pm1.hpp

use crate::ffi;

/// Computes `sqrt(1 + x) - 1`
///
/// This function is useful when you need the difference between `sqrt(x)` and `1`, when `x` is
/// itself close to `1`.
///
/// See [`boost::math::ccmath::sqrt`](crate::math::ccmath::sqrt) for the square root.
///
/// Corresponds to `boost::math::sqrt1pm1(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/sqrt1pm1.html>
pub fn sqrt1pm1(x: f64) -> f64 {
    unsafe { ffi::math_sqrt1pm1(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::sqrt1pm1;

    #[test]
    fn test_sqrt1pm1() {
        assert!(sqrt1pm1(f64::NAN).is_nan());
        assert!(sqrt1pm1(-2.0).is_nan());
        assert_eq!(sqrt1pm1(-1.0), -1.0);
        assert_eq!(sqrt1pm1(0.0), 0.0);
        assert_eq!(sqrt1pm1(3.0), 1.0);
        assert_eq!(sqrt1pm1(f64::INFINITY), f64::INFINITY);
    }
}
