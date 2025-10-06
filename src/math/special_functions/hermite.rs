//! boost/math/special_functions/hermite.hpp

/// Recurrence relation for [`hermite_h`]
///
/// *H<sub>n+1</sub>(x) = 2xH<sub>n</sub>(x) - 2nH<sub>n-1</sub>(x)*
///
/// # Examples
///
/// ```
/// # use boost::math::{hermite_h, hermite_h_next};
/// let x = 0.42;
/// let p0 = hermite_h(0, x); // 1
/// let p1 = hermite_h(1, x); // 2x
/// let p2 = hermite_h(2, x); // 4x² - 2
/// let p3 = hermite_h(3, x); // 8x³ - 12x
/// assert_eq!(hermite_h_next(1, x, p1, p0), p2);
/// assert_eq!(hermite_h_next(2, x, p2, p1), p3);
/// ```
#[inline(always)]
pub fn hermite_h_next(n: u32, x: f64, pn: f64, pn_prev: f64) -> f64 {
    2.0 * hermite_he_next(n, x, pn, pn_prev)
}

/// Recurrence relation for [`hermite_he`]
///
/// *He<sub>n+1</sub>(x) = x He<sub>n</sub>(x) - n He<sub>n-1</sub>(x)*
///
/// # Examples
///
/// ```
/// # use boost::math::{hermite_he, hermite_he_next};
/// let x = 0.42;
/// let p0 = hermite_he(0, x); // 1
/// let p1 = hermite_he(1, x); // 2x
/// let p2 = hermite_he(2, x); // 4x² - 2
/// let p3 = hermite_he(3, x); // 8x³ - 12x
/// assert_eq!(hermite_he_next(1, x, p1, p0), p2);
/// assert_eq!(hermite_he_next(2, x, p2, p1), p3);
/// ```
#[inline(always)]
pub fn hermite_he_next(n: u32, x: f64, pn: f64, pn_prev: f64) -> f64 {
    x * pn - (n as f64) * pn_prev
}

/// Hermite Polynomial *H<sub>n</sub>(x)*
///
/// Note that this is the  "physicist's" Hermite polynomial.
///
/// This is a pure rust implementation equivalent to the `boost::math::hermite(n, x)` C++ function.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/hermite.html>
pub fn hermite_h(n: u32, x: f64) -> f64 {
    // Implement Hermite polynomials via recurrence:
    let (mut p0, mut p1) = (1.0, 2.0 * x);
    if n == 0 {
        p0
    } else {
        for c in 1..n {
            (p0, p1) = (p1, hermite_h_next(c, x, p1, p0));
        }
        p1
    }
}

/// Monic Hermite Polynomial *He<sub>n</sub>(x)*
///
/// Note that this is the "probabilist's" Hermite polynomial, which is monic (leading coefficient
/// is 1). It is related to the "physicist's" Hermite polynomial ([`hermite_h`]) by
///
/// *He<sub>n</sub>(x) = 2<sup>-n/2</sup> H<sub>n</sub>(x / √2)*
///
/// This function does not exist in the Boost Math C++ library.
pub fn hermite_he(n: u32, x: f64) -> f64 {
    let (mut p0, mut p1) = (1.0, x);
    if n == 0 {
        p0
    } else {
        for c in 1..n {
            (p0, p1) = (p1, hermite_he_next(c, x, p1, p0));
        }
        p1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::FRAC_1_SQRT_2;

    const ATOL: f64 = 1e-15;
    const RTOL: f64 = 1e-12;

    fn hermite_h_ffi(n: u32, x: f64) -> f64 {
        unsafe { crate::ffi::math_hermite(n as core::ffi::c_uint, x) }
    }

    #[test]
    fn test_hermite_h() {
        for n in 0..20 {
            for x in -10..=10 {
                let x = x as f64 * 0.1;
                let h = hermite_h_ffi(n, x);
                assert_relative_eq!(hermite_h(n, x), h, epsilon = ATOL, max_relative = RTOL);
            }
        }
    }

    #[test]
    fn test_hermite_he() {
        for n in 0..20 {
            let c = FRAC_1_SQRT_2.powi(n as i32);
            for x in -10..=10 {
                let x = x as f64 * 0.1;
                let h = c * hermite_h(n, FRAC_1_SQRT_2 * x);
                assert_relative_eq!(hermite_he(n, x), h, epsilon = ATOL, max_relative = RTOL);
            }
        }
    }
}
