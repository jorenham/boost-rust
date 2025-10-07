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

/// *k*-th derivative of the Hermite polynomial *H<sub>n</sub>(x)*
///
/// This function does not exist in the Boost Math C++ library.
#[doc(alias = "hermite_h_prime")]
pub fn hermite_h_derivative(n: u32, x: f64, k: u32) -> f64 {
    if n < k {
        0.0
    } else {
        let mut p = hermite_h(n - k, x);
        for m in 0..k {
            p *= 2.0 * (n - m) as f64
        }
        p
    }
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

/// *k*-th derivative of the Hermite polynomial *He<sub>n</sub>(x)*
///
/// This function does not exist in the Boost Math C++ library.
#[doc(alias = "hermite_he_prime")]
pub fn hermite_he_derivative(n: u32, x: f64, k: u32) -> f64 {
    if n < k {
        0.0
    } else {
        let mut p = hermite_he(n - k, x);
        for m in 0..k {
            p *= (n - m) as f64
        }
        p
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

    #[test]
    fn test_hermite_h_derivative() {
        for n in 0..20 {
            for x in -10..=10 {
                let x = x as f64 * 0.1;

                let d0 = hermite_h_derivative(n, x, 0);
                assert_relative_eq!(d0, hermite_h(n, x));

                let d1 = hermite_h_derivative(n, x, 1);
                if n >= 1 {
                    assert_relative_eq!(d1, (2 * n) as f64 * hermite_h(n - 1, x));
                } else {
                    assert_eq!(d1, 0.0)
                }

                let d2 = hermite_h_derivative(n, x, 2);
                if n >= 2 {
                    assert_relative_eq!(d2, (4 * n * (n - 1)) as f64 * hermite_h(n - 2, x));
                } else {
                    assert_eq!(d2, 0.0)
                }
            }
        }
    }

    #[test]
    fn test_hermite_he_derivative() {
        for n in 0..20 {
            for x in -10..=10 {
                let x = x as f64 * 0.1;

                let d0 = hermite_he_derivative(n, x, 0);
                assert_relative_eq!(d0, hermite_he(n, x));

                let d1 = hermite_he_derivative(n, x, 1);
                if n >= 1 {
                    assert_relative_eq!(d1, n as f64 * hermite_he(n - 1, x));
                } else {
                    assert_eq!(d1, 0.0)
                }

                let d2 = hermite_he_derivative(n, x, 2);
                if n >= 2 {
                    assert_relative_eq!(d2, (n * (n - 1)) as f64 * hermite_he(n - 2, x));
                } else {
                    assert_eq!(d2, 0.0)
                }
            }
        }
    }
}
