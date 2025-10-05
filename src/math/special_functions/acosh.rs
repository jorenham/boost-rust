//! boost/math/special_functions/acosh.hpp

use crate::ffi;

/// Inverse hyperbolic cosine function *cosh<sup>-1</sup>(x)*
///
/// *cosh<sup>-1</sup>(x) = ln(x + (x<sup>2</sup> - 1)<sup>1/2</sup>)*
///
/// Corresponds to `boost::math::acosh` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/inv_hyper/acosh.html>
pub fn acosh(x: f64) -> f64 {
    unsafe { ffi::math_acosh(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::acosh;

    #[test]
    fn test_acosh() {
        assert_eq!(acosh(1.0), 0.0);
        assert_relative_eq!(acosh(2.0), 1.316_957_896_924_816_8);
    }
}
