//! boost/math/special_functions/factorials.hpp

use crate::ffi;
use core::ffi::{c_int, c_uint};

/// Returns *n!* (factorial)
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/factorials/sf_factorial.html>
pub fn factorial(n: u32) -> f64 {
    unsafe { ffi::math_factorial(n as c_uint) }
}

/// Returns *n!!* (double factorial)
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/factorials/sf_double_factorial.html>
pub fn double_factorial(n: u32) -> f64 {
    unsafe { ffi::math_double_factorial(n as c_uint) }
}

/// Falling factorial *x!/(x-n)! = x(x-1)(x-2)...(x-n+1)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/factorials/sf_falling_factorial.html>
pub fn falling_factorial(x: f64, n: u32) -> f64 {
    unsafe { ffi::math_falling_factorial(x, n as c_uint) }
}

/// Rising factorial *x(x+1)(x+2)...(x+n-1)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/factorials/sf_rising_factorial.html>
pub fn rising_factorial(x: f64, n: i32) -> f64 {
    unsafe { ffi::math_rising_factorial(x, n as c_int) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RTOL: f64 = f64::EPSILON;

    #[test]
    fn test_factorial() {
        assert_relative_eq!(factorial(0), 1.0, epsilon = RTOL);
        assert_relative_eq!(factorial(5), 120.0, epsilon = RTOL);
        assert_relative_eq!(factorial(10), 3_628_800.0, epsilon = RTOL);
    }

    #[test]
    fn test_double_factorial() {
        assert_relative_eq!(double_factorial(0), 1.0, epsilon = RTOL);
        assert_relative_eq!(double_factorial(1), 1.0, epsilon = RTOL);
        assert_relative_eq!(double_factorial(5), 15.0, epsilon = RTOL);
        assert_relative_eq!(double_factorial(6), 48.0, epsilon = RTOL);
    }

    #[test]
    fn test_falling_factorial() {
        assert_relative_eq!(falling_factorial(5.0, 0), 1.0, epsilon = RTOL);
        assert_relative_eq!(falling_factorial(5.0, 3), 60.0, epsilon = RTOL);
        assert_relative_eq!(falling_factorial(5.0, 5), 120.0, epsilon = RTOL);
    }

    #[test]
    fn test_rising_factorial() {
        assert_relative_eq!(rising_factorial(5.0, 0), 1.0, epsilon = RTOL);
        assert_relative_eq!(rising_factorial(5.0, 3), 210.0, epsilon = RTOL);
        assert_relative_eq!(rising_factorial(5.0, 5), 15120.0, epsilon = RTOL);
    }
}
