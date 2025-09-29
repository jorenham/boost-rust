use crate::ffi;
use core::ffi::c_uint;

/// Binomial Coefficient *<sub>n</sub>C<sub>k</sub>*
///
/// Requires *k ≤ n*.
///
/// Corresponds to `boost::math::binomial_coefficient<double>(n, k)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/factorials/sf_binomial.html>
pub fn binomial_coefficient(n: u32, k: u32) -> f64 {
    unsafe { ffi::math_binomial_coefficient(n as c_uint, k as c_uint) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_coefficient() {
        assert_eq!(binomial_coefficient(0, 0), 1.0);
        assert!(binomial_coefficient(0, 1).is_nan());

        assert_eq!(binomial_coefficient(6, 0), 1.0);
        assert_eq!(binomial_coefficient(6, 1), 6.0);
        assert_eq!(binomial_coefficient(6, 2), 15.0);
        assert_eq!(binomial_coefficient(6, 3), 20.0);
        assert_eq!(binomial_coefficient(6, 4), 15.0);
        assert_eq!(binomial_coefficient(6, 5), 6.0);
        assert_eq!(binomial_coefficient(6, 6), 1.0);
    }
}
