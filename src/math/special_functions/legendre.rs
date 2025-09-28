//! boost/math/special_functions/legendre.hpp
//!
//! # TODO:
//! - `double legendre_next(unsigned l, double x, double Pl, double Plm1)`
//! - `double legendre_next(unsigned l, unsigned m, double x, double Pl, double Plm1)`

use crate::ffi;
use alloc::{vec, vec::Vec};
use core::ffi::{c_int, c_uint};

/// Legendre Polynomial of the 1st kind *P<sub>l</sub>(x)* on *[-1, 1]*
///
/// Corresponds to `boost::math::legendre_p(l, x)`.
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/legendre.html>
pub fn legendre_p(l: i32, x: f64) -> f64 {
    unsafe { ffi::math_legendre_p(l as c_int, x) }
}

/// Associated Legendre Polynomial of the 1st kind *P<sub>l</sub><sup>m</sup>(x)* on *[-1, 1]*
///
/// Corresponds to `boost::math::legendre_p(l, m, x)`.
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/legendre.html>
pub fn legendre_p_assoc(l: i32, m: i32, x: f64) -> f64 {
    unsafe { ffi::math_legendre_p_assoc(l as c_int, m as c_int, x) }
}

/// Derivative of [`legendre_p`] with respect to `x`; *P'<sub>l</sub>(x)*
///
/// Corresponds to `boost::math::legendre_p_prime(l, x)`.
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/legendre.html>
pub fn legendre_p_prime(l: i32, x: f64) -> f64 {
    unsafe { ffi::math_legendre_p_prime(l as c_int, x) }
}

/// Zeros of the Legendre Polynomial of the 1st kind *P<sub>l</sub>(x)*
///
/// Note that only the non-negative zeros are returned, of which there are `ceil(l / 2)`.
///
/// Corresponds to `boost::math::legendre_p_zeros<double>(l)`.
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/legendre.html>
pub fn legendre_p_zeros(l: i32) -> Vec<f64> {
    // total number of zeros
    let k = (if l < 0 { -l - 1 } else { l }) as usize;
    // number of positive zeros
    let mut out = vec![f64::NAN; k.div_ceil(2)];
    unsafe { ffi::math_legendre_p_zeros(l as c_int, out.as_mut_ptr()) };
    out
}

/// Legendre Polynomial of the 2nd kind *Q<sub>l</sub>(x)* on *[-1, 1]*
///
/// Corresponds to `boost::math::legendre_q(l, x)`.
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/legendre.html>
pub fn legendre_q(l: u32, x: f64) -> f64 {
    unsafe { ffi::math_legendre_q(l as c_uint, x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ATOL: f64 = f64::EPSILON;

    const LN_3: f64 = 1.098_612_288_668_109_8; // ln(3)
    const SQRT_3: f64 = 1.732_050_807_568_877_2; // sqrt(3)
    const FRAC_1_SQRT_3: f64 = 0.577_350_269_189_625_7; // 1/sqrt(3)
    const SQRT_FRAC_3_5: f64 = 0.774_596_669_241_483_4; // sqrt(3/5)

    #[test]
    fn test_legendre_p() {
        assert_eq!(legendre_p(0, 0.5), 1.0);
        assert_eq!(legendre_p(-1, 0.5), 1.0);
        assert_eq!(legendre_p(1, 0.5), 0.5);
        assert_eq!(legendre_p(-2, 0.5), 0.5);
        assert_eq!(legendre_p(2, 0.5), -0.125);
        assert_eq!(legendre_p(-3, 0.5), -0.125);
    }

    #[test]
    fn test_legendre_p_assoc() {
        assert_eq!(legendre_p_assoc(0, 0, 0.5), 1.0);
        assert_eq!(legendre_p_assoc(1, 0, 0.5), 0.5);
        assert_eq!(legendre_p_assoc(2, 0, 0.5), -0.125);
        assert_eq!(legendre_p_assoc(0, 1, 0.5), 0.0);
        assert_abs_diff_eq!(legendre_p_assoc(1, 1, 0.5), -0.5 * SQRT_3, epsilon = ATOL);
        assert_abs_diff_eq!(legendre_p_assoc(2, 1, 0.5), -0.75 * SQRT_3, epsilon = ATOL);
    }

    #[test]
    fn test_legendre_p_prime() {
        assert_eq!(legendre_p_prime(0, 0.5), 0.0);
        assert_eq!(legendre_p_prime(-1, 0.5), 0.0);
        assert_eq!(legendre_p_prime(1, 0.5), 1.0);
        assert_eq!(legendre_p_prime(-2, 0.5), 1.0);
        assert_eq!(legendre_p_prime(2, 0.5), 1.5);
        assert_eq!(legendre_p_prime(-3, 0.5), 1.5);
    }

    #[test]
    fn test_legendre_p_zeros() {
        assert_eq!(legendre_p_zeros(0), vec![]);
        assert_eq!(legendre_p_zeros(-1), vec![]);
        assert_eq!(legendre_p_zeros(1), vec![0.0]);
        assert_eq!(legendre_p_zeros(-2), vec![0.0]);
        assert_eq!(legendre_p_zeros(2), vec![FRAC_1_SQRT_3]);
        assert_eq!(legendre_p_zeros(-3), vec![FRAC_1_SQRT_3]);
        assert_eq!(legendre_p_zeros(3), vec![0.0, SQRT_FRAC_3_5]);
        assert_eq!(legendre_p_zeros(-4), vec![0.0, SQRT_FRAC_3_5]);
    }

    #[test]
    fn test_legendre_q() {
        assert_abs_diff_eq!(legendre_q(0, 0.5), 0.5 * LN_3, epsilon = ATOL);
        assert_abs_diff_eq!(legendre_q(1, 0.5), 0.25 * LN_3 - 1.0, epsilon = ATOL);
        assert_abs_diff_eq!(legendre_q(2, 0.5), -0.0625 * LN_3 - 0.75, epsilon = ATOL);
    }
}
