//! boost/math/special_functions/ellint_d.hpp

use crate::ffi;

/// Complete elliptic integral *D(k)*
///
/// Requires that *|k| < 1*.
///
/// Corresponds to `boost::math::ellint_d(k)` in C++.
/// <https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_d.html>
pub fn ellint_d(k: f64) -> f64 {
    unsafe { ffi::math_ellint_d(k) }
}

/// Incomplete elliptic integral *D(φ,k)*
///
/// Requires that *k<sup>2</sup> sin<sup>2</sup>(φ) < 1*.
///
/// Corresponds to `boost::math::ellint_d(k, phi)` in C++.
/// <https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_d.html>
pub fn ellint_d_inc(k: f64, phi: f64) -> f64 {
    unsafe { ffi::math_ellint_d_inc(k, phi) }
}

#[cfg(test)]
mod smoketests {
    use crate::math::{ellint_d, ellint_d_inc};

    #[test]
    fn test_ellint_d() {
        assert!(ellint_d(0.5) > 0.0);
    }

    #[test]
    fn test_ellint_d_inc() {
        assert!(ellint_d_inc(0.5, 0.3) > 0.0);
    }
}
