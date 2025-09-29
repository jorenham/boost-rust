//! boost/math/special_functions/zeta.hpp

use crate::ffi;

/// Riemann zeta function *ζ(s)*
///
/// See <https://mathworld.wolfram.com/RiemannZetaFunction.html> for more information.
///
/// Corresponds to `boost::math::zeta(s)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/zetas/zeta.html>
pub fn zeta(s: f64) -> f64 {
    unsafe { ffi::math_zeta(s) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::PI;

    #[test]
    fn test_zeta() {
        assert_relative_eq!(zeta(2.0), PI.powi(2) / 6.0, epsilon = f64::EPSILON);
    }
}
