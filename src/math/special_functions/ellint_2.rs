//! boost/math/special_functions/ellint_2.hpp

use crate::ffi;

/// Complete elliptic integral of the second kind *E(k)*
///
/// Requires that *|k| < 1*.
///
/// Corresponds to `boost::math::ellint_2(k)` in C++.
/// <https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_2.html>
pub fn ellint_2(k: f64) -> f64 {
    unsafe { ffi::math_ellint_2(k) }
}

/// Incomplete elliptic integral of the second kind *E(φ,k)*
///
/// Requires that *k<sup>2</sup> sin<sup>2</sup>(φ) < 1*.
///
/// Corresponds to `boost::math::ellint_2(k, phi)` in C++.
/// <https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_2.html>
pub fn ellint_2_inc(k: f64, phi: f64) -> f64 {
    unsafe { ffi::math_ellint_2_inc(k, phi) }
}

#[cfg(test)]
mod smoketests {
    use crate::math::{ellint_2, ellint_2_inc};

    #[test]
    fn test_ellint_2() {
        assert!(ellint_2(0.5) > 0.0);
    }

    #[test]
    fn test_ellint_2_inc() {
        assert!(ellint_2_inc(0.5, 0.3) > 0.0);
    }
}
