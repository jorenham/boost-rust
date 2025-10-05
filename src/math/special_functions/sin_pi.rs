//! boost/math/special_functions/sin_pi.hpp

use crate::ffi;

/// Computes *sin(π x)*
///
/// See also [`cos_pi`][crate::math::cos_pi].
///
/// Corresponds to Boost's `boost::math::sin_pi`.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/sin_pi.html>
pub fn sin_pi(x: f64) -> f64 {
    unsafe { ffi::math_sin_pi(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::sin_pi;

    #[test]
    fn test_sin_pi() {
        for &(input, expected) in &[
            (0.0, 0.0),
            (0.5, 1.0),
            (1.0, 0.0),
            (1.5, -1.0),
            (2.0, 0.0),
            (-0.5, -1.0),
            (-1.0, 0.0),
            (-1.5, 1.0),
            (-2.0, 0.0),
            (2.5, 1.0),
            (3.5, -1.0),
        ] {
            let result = sin_pi(input);
            assert!(
                (result - expected).abs() < 1e-10,
                "sin_pi({}) = {}, expected {}",
                input,
                result,
                expected
            );
        }

        for &x in &[f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            let result = sin_pi(x);
            assert!(result.is_nan(), "sin_pi({}) = {}, expected NaN", x, result);
        }
    }
}
