//! boost/math/special_functions/asinh.hpp

use crate::ffi;

/// Inverse hyperbolic sine function *sinh<sup>-1</sup>(x)*
///
/// *sinh<sup>-1</sup>(x) = ln(x + (x<sup>2</sup> + 1)<sup>1/2</sup>)*
///
/// Corresponds to `boost::math::asinh` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/inv_hyper/asinh.html>
pub fn asinh(x: f64) -> f64 {
    unsafe { ffi::math_asinh(x) }
}

#[cfg(test)]
mod tests {
    use crate::math::asinh;

    #[test]
    fn test_asinh() {
        assert_eq!(asinh(0.0), 0.0);
        assert_relative_eq!(asinh(0.5), 0.481_211_825_059_603_47);
        assert_relative_eq!(asinh(1.0), 0.881_373_587_019_543);
    }
}
