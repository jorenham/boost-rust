//! boost/math/special_functions/airy.hpp
//!
//! TODO:
//! - Support for `Range` in zero functions

use crate::ffi;
use core::ffi::c_int;

/// Airy function *Ai(x)*
///
/// Corresponds to `boost::math::airy_ai(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/airy/ai.html>
pub fn airy_ai(x: f64) -> f64 {
    unsafe { ffi::math_airy_ai(x) }
}

/// Derivative of [`airy_ai`]
///
/// Corresponds to `boost::math::airy_ai_prime(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/airy/aip.html>
pub fn airy_ai_prime(x: f64) -> f64 {
    unsafe { ffi::math_airy_ai_prime(x) }
}

/// The *k*<sup>th</sup> zero of [`airy_ai`]
///
/// Zero-based indexing: `airy_ai_zero(0)` is the first zero.
///
/// Corresponds to `boost::math::airy_ai_zero(n)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/airy/ai.html>
pub fn airy_ai_zero(k: u32) -> f64 {
    // The +1 is because Boost uses 1-based indexing
    unsafe { ffi::math_airy_ai_zero(k as c_int + 1) }
}

/// Airy function *Bi(x)*
///
/// Corresponds to `boost::math::airy_bi(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/airy/bi.html>
pub fn airy_bi(x: f64) -> f64 {
    unsafe { ffi::math_airy_bi(x) }
}

/// Derivative of [`airy_bi`]
///
/// Corresponds to `boost::math::airy_bi_prime(x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/airy/bip.html>
pub fn airy_bi_prime(x: f64) -> f64 {
    unsafe { ffi::math_airy_bi_prime(x) }
}

/// The *k*<sup>th</sup> zero of [`airy_bi`]
///
/// Zero-based indexing: `airy_bi_zero(0)` is the first zero.
///
/// Corresponds to `boost::math::airy_bi_zero(n)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/airy/bi.html>
pub fn airy_bi_zero(k: u32) -> f64 {
    // The +1 is because Boost uses 1-based indexing
    unsafe { ffi::math_airy_bi_zero(k as c_int + 1) }
}

#[cfg(test)]
mod tests {
    use crate::math::{airy_ai, airy_ai_prime, airy_ai_zero, airy_bi, airy_bi_prime, airy_bi_zero};

    const RTOL: f64 = 5e-16;

    // values from Wolfram Alpha

    #[test]
    fn test_airy_ai() {
        assert_relative_eq!(airy_ai(0.0), 0.355_028_053_887_817_2, epsilon = RTOL);
    }

    #[test]
    fn test_airy_ai_prime() {
        assert_relative_eq!(airy_ai_prime(0.0), -0.258_819_403_792_806_8, epsilon = RTOL);
    }

    #[test]
    fn test_airy_ai_zero() {
        assert_relative_eq!(airy_ai_zero(0), -2.338_107_410_459_767, epsilon = RTOL);
        assert_relative_eq!(airy_ai_zero(1), -4.087_949_444_130_97, epsilon = RTOL);
    }

    #[test]
    fn test_airy_bi() {
        assert_relative_eq!(airy_bi(0.0), 0.614_926_627_446_000_7, epsilon = RTOL);
    }

    #[test]
    fn test_airy_bi_prime() {
        assert_relative_eq!(airy_bi_prime(0.0), 0.448_288_357_353_826_4, epsilon = RTOL);
    }

    #[test]
    fn test_airy_bi_zero() {
        assert_relative_eq!(airy_bi_zero(0), -1.173_713_222_709_128, epsilon = RTOL);
        assert_relative_eq!(airy_bi_zero(1), -3.271_093_302_836_353, epsilon = RTOL);
    }
}
