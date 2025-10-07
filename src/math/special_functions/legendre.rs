//! boost/math/special_functions/legendre.hpp

use crate::ffi;
use alloc::{vec, vec::Vec};
use core::ffi::{c_int, c_uint};

/// Legendre Polynomial of the 1st kind *P<sub>n</sub>(x)* on *[-1, 1]*
///
/// Corresponds to `boost::math::legendre_p(n, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/legendre.html>
pub fn legendre_p(n: u32, x: f64) -> f64 {
    unsafe { ffi::math_legendre_p(n as c_int, x) }
}

/// Derivative of [`legendre_p`] with respect to `x`; *P'<sub>n</sub>(x)*
///
/// Corresponds to `boost::math::legendre_p_prime(n, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/legendre.html>
#[doc(alias = "legendre_p_derivative")]
pub fn legendre_p_prime(n: u32, x: f64) -> f64 {
    unsafe { ffi::math_legendre_p_prime(n as c_int, x) }
}

/// Associated Legendre Polynomial of the 1st kind *P<sub>n</sub><sup>m</sup>(x)* on *[-1, 1]*
///
/// Corresponds to `boost::math::legendre_p(n, m, x)` in C++
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/legendre.html>
pub fn legendre_p_assoc(n: u32, m: i32, x: f64) -> f64 {
    unsafe { ffi::math_legendre_p_assoc(n as c_int, m as c_int, x) }
}

/// Zeros (roots) of [`legendre_p`] on *[0, 1]*.
///
/// Note that only the non-negative zeros are returned, of which there are `n.div_ceil(2)`.
///
/// Corresponds to `boost::math::legendre_p_zeros<double>(n)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/legendre.html>
pub fn legendre_p_zeros(n: usize) -> Vec<f64> {
    // number of positive zeros
    let mut out = vec![f64::NAN; n.div_ceil(2)];
    unsafe { ffi::math_legendre_p_zeros(n as c_int, out.as_mut_ptr()) };
    out
}

/// Legendre Polynomial of the 2nd kind *Q<sub>n</sub>(x)* on *[-1, 1]*
///
/// Corresponds to `boost::math::legendre_q(n, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/legendre.html>
pub fn legendre_q(n: u32, x: f64) -> f64 {
    unsafe { ffi::math_legendre_q(n as c_uint, x) }
}

/// Recurrence relation for [`legendre_p`] and [`legendre_q`]
///
/// *(n+1)P<sub>n+1</sub>(x) = (2n+1)xP<sub>n</sub>(x) - nP<sub>n-1</sub>(x)*
///
/// # Examples
///
/// [`legendre_p`] recurrence:
///
/// ```
/// # use boost::math::{legendre_p, legendre_next};
/// let x = 0.42;
/// let p0 = legendre_p(0, x); // 1
/// let p1 = legendre_p(1, x); // x
/// let p2 = legendre_p(2, x); // (3x² - 1) / 2
/// let p3 = legendre_p(3, x); // (5x³ - 3x) / 2
/// assert_eq!(legendre_next(1, x, p1, p0), p2);
/// assert_eq!(legendre_next(2, x, p2, p1), p3);
/// ```
///
/// [`legendre_q`] recurrence:
///
/// ```
/// # use approx::assert_relative_eq;
/// # use boost::math::{legendre_q, legendre_next};
/// let x = 0.42;
/// let q0 = legendre_q(0, x);
/// let q1 = legendre_q(1, x);
/// let q2 = legendre_q(2, x);
/// let q3 = legendre_q(3, x);
/// assert_relative_eq!(legendre_next(1, x, q1, q0), q2);
/// assert_relative_eq!(legendre_next(2, x, q2, q1), q3);
/// ```
///
/// # See also
///
/// - [`legendre_p`]
/// - [`legendre_assoc_next`]
#[inline(always)]
#[allow(non_snake_case)]
#[doc(alias = "legendre_p_next")]
#[doc(alias = "legendre_q_next")]
pub fn legendre_next(n: u32, x: f64, Pn: f64, Pn_prev: f64) -> f64 {
    legendre_assoc_next(n, 0, x, Pn, Pn_prev)
}

/// Recurrence relation for [`legendre_p_assoc`]
///
/// *(n-m+1)P<sub>n+1</sub><sup>m</sup>(x)
/// = (2n+1)xP<sub>n</sub><sup>m</sup>(x) - (n+m)P<sub>n-1</sub><sup>m</sup>(x)*
///
/// # Examples
///
/// ```
/// # use approx::assert_relative_eq;
/// # use boost::math::{legendre_p_assoc, legendre_assoc_next};
/// let m = 1;
/// let x = 0.42;
/// let p0 = legendre_p_assoc(0, m, x);
/// let p1 = legendre_p_assoc(1, m, x);
/// let p2 = legendre_p_assoc(2, m, x);
/// let p3 = legendre_p_assoc(3, m, x);
/// assert_relative_eq!(legendre_assoc_next(1, m, x, p1, p0), p2);
/// assert_relative_eq!(legendre_assoc_next(2, m, x, p2, p1), p3);
/// ```
///
/// # See also
///
/// - [`legendre_p_assoc`]
/// - [`legendre_next`]
#[inline(always)]
#[allow(non_snake_case)]
#[doc(alias = "legendre_p_assoc_next")]
pub fn legendre_assoc_next(n: u32, m: i32, x: f64, Pn: f64, Pn_prev: f64) -> f64 {
    let n = n as i32;
    ((2 * n + 1) as f64 * x * Pn - (n + m) as f64 * Pn_prev) / ((n - m + 1) as f64)
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
        assert_eq!(legendre_p(1, 0.5), 0.5);
        assert_eq!(legendre_p(2, 0.5), -0.125);
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
        assert_eq!(legendre_p_prime(1, 0.5), 1.0);
        assert_eq!(legendre_p_prime(2, 0.5), 1.5);
    }

    #[test]
    fn test_legendre_p_zeros() {
        assert_eq!(legendre_p_zeros(0), vec![]);
        assert_eq!(legendre_p_zeros(1), vec![0.0]);
        assert_eq!(legendre_p_zeros(2), vec![FRAC_1_SQRT_3]);
        assert_eq!(legendre_p_zeros(3), vec![0.0, SQRT_FRAC_3_5]);
    }

    #[test]
    fn test_legendre_q() {
        assert_abs_diff_eq!(legendre_q(0, 0.5), 0.5 * LN_3, epsilon = ATOL);
        assert_abs_diff_eq!(legendre_q(1, 0.5), 0.25 * LN_3 - 1.0, epsilon = ATOL);
        assert_abs_diff_eq!(legendre_q(2, 0.5), -0.0625 * LN_3 - 0.75, epsilon = ATOL);
    }
}
