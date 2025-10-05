//! boost/math/special_functions/powm1.hpp

use crate::ffi;

/// Computes *b<sup>x</sup> - 1*
///
/// See [`expm1`](crate::math::expm1) for the special case *e<sup>x</sup> - 1*.
///
/// Corresponds to `boost::math::powm1(x, y)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/powers/powm1.html>
pub fn powm1(b: f64, x: f64) -> f64 {
    if x.is_nan() {
        // workaround a bug in Boost where powm1(_, NaN) returns 0.0
        f64::NAN
    } else if x.is_infinite() && b == 1.0 {
        // workaround a bug in Boost where powm1(1.0, INF) returns 0.0
        f64::NAN
    } else if x == 0.0 && (b == 0.0 || b.is_infinite()) {
        // workaround a bug in Boost where powm1(0.0, 0.0) and powm1(INF, 0.0) return 0.0
        f64::NAN
    } else {
        unsafe { ffi::math_powm1(b, x) }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::powm1;
    use core::f64::consts::LN_2;

    const INF: f64 = f64::INFINITY;
    const NAN: f64 = f64::NAN;
    const EPS: f64 = 1e-15;

    #[test]
    fn test_powm1() {
        assert!(powm1(NAN, 1.0).is_nan());
        assert!(powm1(1.0, NAN).is_nan());

        assert!(powm1(INF, 0.0).is_nan());
        assert_eq!(powm1(INF, 1.0), INF);

        assert_eq!(powm1(0.5, INF), -1.0);
        assert!(powm1(1.0, INF).is_nan());
        assert_eq!(powm1(2.0, INF), INF);

        assert_eq!(powm1(0.5, -INF), INF);
        assert!(powm1(1.0, -INF).is_nan());
        assert_eq!(powm1(2.0, -INF), -1.0);

        assert!(powm1(0.0, 0.0).is_nan());
        assert_eq!(powm1(0.5, 0.0), 0.0);
        assert_eq!(powm1(1.0, 0.0), 0.0);
        assert_eq!(powm1(2.0, 0.0), 0.0);

        assert_eq!(powm1(0.0, 1.0), -1.0);
        assert_eq!(powm1(0.5, 1.0), -0.5);
        assert_eq!(powm1(1.0, 1.0), 0.0);
        assert_eq!(powm1(2.0, 1.0), 1.0);

        assert_relative_eq!(powm1(2.0, EPS), EPS * LN_2);
        assert_relative_eq!(powm1(0.5, EPS), -EPS * LN_2);
        assert_relative_eq!(powm1(2.0, -EPS), -EPS * LN_2);
        assert_relative_eq!(powm1(0.5, -EPS), EPS * LN_2);
    }
}
