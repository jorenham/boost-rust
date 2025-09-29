//! boost/math/special_functions/erf.hpp

use crate::ffi;

/// Error function
///
/// Corresponds to `boost::math::erf(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_erf/error_function.html>
pub fn erf(x: f64) -> f64 {
    unsafe { ffi::math_erf(x) }
}

/// Complement of the error function
///
/// Corresponds to `boost::math::erfc(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_erf/error_function.html>
pub fn erfc(x: f64) -> f64 {
    unsafe { ffi::math_erfc(x) }
}

/// Inverse of [`erf`]
///
/// Corresponds to `boost::math::erf_inv(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_erf/error_inv.html>
pub fn erf_inv(p: f64) -> f64 {
    unsafe { ffi::math_erf_inv(p) }
}

/// Inverse of [`erfc`]
///
/// Corresponds to `boost::math::erfc_inv(q)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_erf/error_inv.html>
pub fn erfc_inv(q: f64) -> f64 {
    unsafe { ffi::math_erfc_inv(q) }
}

#[cfg(test)]
mod smoketests {
    use super::*;

    #[test]
    fn test_erf() {
        assert!(erf(1e-9).is_finite());
        assert!(erf(1.0 - 1e-9).is_finite());
    }

    #[test]
    fn test_erfc() {
        assert!(erfc(1e-9).is_finite());
        assert!(erfc(1.0 - 1e-9).is_finite());
    }
    #[test]
    fn test_erf_inv() {
        assert!(erf_inv(1e-9).is_finite());
        assert!(erf_inv(1.0 - 1e-9).is_finite());
    }

    #[test]
    fn test_erfc_inv() {
        assert!(erfc_inv(1e-9).is_finite());
        assert!(erfc_inv(1.0 - 1e-9).is_finite());
    }
}
