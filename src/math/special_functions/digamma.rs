//! boost/math/special_functions/digamma.hpp

use crate::ffi;

/// Digamma function *𝟊(x) = Γ'(x)/Γ(x)*
///
/// Corresponds to `boost::math::digamma(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_gamma/digamma.html>
pub fn digamma(x: f64) -> f64 {
    unsafe { ffi::math_digamma(x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RTOL: f64 = 1e-15;

    #[test]
    fn test_digamma_roots() {
        assert_relative_eq!(digamma(1.461_632_144_968_362_3), 0.0, epsilon = RTOL);
        assert_relative_eq!(digamma(-0.504_083_008_264_455_4), 0.0, epsilon = RTOL);
        assert_relative_eq!(digamma(-1.573_498_473_162_390_5), 0.0, epsilon = RTOL);
    }

    #[test]
    fn test_digamma_singularities() {
        assert!(digamma(f64::NAN).is_nan());
        assert_eq!(digamma(f64::INFINITY), f64::INFINITY);
        assert!(digamma(0.0).is_nan());
        assert!(digamma(-1.0).is_nan());
        assert!(digamma(-2.0).is_nan());
        assert!(digamma(-f64::INFINITY).is_nan());
    }
}
