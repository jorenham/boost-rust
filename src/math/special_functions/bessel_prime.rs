//! boost/math/special_functions/bessel_prime.hpp

use crate::ffi;

/// Derivative of [`cyl_bessel_j(x)`](crate::math::cyl_bessel_j)
///
/// *2 J'<sub>ν</sub>(x) = J<sub>ν-1</sub>(x) - J<sub>ν+1</sub>(x)*
///
/// Corresponds to `boost::math::cyl_bessel_j_prime` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/bessel_derivatives.html>
pub fn cyl_bessel_j_prime(nu: f64, x: f64) -> f64 {
    unsafe { ffi::math_cyl_bessel_j_prime(nu, x) }
}

/// Derivative of [`cyl_neumann(x)`](crate::math::cyl_neumann)
///
/// *2 Y'<sub>ν</sub>(x) = Y<sub>ν-1</sub>(x) - Y<sub>ν+1</sub>(x)*
///
/// Corresponds to `boost::math::cyl_neumann_prime` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/bessel_derivatives.html>
#[doc(alias = "cyl_bessel_y_prime")]
pub fn cyl_neumann_prime(nu: f64, x: f64) -> f64 {
    unsafe { ffi::math_cyl_neumann_prime(nu, x) }
}

/// Derivative of [`cyl_bessel_i(x)`](crate::math::cyl_bessel_i)
///
/// *2 I'<sub>ν</sub>(x) = I<sub>ν-1</sub>(x) + I<sub>ν+1</sub>(x)*
///
/// Corresponds to `boost::math::cyl_bessel_i_prime` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/bessel_derivatives.html>
pub fn cyl_bessel_i_prime(nu: f64, x: f64) -> f64 {
    unsafe { ffi::math_cyl_bessel_i_prime(nu, x) }
}

/// Derivative of [`cyl_bessel_k(x)`](crate::math::cyl_bessel_k)
///
/// *-2 K'<sub>ν</sub>(x) = K<sub>ν-1</sub>(x) + K<sub>ν+1</sub>(x)*
///
/// Corresponds to `boost::math::cyl_bessel_k_prime` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/bessel_derivatives.html>
pub fn cyl_bessel_k_prime(nu: f64, x: f64) -> f64 {
    unsafe { ffi::math_cyl_bessel_k_prime(nu, x) }
}

/// Derivative of [`sph_bessel(n, x)`](crate::math::sph_bessel)
///
/// *j'<sub>n</sub>(x) = (n/x) j<sub>n</sub>(x) - j<sub>n+1</sub>(x)*
///
/// Corresponds to `boost::math::sph_bessel_prime` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/bessel_derivatives.html>
#[doc(alias = "sph_bessel_j_prime")]
pub fn sph_bessel_prime(n: u32, x: f64) -> f64 {
    unsafe { ffi::math_sph_bessel_prime(n, x) }
}

/// Derivative of [`sph_neumann(n, x)`](crate::math::sph_neumann)
///
/// *y'<sub>n</sub>(x) = (n/x) y<sub>n</sub>(x) - y<sub>n+1</sub>(x)*
///
/// Corresponds to `boost::math::sph_neumann_prime` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/bessel_derivatives.html>
#[doc(alias = "sph_bessel_y_prime")]
pub fn sph_neumann_prime(n: u32, x: f64) -> f64 {
    unsafe { ffi::math_sph_neumann_prime(n, x) }
}

#[cfg(test)]
mod tests {
    use crate::math::{
        cyl_bessel_i_prime, cyl_bessel_j_prime, cyl_bessel_k_prime, cyl_neumann_prime,
        sph_bessel_prime, sph_neumann_prime,
    };

    const EPS: f64 = 1e-15;

    // values from Wolfram Alpha

    #[test]
    fn test_cyl_bessel_j_prime() {
        assert_abs_diff_eq!(cyl_bessel_j_prime(0.0, 1.0), -0.440_050_585_744_933_5);
    }

    #[test]
    fn test_cyl_neumann_prime() {
        assert_abs_diff_eq!(cyl_neumann_prime(0.0, 1.0), 0.781_212_821_300_288_7);
    }

    #[test]
    fn test_cyl_bessel_i_prime() {
        assert_abs_diff_eq!(cyl_bessel_i_prime(0.0, 1.0), 0.565_159_103_992_485);
    }

    #[test]
    fn test_cyl_bessel_k_prime() {
        assert_abs_diff_eq!(cyl_bessel_k_prime(0.0, 1.0), -0.601_907_230_197_234_6);
    }

    #[test]
    fn test_sph_bessel_prime() {
        assert_abs_diff_eq!(
            sph_bessel_prime(0, 1.0),
            -0.301_168_678_939_756_8,
            epsilon = EPS,
        );
    }

    #[test]
    fn test_sph_neumann_prime() {
        assert_abs_diff_eq!(
            sph_neumann_prime(0, 1.0),
            1.381_773_290_676_036_3,
            epsilon = EPS,
        );
    }
}
