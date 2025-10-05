//! boost/math/special_functions/cos_pi.hpp

use crate::ffi;

/// Computes *cos(π x)*
///
/// See also [`sin_pi`][crate::math::sin_pi].
///
/// Corresponds to Boost's `boost::math::cos_pi`.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/cos_pi.html>
pub fn cos_pi(x: f64) -> f64 {
    unsafe { ffi::math_cos_pi(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::cos_pi;

    #[test]
    fn test_cos_pi() {
        for &(input, expected) in &[
            (0.0, 1.0),
            (0.5, 0.0),
            (1.0, -1.0),
            (1.5, 0.0),
            (2.0, 1.0),
            (-0.5, 0.0),
            (-1.0, -1.0),
            (-1.5, 0.0),
            (-2.0, 1.0),
            (2.5, 0.0),
            (3.5, 0.0),
        ] {
            let result = cos_pi(input);
            assert!(
                (result - expected).abs() < 1e-10,
                "cos_pi({}) = {}, expected {}",
                input,
                result,
                expected
            );
        }

        for &x in &[f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            let result = cos_pi(x);
            assert!(result.is_nan(), "cos_pi({}) = {}, expected NaN", x, result);
        }
    }
}
