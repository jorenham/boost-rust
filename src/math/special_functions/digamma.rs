use super::super::ffi;

/// Digamma function ψ(x) = Γ'(x) / Γ(x)
///
/// https://www.boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/digamma.html
///
/// Defined in `boost/math/special_functions/digamma.hpp`
pub fn digamma(x: f64) -> f64 {
    unsafe { ffi::boost_math_digamma(x) }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_digamma_roots() {
        // assert!((crate::math::digamma(1.0) - 0.0).abs() < f64::EPSILON);
        assert_abs_diff_eq!(
            crate::math::digamma(1.461_632_144_968_362_2),
            0.0,
            epsilon = f64::EPSILON
        );
        assert_abs_diff_eq!(
            crate::math::digamma(-0.504_083_008_264_455_4),
            0.0,
            epsilon = f64::EPSILON
        );
        assert_abs_diff_eq!(
            crate::math::digamma(-1.573_498_473_162_390_4),
            0.0,
            epsilon = f64::EPSILON
        );
    }
}
