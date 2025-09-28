//! boost/math/special_functions/digamma.hpp

use crate::ffi;

/// Digamma function *ψ(x) = Γ'(x) / Γ(x)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/digamma.html>
pub fn digamma(x: f64) -> f64 {
    unsafe { ffi::math_digamma(x) }
}

#[cfg(test)]
mod tests {
    const ATOL: f64 = 5e-16;

    #[test]
    fn test_digamma_roots() {
        assert_abs_diff_eq!(
            crate::math::digamma(1.461_632_144_968_362_3),
            0.0,
            epsilon = ATOL,
        );
        assert_abs_diff_eq!(
            crate::math::digamma(-0.504_083_008_264_455_4),
            0.0,
            epsilon = ATOL,
        );
        assert_abs_diff_eq!(
            crate::math::digamma(-1.573_498_473_162_390_5),
            0.0,
            epsilon = ATOL,
        );
    }
}
