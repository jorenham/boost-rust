use super::super::ffi;

/// Beta function *B(a,b) = Γ(a) Γ(b) / Γ(a + b)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_beta/beta_function.html>
///
/// Defined in `boost/math/special_functions/beta.hpp`
pub fn beta(a: f64, b: f64) -> f64 {
    unsafe { ffi::boost_math_beta(a, b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beta_symmetry() {
        let (a, b) = (2.0, 3.0);
        assert_eq!(beta(a, b), beta(b, a));
    }

    #[test]
    fn test_beta_literal() {
        assert_eq!(beta(1.0, 1.0), 1.0);
        assert_eq!(beta(5.0, 1.0), 0.2);
    }

    #[test]
    fn test_beta_nan() {
        assert!(beta(f64::NAN, 1.0).is_nan());
        assert!(beta(1.0, f64::NAN).is_nan());
    }

    #[test]
    fn test_beta_invalid() {
        assert!(beta(-1.0, -1.0).is_nan());
        assert!(beta(-1.0, 1.0).is_nan());
        assert!(beta(1.0, -1.0).is_nan());
    }
}
