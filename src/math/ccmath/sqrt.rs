//! boost/math/ccmath/sqrt.hpp

use crate::ffi;

/// Square root *√x*
///
/// Corresponds to `boost::math::ccmath::sqrt(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ccmath.html>
///
/// # See Also:
/// - [`boost::math::rsqrt`](crate::math::rsqrt): `1 / sqrt(x)`
/// - [`boost::math::sqrt1pm1`](crate::math::sqrt1pm1): `sqrt(1 + x) - 1`
pub fn sqrt(x: f64) -> f64 {
    unsafe { ffi::math_ccmath_sqrt(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::ccmath::sqrt;

    #[test]
    fn test_sqrt() {
        assert!(sqrt(f64::NAN).is_nan());
        assert!(sqrt(-1.0).is_nan());
        assert_eq!(sqrt(1.0), 1.0);
        assert_eq!(sqrt(4.0), 2.0);
        assert_eq!(sqrt(0.0), 0.0);
        assert_eq!(sqrt(f64::INFINITY), f64::INFINITY);
    }
}
