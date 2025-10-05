//! boost/math/special_functions/chebyshev.hpp

use crate::ffi;

/// Chebyshev polynomial of the first kind T<sub>n</sub>(x).
///
/// Defined as <i>T<sub>n</sub>(</i>cos<i>(θ)) = </i>cos<i>(n θ)</i>.
///
/// See [`chebyshev_t_prime`] for the derivative, and [`chebyshev_u`] for the second kind.
///
/// Corresponds to `boost::math::chebyshev_t` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/chebyshev.html>
pub fn chebyshev_t(n: u32, x: f64) -> f64 {
    unsafe { ffi::math_chebyshev_t(n, x) }
}

/// Derivative of [`chebyshev_t`].
///
/// Corresponds to `boost::math::chebyshev_t_prime` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/chebyshev.html>
#[doc(alias = "chebyshev_t_derivative")]
pub fn chebyshev_t_prime(n: u32, x: f64) -> f64 {
    unsafe { ffi::math_chebyshev_t_prime(n, x) }
}

/// Chebyshev polynomial of the second kind U<sub>n</sub>(x).
///
/// Defined as <i>U<sub>n</sub>(</i>cos<i>(θ)) = </i>sin<i>((n+1) θ) / </i>sin<i>(θ)</i>.
///
/// See [`chebyshev_t`] for the first kind.
///
/// Corresponds to `boost::math::chebyshev_u` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/chebyshev.html>
pub fn chebyshev_u(n: u32, x: f64) -> f64 {
    unsafe { ffi::math_chebyshev_u(n, x) }
}

#[cfg(test)]
mod test {
    use crate::math::{chebyshev_t, chebyshev_t_prime, chebyshev_u};

    #[test]
    fn test_chebyshev_t() {
        assert_relative_eq!(chebyshev_t(0, 0.5), 1.0);
        assert_relative_eq!(chebyshev_t(1, 0.5), 0.5);
        assert_relative_eq!(chebyshev_t(2, 0.5), -0.5);
        assert_relative_eq!(chebyshev_t(3, 0.5), -1.0);
        assert_relative_eq!(chebyshev_t(4, 0.5), -0.5);
        assert_relative_eq!(chebyshev_t(5, 0.5), 0.5);
        assert_relative_eq!(chebyshev_t(6, 0.5), 1.0);
    }

    #[test]
    fn test_chebyshev_t_prime() {
        assert_relative_eq!(chebyshev_t_prime(0, 0.5), 0.0);
        assert_relative_eq!(chebyshev_t_prime(1, 0.5), 1.0);
        assert_relative_eq!(chebyshev_t_prime(2, 0.5), 2.0);
        assert_relative_eq!(chebyshev_t_prime(3, 0.5), 0.0);
        assert_relative_eq!(chebyshev_t_prime(4, 0.5), -4.0);
        assert_relative_eq!(chebyshev_t_prime(5, 0.5), -5.0);
        assert_relative_eq!(chebyshev_t_prime(6, 0.5), 0.0);
    }

    #[test]
    fn test_chebyshev_u() {
        assert_relative_eq!(chebyshev_u(0, 0.5), 1.0);
        assert_relative_eq!(chebyshev_u(1, 0.5), 1.0);
        assert_relative_eq!(chebyshev_u(2, 0.5), 0.0);
        assert_relative_eq!(chebyshev_u(3, 0.5), -1.0);
        assert_relative_eq!(chebyshev_u(4, 0.5), -1.0);
        assert_relative_eq!(chebyshev_u(5, 0.5), 0.0);
        assert_relative_eq!(chebyshev_u(6, 0.5), 1.0);
    }
}
