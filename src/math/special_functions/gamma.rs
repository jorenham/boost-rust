//! boost/math/special_functions/gamma.hpp
//!
//! # TODO:
//! - `tgamma_lower`
//! - `tgamma_delta_ratio`
//! - `tgamma_ratio`
//! - `gamma_p_derivative`
//! - `gamma_p_inv`
//! - `gamma_p_inva`
//! - `gamma_q_inv`
//! - `gamma_q_inva`

use crate::ffi;

/// Gamma function *Γ(x)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/tgamma.html>
pub fn tgamma(x: f64) -> f64 {
    unsafe { ffi::math_tgamma(x) }
}

/// Accurate evaluation of `tgamma(x + 1) - 1` for very small `x`
///
/// Internally the implementation does not make use of the addition and subtraction implied by the
/// definition, leading to accurate results even for very small `x`.
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/tgamma.html>
pub fn tgamma1pm1(x: f64) -> f64 {
    unsafe { ffi::math_tgamma1pm1(x) }
}

/// Log-Gamma function *ln |Γ(x)|*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/lgamma.html>
pub fn lgamma(x: f64) -> f64 {
    unsafe { ffi::math_lgamma(x) }
}

/// Incomplete gamma function *P(a,x) = γ(a,x) / Γ(a)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/igamma.html>
pub fn gamma_p(a: f64, x: f64) -> f64 {
    unsafe { ffi::math_gamma_p(a, x) }
}

/// Incomplete gamma function *Q(a,x) = Γ(a,x) / Γ(a)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/igamma.html>
pub fn gamma_q(a: f64, x: f64) -> f64 {
    unsafe { ffi::math_gamma_q(a, x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RTOL: f64 = f64::EPSILON;
    const SQRT_PI: f64 = 1.772_453_850_905_516; // √π

    #[test]
    fn test_tgamma() {
        assert!(tgamma(0.0).is_infinite());
        assert_relative_eq!(tgamma(-0.5), -2.0 * SQRT_PI, epsilon = RTOL);
        assert_relative_eq!(tgamma(0.5), SQRT_PI, epsilon = RTOL);
        assert_relative_eq!(tgamma(1.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(tgamma(2.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(tgamma(3.0), 2.0, epsilon = RTOL);
    }

    #[test]
    fn test_tgamma1pm1() {
        assert_relative_eq!(tgamma1pm1(-0.5), SQRT_PI - 1.0, epsilon = RTOL);
        assert_relative_eq!(tgamma1pm1(0.0), 0.0, epsilon = RTOL);
        assert_relative_eq!(tgamma1pm1(1.0), 0.0, epsilon = RTOL);
        assert_relative_eq!(tgamma1pm1(2.0), 1.0, epsilon = RTOL);
        // results from Wolphram Alpha
        assert_abs_diff_eq!(
            tgamma1pm1(-1e-14),
            5.772_156_649_015_427_5e-15,
            epsilon = 1e-30
        );
        assert_abs_diff_eq!(
            tgamma1pm1(1e-14),
            -5.772_156_649_015_229_5e-15,
            epsilon = 1e-30
        );
    }

    #[test]
    fn test_lgamma() {
        assert_relative_eq!(lgamma(1.0), 0.0, epsilon = RTOL);
    }

    #[test]
    fn test_gamma_p() {
        assert_relative_eq!(gamma_p(2.0, 0.0), 0.0, epsilon = RTOL);
    }
    #[test]
    fn test_gamma_q() {
        assert_relative_eq!(gamma_q(2.0, 0.0), 1.0, epsilon = RTOL);
    }

    #[test]
    fn test_gamma_p_q() {
        assert_relative_eq!(gamma_p(2.0, 1.0) + gamma_q(2.0, 1.0), 1.0, epsilon = RTOL);
    }
}
