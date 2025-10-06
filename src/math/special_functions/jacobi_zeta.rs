//! boost/math/special_functions/jacobi_zeta.hpp

use crate::ffi;

/// Jacobi's Zeta function *Z(φ,m)*.
///
/// Corresponds to `boost::math::jacobi_zeta` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ellint/jacobi_zeta.html>
pub fn jacobi_zeta(k: f64, phi: f64) -> f64 {
    unsafe { ffi::math_jacobi_zeta(k, phi) }
}

#[cfg(test)]
mod smoketests {
    use crate::math::jacobi_zeta;

    #[test]
    fn test_jacobi_zeta() {
        assert!(jacobi_zeta(0.5, 1.0).is_finite());
    }
}
