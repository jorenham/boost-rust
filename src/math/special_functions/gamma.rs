//! boost/math/special_functions/gamma.hpp
//!
//! Note that we deviate from the original "tgamma" names by dropping the "t" prefix.
//!
//! # TODO:
//! - `tgamma_delta_ratio`
//! - `tgamma_ratio`
//! - `gamma_p_derivative`
//! - `gamma_p_inv`
//! - `gamma_p_inva`
//! - `gamma_q_inv`
//! - `gamma_q_inva`

use crate::ffi;
use core::ffi::c_int;

/// Gamma function *Γ(x)*
///
/// Corresponds to `boost::math::tgamma(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/tgamma.html>
pub fn gamma(x: f64) -> f64 {
    unsafe { ffi::math_tgamma(x) }
}

/// Accurate evaluation of `tgamma(x + 1) - 1` for very small `x`
///
/// Internally the implementation does not make use of the addition and subtraction implied by the
/// definition, leading to accurate results even for very small `x`.
///
/// See [`gamma`] for the gamma function itself.
///
/// Corresponds to `boost::math::tgamma1pm1(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/tgamma.html>
pub fn gamma1pm1(x: f64) -> f64 {
    unsafe { ffi::math_tgamma1pm1(x) }
}

/// Natural logarithm of the absolute value of the gamma function
///
/// The integer part of the tuple indicates the sign of the gamma function.
///
/// See [`gamma`] for the gamma function itself.
///
/// Corresponds to `boost::math::lgamma(x, *sign)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/lgamma.html>
pub fn lgamma(x: f64) -> (f64, i32) {
    let mut sign: c_int = 0;
    let out = unsafe { ffi::math_lgamma(x, &mut sign) };
    assert_ne!(sign, 0);
    (out, sign)
}

/// Lower incomplete gamma function *γ(a,x)*
///
/// See also:
/// - [`gamma`]: Gamma function *Γ(x)*
/// - [`gamma_upper`]: Upper incomplete gamma function *Γ(a,x)*
/// - [`gamma_p`]: Normalized lower incomplete gamma function *P(a,x) = γ(a,x) / Γ(a)*
///
/// Corresponds to `boost::math::tgamma_lower(a, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/igamma.html>
pub fn gamma_lower(a: f64, x: f64) -> f64 {
    unsafe { ffi::math_tgamma_lower(a, x) }
}

/// Upper incomplete gamma function *Γ(a,x)*
///
/// See also:
/// - [`gamma`]: Gamma function *Γ(x)*
/// - [`gamma_lower`]: Lower incomplete gamma function *γ(a,x)*
/// - [`gamma_q`]: Normalized upper incomplete gamma function *Q(a,x) = Γ(a,x) / Γ(a)*
///
/// Corresponds to `boost::math::tgamma(a, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/igamma.html>
pub fn gamma_upper(a: f64, x: f64) -> f64 {
    unsafe { ffi::math_tgamma_(a, x) }
}

/// Normalized lower incomplete gamma function *P(a,x)*
///
/// *P(a,x) = γ(a,x) / Γ(a)*
///
/// See also:
/// - [`gamma`]: Gamma function *Γ(x)*
/// - [`gamma_lower`]: Lower incomplete gamma function *γ(a,x)*
/// - [`gamma_q`]: Normalized upper incomplete gamma function *Q(a,x)*
///
/// Corresponds to `boost::math::gamma_p(a, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/igamma.html>
pub fn gamma_p(a: f64, x: f64) -> f64 {
    unsafe { ffi::math_gamma_p(a, x) }
}

/// Normalized upper incomplete gamma function *Q(a,x)*
///
/// *Q(a,x) = Γ(a,x) / Γ(a)*
///
/// See also:
/// - [`gamma`]: Gamma function *Γ(x)*
/// - [`gamma_upper`]: Upper incomplete gamma function *Γ(a,x)*
/// - [`gamma_p`]: Normalized lower incomplete gamma function *P(a,x)*
///
/// Corresponds to `boost::math::gamma_q(a, x)` in C++.
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
    fn test_gamma() {
        assert!(gamma(0.0).is_infinite());
        assert_relative_eq!(gamma(-0.5), -2.0 * SQRT_PI, epsilon = RTOL);
        assert_relative_eq!(gamma(0.5), SQRT_PI, epsilon = RTOL);
        assert_relative_eq!(gamma(1.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(gamma(2.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(gamma(3.0), 2.0, epsilon = RTOL);
    }

    #[test]
    fn test_gamma1pm1() {
        assert_relative_eq!(gamma1pm1(-0.5), SQRT_PI - 1.0, epsilon = RTOL);
        assert_relative_eq!(gamma1pm1(0.0), 0.0, epsilon = RTOL);
        assert_relative_eq!(gamma1pm1(1.0), 0.0, epsilon = RTOL);
        assert_relative_eq!(gamma1pm1(2.0), 1.0, epsilon = RTOL);
        // results from Wolphram Alpha
        assert_abs_diff_eq!(
            gamma1pm1(-1e-14),
            5.772_156_649_015_427_5e-15,
            epsilon = 1e-30
        );
        assert_abs_diff_eq!(
            gamma1pm1(1e-14),
            -5.772_156_649_015_229_5e-15,
            epsilon = 1e-30
        );
    }

    #[test]
    fn test_lgamma() {
        let (val, sign) = lgamma(0.5);
        assert!(val.is_finite());
        assert!(val > 0.0);
        assert_eq!(sign, 1);

        let (val, sign) = lgamma(-0.5);
        assert!(val.is_finite());
        assert!(val > 0.0);
        assert_eq!(sign, -1);
    }

    #[test]
    fn test_gamma_lower() {
        assert!(gamma_lower(4.2, 0.5).is_finite());
    }

    #[test]
    fn test_gamma_upper() {
        assert!(gamma_upper(4.2, 0.5).is_finite());
    }

    #[test]
    fn test_gamma_p() {
        assert!(gamma_p(4.2, 0.5).is_finite());
    }

    #[test]
    fn test_gamma_q() {
        assert!(gamma_q(4.2, 0.5).is_finite());
    }
}
