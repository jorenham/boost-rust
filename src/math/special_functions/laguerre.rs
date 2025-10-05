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
