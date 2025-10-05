//! boost/math/special_functions/hermite.hpp

use crate::ffi;
use core::ffi::c_uint;

/// Hermite Polynomial *H<sub>n</sub>(x)*
///
/// Corresponds to `boost::math::hermite(n, x)`.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/hermite.html>
pub fn hermite(n: u32, x: f64) -> f64 {
    unsafe { ffi::math_hermite(n as c_uint, x) }
}

/// Recurrence relation for [`hermite`]
///
/// *H<sub>n+1</sub>(x) = 2xH<sub>n</sub>(x) - 2nH<sub>n-1</sub>(x)*
///
/// # Examples
///
/// ```
/// # use boost::math::{hermite, hermite_next};
/// let x = 0.42;
/// let h0 = hermite(0, x); // 1
/// let h1 = hermite(1, x); // 2x
/// let h2 = hermite(2, x); // 4x² - 2
/// let h3 = hermite(3, x); // 8x³ - 12x
/// assert_eq!(hermite_next(1, &x, &h1, &h0), h2);
/// assert_eq!(hermite_next(2, &x, &h2, &h1), h3);
/// ```
#[allow(non_snake_case)]
#[inline(always)]
pub fn hermite_next(n: u32, x: &f64, Hn: &f64, Hn_1: &f64) -> f64 {
    2.0 * (x * Hn - (n as f64) * Hn_1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hermite() {
        assert_relative_eq!(hermite(0, 1.0), 1.0);
        assert_relative_eq!(hermite(1, 1.0), 2.0);
        assert_relative_eq!(hermite(2, 1.0), 2.0);
        assert_relative_eq!(hermite(3, 1.0), -4.0);
        assert_relative_eq!(hermite(4, 1.0), -20.0);
    }
}
