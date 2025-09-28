//! boost/math/special_functions/jacobi.hpp

use crate::ffi;
use core::ffi::c_uint;

/// Jacobi Polynomial *P<sub>n</sub><sup>(α, β)</sup>(x)*
///
/// Corresponds to `boost::math::jacobi(n, alpha, beta, x)`.
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/jacobi.html>
pub fn jacobi(n: u32, alpha: f64, beta: f64, x: f64) -> f64 {
    unsafe { ffi::math_jacobi(n as c_uint, alpha, beta, x) }
}

/// *k*-th derivative of [`jacobi`] with respect to `x`
///
/// Corresponds to `boost::math::jacobi_derivative(n, alpha, beta, x, k)`.
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/jacobi.html>
pub fn jacobi_derivative(n: u32, alpha: f64, beta: f64, x: f64, k: u32) -> f64 {
    unsafe { ffi::math_jacobi_derivative(n as c_uint, alpha, beta, x, k as c_uint) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jacobi() {
        assert_eq!(jacobi(0, 2.0, 3.0, 0.5), 1.0);
        assert_eq!(jacobi(1, 2.0, 3.0, 0.5), 1.25);
        assert_eq!(jacobi(2, 2.0, 3.0, 0.5), 0.25);
        assert_eq!(jacobi(3, 2.0, 3.0, 0.5), -1.015625);
        assert_eq!(jacobi(4, 2.0, 3.0, 0.5), -1.26953125);
    }

    #[test]
    fn test_jacobi_derivative() {
        assert_eq!(jacobi_derivative(0, 2.0, 3.0, 0.5, 1), 0.0);
        assert_eq!(jacobi_derivative(1, 2.0, 3.0, 0.5, 1), 3.5);
        assert_eq!(jacobi_derivative(2, 2.0, 3.0, 0.5, 1), 7.0);
        assert_eq!(jacobi_derivative(3, 2.0, 3.0, 0.5, 1), 4.21875);
        assert_eq!(jacobi_derivative(4, 2.0, 3.0, 0.5, 1), -4.84375);
    }
}
