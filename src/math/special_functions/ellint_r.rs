//! Carlson Elliptic Integrals
//! boost/math/special_functions/ellint_rc.hpp
//! boost/math/special_functions/ellint_rd.hpp
//! boost/math/special_functions/ellint_rf.hpp
//! boost/math/special_functions/ellint_rg.hpp
//! boost/math/special_functions/ellint_rj.hpp

use crate::ffi;

/// Carlson's elliptic integral *R<sub>C</sub>(x,y)*.
///
/// Requires that *x > 0* and *y ≠ 0*.
///
/// Corresponds to `boost::math::ellint_rc` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_carlson.html>
pub fn ellint_rc(x: f64, y: f64) -> f64 {
    unsafe { ffi::math_ellint_rc(x, y) }
}

/// Carlson's elliptic integral *R<sub>D</sub>(x,y,z)*.
///
/// Requires that *x,y,z ≥ 0* and *x+y > 0*.
///
/// Corresponds to `boost::math::ellint_rd` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_carlson.html>
pub fn ellint_rd(x: f64, y: f64, z: f64) -> f64 {
    unsafe { ffi::math_ellint_rd(x, y, z) }
}

/// Carlson's elliptic integral *R<sub>F</sub>(x,y,z)*.
///
/// Requires that *x,y,z ≥ 0* and at most one of *x,y,z* is zero.
///
/// Corresponds to `boost::math::ellint_rf` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_carlson.html>
pub fn ellint_rf(x: f64, y: f64, z: f64) -> f64 {
    unsafe { ffi::math_ellint_rf(x, y, z) }
}

/// Carlson's elliptic integral *R<sub>G</sub>(x,y,z)*.
///
/// Requires that *x,y ≥ 0*.
///
/// Corresponds to `boost::math::ellint_rg` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_carlson.html>
pub fn ellint_rg(x: f64, y: f64, z: f64) -> f64 {
    unsafe { ffi::math_ellint_rg(x, y, z) }
}

/// Carlson's elliptic integral *R<sub>J</sub>(x,y,z,p)*.
///
/// Requires that *x,y,z ≥ 0*, *p>0*, and at most one of *x,y,z* is zero.
///
/// Corresponds to `boost::math::ellint_rj` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/ellint_carlson.html>
pub fn ellint_rj(x: f64, y: f64, z: f64, p: f64) -> f64 {
    unsafe { ffi::math_ellint_rj(x, y, z, p) }
}

#[cfg(test)]
mod smoketests {
    use crate::math::{ellint_rc, ellint_rd, ellint_rf, ellint_rg, ellint_rj};

    #[test]
    fn test_ellint_rc() {
        assert!(ellint_rc(0.0, 0.0).is_nan());
        assert!(ellint_rc(1.0, 2.0).is_finite());
    }

    #[test]
    fn test_ellint_rd() {
        assert!(ellint_rd(0.0, 0.0, 0.0).is_nan());
        assert!(ellint_rd(1.0, 2.0, 3.0).is_finite());
    }

    #[test]
    fn test_ellint_rf() {
        assert!(ellint_rf(0.0, 0.0, 0.0).is_nan());
        assert!(ellint_rf(1.0, 2.0, 3.0).is_finite());
    }

    #[test]
    fn test_ellint_rg() {
        assert!(ellint_rg(-1.0, 0.0, 0.0).is_nan());
        assert!(ellint_rg(0.0, -1.0, 0.0).is_nan());
        assert!(ellint_rg(0.0, 0.0, 0.0).is_finite());
        assert!(ellint_rg(1.0, 2.0, 3.0).is_finite());
    }

    #[test]
    fn test_ellint_rj() {
        assert!(ellint_rj(0.0, 0.0, 0.0, 0.0).is_nan());
        assert!(ellint_rj(1.0, 2.0, 3.0, 4.0).is_finite());
    }
}
