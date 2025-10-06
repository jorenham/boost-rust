//! boost/math/special_functions/ellint_1.hpp

use crate::ffi;

/// Complete elliptic integral of the first kind *K(k)*
///
/// Requires that *|k| < 1*.
///
/// Corresponds to `boost::math::ellint_1(k)` in C++.
/// <https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_1.html>
pub fn ellint_1(k: f64) -> f64 {
    unsafe { ffi::math_ellint_1(k) }
}

/// Incomplete elliptic integral of the first kind *F(φ,k)*
///
/// Requires that *k<sup>2</sup> sin<sup>2</sup>(φ) < 1*.
///
/// Corresponds to `boost::math::ellint_1(k, phi)` in C++.
/// <https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_1.html>
pub fn ellint_1_inc(k: f64, phi: f64) -> f64 {
    unsafe { ffi::math_ellint_1_inc(k, phi) }
}

#[cfg(test)]
mod smoketests {
    use crate::math::{ellint_1, ellint_1_inc};

    #[test]
    fn test_ellint_1() {
        assert!(ellint_1(0.5) > 0.0);
    }

    #[test]
    fn test_ellint_1_inc() {
        assert!(ellint_1_inc(0.5, 0.3) > 0.0);
    }
}
