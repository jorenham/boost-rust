//! boost/math/special_functions/hypergeometric_0F1.hpp
//! boost/math/special_functions/hypergeometric_1F0.hpp
//! boost/math/special_functions/hypergeometric_1F1.hpp
//! boost/math/special_functions/hypergeometric_2F0.hpp

use core::ffi::c_int;

use crate::ffi;

/// Hypergeometric *<sub>0</sub>F<sub>1</sub>*
///
/// Corresponds to `boost::math::log_hypergeometric_0F1(b, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/hypergeometric/hypergeometric_0f1.html>
pub fn hypergeometric_0f1(b: f64, x: f64) -> f64 {
    unsafe { ffi::math_hypergeometric_0F1(b, x) }
}

/// Hypergeometric *<sub>1</sub>F<sub>0</sub>*
///
/// Corresponds to `boost::math::log_hypergeometric_1F0(a, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/hypergeometric/hypergeometric_1f0.html>
pub fn hypergeometric_1f0(a: f64, x: f64) -> f64 {
    unsafe { ffi::math_hypergeometric_1F0(a, x) }
}

/// Hypergeometric *<sub>1</sub>F<sub>1</sub>*
///
/// Corresponds to `boost::math::log_hypergeometric_1F1(a, b, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/hypergeometric/hypergeometric_1f1.html>
pub fn hypergeometric_1f1(a: f64, b: f64, x: f64) -> f64 {
    unsafe { ffi::math_hypergeometric_1F1(a, b, x) }
}

/// Regularized hypergeometric *<sub>1</sub>F&#771;<sub>1</sub>*
///
/// *<sub>1</sub>F&#771;<sub>1</sub>(a; b; x) = <sub>1</sub>F<sub>1</sub>(a; b; x) / Γ(b)*
///
/// See [`hypergeometric_1f1`] for *<sub>1</sub>F<sub>1</sub>* itself.
///
/// Corresponds to `boost::math::log_hypergeometric_1F1_regularized(a, b, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/hypergeometric/hypergeometric_1f1.html>
pub fn hypergeometric_1f1_regularized(a: f64, b: f64, x: f64) -> f64 {
    unsafe { ffi::math_hypergeometric_1F1_regularized(a, b, x) }
}

/// Logarithm of the absolute value of hypergeometric *<sub>1</sub>F<sub>1</sub>*
///
/// The integer part of the tuple indicates the sign of the hypergeometric function.
///
/// See [`hypergeometric_1f1`] for *<sub>1</sub>F<sub>1</sub>* itself.
///
/// Corresponds to `boost::math::log_hypergeometric_1F1(a, b, x, *sign)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/hypergeometric/hypergeometric_1f1.html>
pub fn log_hypergeometric_1f1(a: f64, b: f64, x: f64) -> (f64, i32) {
    let mut sign: c_int = 0;
    let out = unsafe { ffi::math_log_hypergeometric_1F1(a, b, x, &mut sign) };
    (out, sign)
}

/// Hypergeometric *<sub>2</sub>F<sub>0</sub>*
///
/// Corresponds to `boost::math::hypergeometric_2F0(a1, a2, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/hypergeometric/hypergeometric_2f0.html>
pub fn hypergeometric_2f0(a1: f64, a2: f64, x: f64) -> f64 {
    unsafe { ffi::math_hypergeometric_2F0(a1, a2, x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RTOL: f64 = 2e-15;

    // Numerical values from Wolfram Alpha

    #[test]
    fn test_0f1() {
        assert_eq!(hypergeometric_0f1(2.0, 0.0), 1.0);
        assert_relative_eq!(
            hypergeometric_0f1(2.0, 1.0),
            1.590_636_854_637_329,
            epsilon = RTOL,
        );
        assert_relative_eq!(
            hypergeometric_0f1(2.0, 2.0),
            2.394_833_099_273_405,
            epsilon = RTOL,
        );
        assert_relative_eq!(
            hypergeometric_0f1(2.0, 3.0),
            3.468_649_618_760_533,
            epsilon = RTOL,
        );
    }

    #[test]
    fn test_1f0() {
        assert_relative_eq!(hypergeometric_1f0(2.0, 0.0), 1.0, epsilon = RTOL);
        assert!(hypergeometric_1f0(2.0, 1.0).is_nan());
        assert_relative_eq!(hypergeometric_1f0(2.0, 2.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(hypergeometric_1f0(2.0, 3.0), 0.25, epsilon = RTOL);
        assert_relative_eq!(hypergeometric_1f0(2.0, 4.0), 1.0 / 9.0, epsilon = RTOL);
    }

    #[test]
    fn test_1f1() {
        assert_relative_eq!(hypergeometric_1f1(2.0, 3.0, 0.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(hypergeometric_1f1(2.0, 3.0, 1.0), 2.0, epsilon = RTOL);
        assert_relative_eq!(
            hypergeometric_1f1(2.0, 3.0, 2.0),
            4.194_528_049_465_325,
            epsilon = RTOL,
        );
        assert_relative_eq!(
            hypergeometric_1f1(2.0, 3.0, 3.0),
            9.149_127_521_416_741,
            epsilon = RTOL,
        );
    }

    #[test]
    fn test_1f1_regularized() {
        assert_relative_eq!(
            hypergeometric_1f1_regularized(2.0, 3.0, 0.0),
            0.5,
            epsilon = RTOL
        );
        assert_relative_eq!(
            hypergeometric_1f1_regularized(2.0, 3.0, 1.0),
            1.0,
            epsilon = RTOL
        );
        assert_relative_eq!(
            hypergeometric_1f1_regularized(2.0, 3.0, 2.0),
            2.097_264_024_732_662_6,
            epsilon = RTOL,
        );
        assert_relative_eq!(
            hypergeometric_1f1_regularized(2.0, 3.0, 3.0),
            4.574_563_760_708_370_5,
            epsilon = RTOL,
        );
    }

    #[test]
    fn test_log_1f1() {
        let (val, sign) = log_hypergeometric_1f1(2.0, 3.0, 0.0);
        assert_eq!(val, 0.0);
        assert_eq!(sign, 1);

        let (val, sign) = log_hypergeometric_1f1(2.0, 3.0, 1.0);
        assert_relative_eq!(val, core::f64::consts::LN_2, epsilon = RTOL);
        assert_eq!(sign, 1);

        let (val, sign) = log_hypergeometric_1f1(2.0, 3.0, 2.0);
        assert_relative_eq!(val, 1.433_780_830_483_027_3, epsilon = RTOL);
        assert_eq!(sign, 1);

        let (val, sign) = log_hypergeometric_1f1(2.0, 3.0, 3.0);
        assert_relative_eq!(val, 2.213_658_521_890_428_3, epsilon = RTOL);
        assert_eq!(sign, 1);
    }

    #[test]
    fn test_2f0() {
        assert_eq!(hypergeometric_2f0(2.0, 3.0, 0.0), 1.0);
        assert!(hypergeometric_2f0(2.0, 3.0, 1.0).is_infinite());
        assert!(hypergeometric_2f0(2.0, 3.0, 2.0).is_infinite());
        assert!(hypergeometric_2f0(2.0, 3.0, 3.0).is_infinite());
    }
}
