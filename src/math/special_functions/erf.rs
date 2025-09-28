//! boost/math/special_functions/erf.hpp

use crate::ffi;

/// Error function *erf(x)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_erf/error_function.html>
pub fn erf(x: f64) -> f64 {
    unsafe { ffi::boost_math_erf(x) }
}

/// Complementary error function *erfc(x)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_erf/error_function.html>
pub fn erfc(x: f64) -> f64 {
    unsafe { ffi::boost_math_erfc(x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erf() {
        assert_abs_diff_eq!(erf(0.0), 0.0, epsilon = f64::EPSILON);
        assert_abs_diff_eq!(erf(-1.0) + erf(1.0), 0.0, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_erfc() {
        assert_abs_diff_eq!(erfc(0.0), 1.0, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_erf_erfc_relation() {
        assert_abs_diff_eq!(erf(1.0) + erfc(1.0), 1.0, epsilon = f64::EPSILON);
    }
}
