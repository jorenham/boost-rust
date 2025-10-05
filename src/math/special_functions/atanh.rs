//! boost/math/special_functions/atanh.hpp

use crate::ffi;

/// Inverse hyperbolic sine function *tanh<sup>-1</sup>(x)*
///
/// *tanh<sup>-1</sup>(x) = ln(x + (x<sup>2</sup> + 1)<sup>1/2</sup>)*
///
/// Corresponds to `boost::math::atanh` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/inv_hyper/atanh.html>
pub fn atanh(x: f64) -> f64 {
    unsafe { ffi::math_atanh(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::atanh;

    #[test]
    fn test_atanh() {
        assert_eq!(atanh(0.0), 0.0);
        assert_relative_eq!(atanh(0.5), 0.549_306_144_334_054_9);
        assert_eq!(atanh(1.0), f64::INFINITY);
    }
}
