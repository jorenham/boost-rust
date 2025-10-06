//! boost/math/special_functions/ellint_3.hpp

use crate::ffi;

/// Complete elliptic integral of the third kind *Π(v,k)*
///
/// Requires that *|k| < 1* and *v < 1*.
///
/// Corresponds to `boost::math::ellint_3(k, v)` in C++.
/// <https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_3.html>
pub fn ellint_3(k: f64, v: f64) -> f64 {
    unsafe { ffi::math_ellint_3(k, v) }
}

/// Incomplete elliptic integral of the third kind *Π(φ,v,k)*
///
/// Requires that *k<sup>2</sup> sin<sup>2</sup>(φ) < 1* and *v sin<sup>2</sup>(φ) < 1*.
///
/// Corresponds to `boost::math::ellint_3(k, v, phi)` in C++.
/// <https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_3.html>
pub fn ellint_3_inc(k: f64, v: f64, phi: f64) -> f64 {
    unsafe { ffi::math_ellint_3_inc(k, v, phi) }
}

#[cfg(test)]
mod smoketests {
    use crate::math::{ellint_3, ellint_3_inc};

    #[test]
    fn test_ellint_3() {
        assert!(ellint_3(0.5, 0.4) > 0.0);
    }

    #[test]
    fn test_ellint_3_inc() {
        assert!(ellint_3_inc(0.5, 0.4, 0.3) > 0.0);
    }
}
