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
