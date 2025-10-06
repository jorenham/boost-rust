//! boost/math/special_functions/hermite.hpp

/// Recurrence relation for [`hermite_h`]
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
/// assert_eq!(hermite_next(1, x, h1, h0), h2);
/// assert_eq!(hermite_next(2, x, h2, h1), h3);
/// ```
#[allow(non_snake_case)]
#[inline(always)]
pub fn hermite_h_next(n: u32, x: f64, Hn: f64, Hn_prev: f64) -> f64 {
    2.0 * (x * Hn - (n as f64) * Hn_prev)
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

#[cfg(test)]
mod tests {
    use super::*;

    const RTOL: f64 = 1e-14;

    fn hermite_h_ffi(n: u32, x: f64) -> f64 {
        unsafe { crate::ffi::math_hermite(n as core::ffi::c_uint, x) }
    }

    #[test]
    fn test_hermite() {
        for n in 0..20 {
            for x in -2..=2 {
                let x = x as f64 * 0.1;
                assert_relative_eq!(hermite_h(n, x), hermite_h_ffi(n, x), max_relative = RTOL);
            }
        }
    }
}
