use super::super::ffi;

/// Gamma function Γ(x)
///
/// https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/tgamma.html
///
/// Defined in `boost/math/special_functions/gamma.hpp`
pub fn tgamma(x: f64) -> f64 {
    unsafe { ffi::boost_math_tgamma(x) }
}

/// Log-Gamma function ln |Γ(x)|
///
/// https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/lgamma.html
///
/// Defined in `boost/math/special_functions/gamma.hpp`
pub fn lgamma(x: f64) -> f64 {
    unsafe { ffi::boost_math_lgamma(x) }
}

/// Incomplete gamma function P(a,x) = γ(a,x) / Γ(a)
///
/// https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/igamma.html
///
/// Defined in `boost/math/special_functions/gamma.hpp`
pub fn gamma_p(a: f64, x: f64) -> f64 {
    unsafe { ffi::boost_math_gamma_p(a, x) }
}

/// Incomplete gamma function Q(a,x) = Γ(a,x) / Γ(a)
///
/// https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/igamma.html
///
/// Defined in `boost/math/special_functions/gamma.hpp`
pub fn gamma_q(a: f64, x: f64) -> f64 {
    unsafe { ffi::boost_math_gamma_q(a, x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tgamma() {
        assert_abs_diff_eq!(tgamma(1.0), 1.0, epsilon = f64::EPSILON);
        assert_abs_diff_eq!(
            tgamma(0.5),
            core::f64::consts::PI.sqrt(),
            epsilon = f64::EPSILON
        );
        assert_abs_diff_eq!(tgamma(5.0), 24.0, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_lgamma() {
        assert_abs_diff_eq!(lgamma(1.0), 0.0, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_gamma_p() {
        assert_abs_diff_eq!(gamma_p(2.0, 0.0), 0.0, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_gamma_q() {
        assert_abs_diff_eq!(gamma_q(2.0, 0.0), 1.0, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_gamma_p_q_relation() {
        assert_abs_diff_eq!(
            gamma_p(2.0, 1.0) + gamma_q(2.0, 1.0),
            1.0,
            epsilon = f64::EPSILON
        );
    }
}
