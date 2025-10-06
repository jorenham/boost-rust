//! boost/math/special_functions/heuman_lambda.hpp

use crate::ffi;

/// Heuman's Lambda function *Λ<sub>0</sub>(φ,k)*
///
/// Corresponds to `boost::math::heuman_lambda` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/heuman_lambda.html>
pub fn heuman_lambda(k: f64, phi: f64) -> f64 {
    unsafe { ffi::math_heuman_lambda(k, phi) }
}

#[cfg(test)]
mod smoketests {
    use crate::math::heuman_lambda;

    #[test]
    fn test_heuman_lambda() {
        assert!(heuman_lambda(0.5, 1.0).is_finite());
    }
}
