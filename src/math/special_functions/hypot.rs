//! boost/math/special_functions/hypot.hpp

use crate::ffi;

/// Hypotenuse *(x<sup>2</sup> + y<sup>2</sup>)<sup>1/2</sup>*
///
/// It is computed in such a way as to avoid undue underflow and overflow.
///
/// Corresponds to `boost::math::hypot(x, y)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/hypot.html>
pub fn hypot(x: f64, y: f64) -> f64 {
    unsafe { ffi::math_hypot(x, y) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INF: f64 = f64::INFINITY;
    const NAN: f64 = f64::NAN;

    #[test]
    fn test_hypot() {
        assert!(hypot(NAN, 1.0).is_nan());
        assert!(hypot(1.0, NAN).is_nan());
        assert!(hypot(NAN, NAN).is_nan());

        assert_eq!(hypot(INF, 1.0), INF);
        assert_eq!(hypot(1.0, INF), INF);
        assert_eq!(hypot(INF, INF), INF);
        assert_eq!(hypot(INF, -INF), INF);
        assert_eq!(hypot(-INF, -INF), INF);

        assert_eq!(hypot(0.0, 0.0), 0.0);
        assert_eq!(hypot(3.0, 4.0), 5.0);
        assert_eq!(hypot(-3.0, 4.0), 5.0);
        assert_eq!(hypot(3.0, -4.0), 5.0);
        assert_eq!(hypot(-3.0, -4.0), 5.0);
    }
}
