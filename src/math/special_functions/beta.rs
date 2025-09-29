//! boost/math/special_functions/beta.hpp

use crate::ffi;

/// Beta function *B(a,b)*
///
/// *B(a,b) = B<sub>1</sub>(a,b) = Γ(a) Γ(b) / Γ(a + b)*
///
/// See also:
/// - [`beta_`]: Incomplete Beta function *B<sub>x</sub>(a,b)*.
/// - [`ibeta`]: Regularized incomplete Beta function *I<sub>x</sub>(a,b)*.
///
/// Corresponds to `boost::math::beta(a, b)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/beta_function.html>
pub fn beta(a: f64, b: f64) -> f64 {
    unsafe { ffi::math_beta(a, b) }
}

/// Incomplete Beta function *B<sub>x</sub>(a,b)*
///
/// Requires *a > 0* and *b > 0*.
///
/// See also:
/// - [`beta`]: Beta function *B(a,b) = B<sub>1</sub>(a,b)*.
/// - [`betac`]: Complement of the Beta function *B<sub>x</sub>(a,b)*.
/// - [`ibeta`]: Regularized incomplete Beta function *I<sub>x</sub>(a,b)*.
///
/// Corresponds to `boost::math::beta(a, b, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/ibeta_function.html>
pub fn beta_(a: f64, b: f64, x: f64) -> f64 {
    unsafe { ffi::math_beta_(a, b, x) }
}

/// Complement of [`beta_`]
///
/// That is, *1 - B<sub>x</sub>(a,b) = B<sub>1-x</sub>(b,a)*.
///
/// Corresponds to `boost::math::betac(a, b, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/ibeta_function.html>
pub fn betac(a: f64, b: f64, x: f64) -> f64 {
    unsafe { ffi::math_betac(a, b, x) }
}

/// Regularized incomplete Beta function *I<sub>x</sub>(a,b)*
///
/// Requires *a ≥ 0* and *b ≥ 0* s.t. *a + b > 0*.
///
/// *I<sub>x</sub>(a,b) = B<sub>x</sub>(a,b) / B(a,b)*.
///
/// See also:
/// - [`beta`]: Beta function *B(a,b)*.
/// - [`beta_`]: Incomplete Beta function *B<sub>x</sub>(a,b)*.
/// - [`ibetac`]: Complement of the regularized incomplete Beta function
///
/// Corresponds to `boost::math::ibeta(a, b, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/ibeta_function.html>
pub fn ibeta(a: f64, b: f64, x: f64) -> f64 {
    unsafe { ffi::math_ibeta(a, b, x) }
}

/// Derivative of [`ibeta`] w.r.t. `x`, *I'<sub>x</sub>(a,b)*
///
/// *I'<sub>x</sub>(a,b) = x<sup>a-1</sup> (1-x)<sup>b-1</sup> / B(a,b)*
///
/// Note that this is the probability density function of the Beta distribution.
///
/// Corresponds to `boost::math::ibeta_derivative(a, b, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/beta_derivative.html>
pub fn ibeta_derivative(a: f64, b: f64, x: f64) -> f64 {
    unsafe { ffi::math_ibeta_derivative(a, b, x) }
}

/// Inverse of [`ibeta`] w.r.t. `x`
///
/// Corresponds to `boost::math::ibeta_inv(a, b, p)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/ibeta_inv_function.html>
pub fn ibeta_inv(a: f64, b: f64, p: f64) -> f64 {
    unsafe { ffi::math_ibeta_inv(a, b, p) }
}

/// Inverse of [`ibeta`] w.r.t. `a`
///
/// Corresponds to `boost::math::ibeta_inva(b, x, p)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/ibeta_inv_function.html>
pub fn ibeta_inva(b: f64, x: f64, p: f64) -> f64 {
    unsafe { ffi::math_ibeta_inva(b, x, p) }
}

/// Inverse of [`ibeta`] w.r.t. `b`
///
/// Corresponds to `boost::math::ibeta_invb(a, x, p)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/ibeta_inv_function.html>
pub fn ibeta_invb(a: f64, x: f64, p: f64) -> f64 {
    unsafe { ffi::math_ibeta_invb(a, x, p) }
}

/// Complement of [`ibeta`]
///
/// That is, *1 - I<sub>x</sub>(a,b) = I<sub>1-x</sub>(b,a)*.
///
/// Corresponds to `boost::math::ibetac(a, b, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/ibeta_function.html>
pub fn ibetac(a: f64, b: f64, x: f64) -> f64 {
    unsafe { ffi::math_ibetac(a, b, x) }
}

/// Inverse of [`ibetac`] w.r.t. `x`
///
/// Corresponds to `boost::math::ibetac_inv(a, b, q)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/ibeta_inv_function.html>
pub fn ibetac_inv(a: f64, b: f64, q: f64) -> f64 {
    unsafe { ffi::math_ibetac_inv(a, b, q) }
}

/// Inverse of [`ibetac`] w.r.t. `a`
///
/// Corresponds to `boost::math::ibetac_inva(b, x, q)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/ibeta_inv_function.html>
pub fn ibetac_inva(b: f64, x: f64, q: f64) -> f64 {
    unsafe { ffi::math_ibetac_inva(b, x, q) }
}

/// Inverse of [`ibetac`] w.r.t. `b`
///
/// Corresponds to `boost::math::ibetac_invb(a, x, q)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/ibeta_inv_function.html>
pub fn ibetac_invb(a: f64, x: f64, q: f64) -> f64 {
    unsafe { ffi::math_ibetac_invb(a, x, q) }
}

#[cfg(test)]
mod smoketests {
    use super::*;

    #[test]
    fn test_beta() {
        let result = beta(0.5, 0.5);
        assert_relative_eq!(result, core::f64::consts::PI, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_beta_() {
        let result = beta_(0.5, 0.5, 0.5);
        assert!(result.is_finite());
    }

    #[test]
    fn test_betac() {
        let result = betac(0.5, 0.5, 0.5);
        assert!(result.is_finite());
    }

    #[test]
    fn test_ibeta() {
        let result = ibeta(0.5, 0.5, 0.5);
        assert!(result.is_finite());
    }

    #[test]
    fn test_ibeta_derivative() {
        let result = ibeta_derivative(0.5, 0.5, 0.5);
        assert!(result.is_finite());
    }

    #[test]
    fn test_ibeta_inv() {
        let result = ibeta_inv(0.5, 0.5, 0.5);
        assert!(result.is_finite());
    }

    #[test]
    fn test_ibeta_inva() {
        let result = ibeta_inva(0.5, 0.5, 0.5);
        assert!(result.is_finite());
    }

    #[test]
    fn test_ibeta_invb() {
        let result = ibeta_invb(0.5, 0.5, 0.5);
        assert!(result.is_finite());
    }

    #[test]
    fn test_ibetac() {
        let result = ibetac(0.5, 0.5, 0.5);
        assert!(result.is_finite());
    }

    #[test]
    fn test_ibetac_inv() {
        let result = ibetac_inv(0.5, 0.5, 0.5);
        assert!(result.is_finite());
    }

    #[test]
    fn test_ibetac_inva() {
        let result = ibetac_inva(0.5, 0.5, 0.5);
        assert!(result.is_finite());
    }

    #[test]
    fn test_ibetac_invb() {
        let result = ibetac_invb(0.5, 0.5, 0.5);
        assert!(result.is_finite());
    }
}
