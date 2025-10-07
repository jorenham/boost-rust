//! boost/math/special_functions/laguerre.hpp

use crate::ffi;
use core::ffi::c_uint;

/// Laguerre Polynomial *L<sub>n</sub>(x)*
///
/// See [`laguerre_assoc`] for the associated Laguerre polynomial *L<sub>n</sub><sup>m</sup>(x)*
///
/// Corresponds to `boost::math::laguerre(n, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/laguerre.html>
pub fn laguerre(n: u32, x: f64) -> f64 {
    unsafe { ffi::math_laguerre(n as c_uint, x) }
}

/// Associated Laguerre Polynomial *L<sub>n</sub><sup>m</sup>(x)*
///
/// See [`laguerre`] for the Laguerre polynomial *L<sub>n</sub>(x)*
///
/// Corresponds to `boost::math::laguerre(n, m, x)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/laguerre.html>
pub fn laguerre_assoc(n: u32, m: u32, x: f64) -> f64 {
    unsafe { ffi::math_laguerre_assoc(n as c_uint, m as c_uint, x) }
}

/// Recurrence relation for [`laguerre`]
///
/// *(n+1)L<sub>n+1</sub>(x) = (2n+1-x)L<sub>n</sub>(x) - nL<sub>n-1</sub>(x)*
///
/// # Examples
///
/// ```
/// # use approx::assert_relative_eq;
/// # use boost::math::{laguerre, laguerre_next};
/// let x = 0.42;
/// let l0 = laguerre(0, x); // 1
/// let l1 = laguerre(1, x); // -x + 1
/// let l2 = laguerre(2, x); // (x² - 4x + 2) / 2
/// let l3 = laguerre(3, x); // (-x³ + 9x² - 18x + 6) / 6
/// assert_relative_eq!(laguerre_next(1, x, l1, l0), l2);
/// assert_relative_eq!(laguerre_next(2, x, l2, l1), l3);
/// ```
///
/// # See also
///
/// - [`laguerre`]
/// - [`laguerre_assoc_next`]
#[inline(always)]
#[allow(non_snake_case)]
pub fn laguerre_next(n: u32, x: f64, Ln: f64, Ln_prev: f64) -> f64 {
    laguerre_assoc_next(n, 0, x, Ln, Ln_prev)
}

/// Recurrence relation for [`laguerre_assoc`]
///
/// *(n+1)L<sub>n+1</sub><sup>m</sup>(x)
/// = (2n+m+1-x)L<sub>n</sub><sup>m</sup>(x) - (n+m)L<sub>n-1</sub><sup>m</sup>(x)*
///
/// # Examples
///
/// ```
/// # use approx::assert_relative_eq;
/// # use boost::math::{laguerre_assoc, laguerre_assoc_next};
/// let m = 3;
/// let x = 0.42;
/// let l0 = laguerre_assoc(0, m, x);
/// let l1 = laguerre_assoc(1, m, x);
/// let l2 = laguerre_assoc(2, m, x);
/// let l3 = laguerre_assoc(3, m, x);
/// assert_relative_eq!(laguerre_assoc_next(1, m, x, l1, l0), l2);
/// assert_relative_eq!(laguerre_assoc_next(2, m, x, l2, l1), l3);
/// ```
///
/// # See also
///
/// - [`laguerre_assoc`]
/// - [`laguerre_next`]
#[inline(always)]
#[allow(non_snake_case)]
pub fn laguerre_assoc_next(n: u32, m: u32, x: f64, Ln: f64, Ln_prev: f64) -> f64 {
    (((2 * n + m + 1) as f64 - x) * Ln - (n + m) as f64 * Ln_prev) / (n + 1) as f64
}

#[cfg(test)]
mod tests {
    use crate::math::{laguerre, laguerre_assoc};

    const RTOL: f64 = 1e-15;

    #[test]
    fn test_laguerre() {
        assert_relative_eq!(laguerre(1, 0.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(laguerre(2, 0.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(laguerre(3, 0.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(laguerre(4, 0.0), 1.0, epsilon = RTOL);

        assert_relative_eq!(laguerre(1, 1.0), 0.0, epsilon = RTOL);
        assert_relative_eq!(laguerre(2, 1.0), -0.5, epsilon = RTOL);
        assert_relative_eq!(laguerre(3, 1.0), -2.0 / 3.0, epsilon = RTOL);
        assert_relative_eq!(laguerre(4, 1.0), -5.0 / 8.0, epsilon = RTOL);
    }

    #[test]
    fn test_laguerre_assoc() {
        assert_relative_eq!(laguerre_assoc(1, 0, 0.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(2, 0, 0.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(3, 0, 0.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(4, 0, 0.0), 1.0, epsilon = RTOL);

        assert_relative_eq!(laguerre_assoc(1, 0, 1.0), 0.0, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(2, 0, 1.0), -0.5, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(3, 0, 1.0), -2.0 / 3.0, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(4, 0, 1.0), -5.0 / 8.0, epsilon = RTOL);

        assert_relative_eq!(laguerre_assoc(1, 1, 0.0), 2.0, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(2, 1, 0.0), 3.0, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(3, 1, 0.0), 4.0, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(4, 1, 0.0), 5.0, epsilon = RTOL);

        assert_relative_eq!(laguerre_assoc(1, 1, 1.0), 1.0, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(2, 1, 1.0), 0.5, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(3, 1, 1.0), -1.0 / 6.0, epsilon = RTOL);
        assert_relative_eq!(laguerre_assoc(4, 1, 1.0), -19.0 / 24.0, epsilon = RTOL);
    }
}
