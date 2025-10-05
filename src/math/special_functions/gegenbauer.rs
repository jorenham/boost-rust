//! boost/math/special_functions/gegenbauer.hpp

use crate::ffi;
use core::ffi::c_uint;

/// Gegenbauer Polynomial *C<sub>n</sub><sup>(&lambda;)</sup>(x)* on *[-1, 1]*
///
/// Corresponds to `boost::math::gegenbauer(n, lambda, x)`.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/gegenbauer.html>
pub fn gegenbauer(n: u32, lambda: f64, x: f64) -> f64 {
    unsafe { ffi::math_gegenbauer(n as c_uint, lambda, x) }
}

/// Derivative of [`gegenbauer`]
///
/// Corresponds to `boost::math::gegenbauer_derivative(n, lambda, x, k)`.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/gegenbauer.html>
#[doc(alias = "gegenbauer_prime")]
pub fn gegenbauer_derivative(n: u32, lambda: f64, x: f64, k: u32) -> f64 {
    unsafe { ffi::math_gegenbauer_derivative(n as c_uint, lambda, x, k as c_uint) }
}

#[cfg(test)]
mod tests {
    use crate::math::{gegenbauer, gegenbauer_derivative};

    const TOL: f64 = 1e-15;

    #[test]
    fn test_gegenbauer() {
        // n=0
        assert_abs_diff_eq!(gegenbauer(0, 0.5, -1.0), 1.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer(0, 0.5, 0.0), 1.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer(0, 0.5, 1.0), 1.0, epsilon = TOL);

        // n=1
        assert_abs_diff_eq!(gegenbauer(1, 0.5, -1.0), -1.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer(1, 0.5, 0.0), 0.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer(1, 0.5, 1.0), 1.0, epsilon = TOL);

        // n=2
        assert_abs_diff_eq!(gegenbauer(2, 0.5, -1.0), 1.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer(2, 0.5, 0.0), -0.5, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer(2, 0.5, 1.0), 1.0, epsilon = TOL);

        // n=3
        assert_abs_diff_eq!(gegenbauer(3, 0.5, -1.0), -1.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer(3, 0.5, 0.0), 0.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer(3, 0.5, 1.0), 1.0, epsilon = TOL);

        // n=4
        assert_abs_diff_eq!(gegenbauer(4, 0.5, -1.0), 1.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer(4, 0.5, 0.0), 0.375, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer(4, 0.5, 1.0), 1.0, epsilon = TOL);
    }

    #[test]
    fn test_gegenbauer_derivative() {
        // n=3, k=1
        assert_abs_diff_eq!(gegenbauer_derivative(3, 0.5, -1.0, 1), 6.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer_derivative(3, 0.5, 0.0, 1), -1.5, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer_derivative(3, 0.5, 1.0, 1), 6.0, epsilon = TOL);

        // n=3, k=2
        assert_abs_diff_eq!(gegenbauer_derivative(3, 0.5, -1.0, 2), -15.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer_derivative(3, 0.5, 0.0, 2), 0.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer_derivative(3, 0.5, 1.0, 2), 15.0, epsilon = TOL);

        // n=3, k=3
        assert_abs_diff_eq!(gegenbauer_derivative(3, 0.5, -1.0, 3), 15.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer_derivative(3, 0.5, 0.0, 3), 15.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer_derivative(3, 0.5, 1.0, 3), 15.0, epsilon = TOL);

        // n=4, k=1
        assert_abs_diff_eq!(gegenbauer_derivative(4, 0.5, -1.0, 1), -10.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer_derivative(4, 0.5, 0.0, 1), 0.0, epsilon = TOL);
        assert_abs_diff_eq!(gegenbauer_derivative(4, 0.5, 1.0, 1), 10.0, epsilon = TOL);
    }
}
