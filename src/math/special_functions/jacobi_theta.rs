//! boost/math/special_functions/jacobi_theta.hpp

use crate::ffi;

/// Jacobi theta function *θ<sub>1</sub>(x, q)*
///
/// Corresponds to `boost::math::jacobi_theta1(x, q)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta1.html>
pub fn jacobi_theta_1(x: f64, q: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta1(x, q) }
}

/// Jacobi theta function *θ<sub>1</sub>(x, τ)*
///
/// Corresponds to `boost::math::jacobi_theta1tau(x, τ)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta1.html>
pub fn jacobi_theta_1_tau(x: f64, tau: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta1tau(x, tau) }
}

/// Jacobi theta function *θ<sub>2</sub>(x, q)*
///
/// Corresponds to `boost::math::jacobi_theta2(x, q)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta2.html>
pub fn jacobi_theta_2(x: f64, q: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta2(x, q) }
}

/// Jacobi theta function *θ<sub>2</sub>(x, τ)*
///
/// Corresponds to `boost::math::jacobi_theta2tau(x, τ)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta2.html>
pub fn jacobi_theta_2_tau(x: f64, tau: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta2tau(x, tau) }
}

/// Jacobi theta function *θ<sub>3</sub>(x, q)*
///
/// Corresponds to `boost::math::jacobi_theta3(x, q)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta3.html>
pub fn jacobi_theta_3(x: f64, q: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta3(x, q) }
}

/// Jacobi theta function *θ<sub>3</sub>(x, τ)*
///
/// Corresponds to `boost::math::jacobi_theta3tau(x, τ)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta3.html>
pub fn jacobi_theta_3_tau(x: f64, tau: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta3tau(x, tau) }
}

/// Jacobi theta function *θ<sub>3</sub>(x, q) - 1*
///
/// Corresponds to `boost::math::jacobi_theta3m1(x, q)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta3.html>
pub fn jacobi_theta_3m1(x: f64, q: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta3m1(x, q) }
}

/// Jacobi theta function *θ<sub>3</sub>(x, τ) - 1*
///
/// Corresponds to `boost::math::jacobi_theta3m1tau(x, τ)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta3.html>
pub fn jacobi_theta_3m1_tau(x: f64, tau: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta3m1tau(x, tau) }
}

/// Jacobi theta function *θ<sub>4</sub>(x, q)*
///
/// Corresponds to `boost::math::jacobi_theta4(x, q)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta4.html>
pub fn jacobi_theta_4(x: f64, q: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta4(x, q) }
}

/// Jacobi theta function *θ<sub>4</sub>(x, τ)*
///
/// Corresponds to `boost::math::jacobi_theta4tau(x, τ)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta4.html>
pub fn jacobi_theta_4_tau(x: f64, tau: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta4tau(x, tau) }
}

/// Jacobi theta function *θ<sub>4</sub>(x, q) = 1*
///
/// Corresponds to `boost::math::jacobi_theta4m1(x, q)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta4.html>
pub fn jacobi_theta_4m1(x: f64, q: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta4m1(x, q) }
}

/// Jacobi theta function *θ<sub>4</sub>(x, τ) - 1*
///
/// Corresponds to `boost::math::jacobi_theta4tau(x, τ)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi_theta/jacobi_theta4.html>
pub fn jacobi_theta_4m1_tau(x: f64, tau: f64) -> f64 {
    unsafe { ffi::math_jacobi_theta4m1tau(x, tau) }
}

#[cfg(test)]
mod smoketests {
    use super::*;

    #[test]
    fn test_jacobi_theta_1() {
        assert!(jacobi_theta_1(0.5, 0.5).is_finite());
    }

    #[test]
    fn test_jacobi_theta_1_tau() {
        assert!(jacobi_theta_1_tau(0.5, 0.5).is_finite());
    }

    #[test]
    fn test_jacobi_theta_2() {
        assert!(jacobi_theta_2(0.5, 0.5).is_finite());
    }

    #[test]
    fn test_jacobi_theta_2_tau() {
        assert!(jacobi_theta_2_tau(0.5, 0.5).is_finite());
    }

    #[test]
    fn test_jacobi_theta_3() {
        assert!(jacobi_theta_3(0.5, 0.5).is_finite());
    }

    #[test]
    fn test_jacobi_theta_3_tau() {
        assert!(jacobi_theta_3_tau(0.5, 0.5).is_finite());
    }

    #[test]
    fn test_jacobi_theta_3m1() {
        assert!(jacobi_theta_3m1(0.5, 0.5).is_finite());
    }

    #[test]
    fn test_jacobi_theta_3m1_tau() {
        assert!(jacobi_theta_3m1_tau(0.5, 0.5).is_finite());
    }

    #[test]
    fn test_jacobi_theta_4() {
        assert!(jacobi_theta_4(0.5, 0.5).is_finite());
    }

    #[test]
    fn test_jacobi_theta_4_tau() {
        assert!(jacobi_theta_4_tau(0.5, 0.5).is_finite());
    }

    #[test]
    fn test_jacobi_theta_4m1() {
        assert!(jacobi_theta_4m1(0.5, 0.5).is_finite());
    }

    #[test]
    fn test_jacobi_theta_4m1_tau() {
        assert!(jacobi_theta_4m1_tau(0.5, 0.5).is_finite());
    }
}
