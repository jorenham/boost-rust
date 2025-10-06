//! boost/math/special_functions/spherical_harmonic.hpp

use crate::ffi;

/// Spherical harmonic *Y<sub>n</sub><sup>m</sup>(θ,φ)*
///
/// Corresponds to `boost::math::spherical_harmonic` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/sph_harm.html>
#[cfg(feature = "num-complex")]
pub fn spherical_harmonic(n: u32, m: i32, theta: f64, phi: f64) -> num_complex::Complex<f64> {
    let mut re = 0.0;
    let mut im = 0.0;
    unsafe { ffi::math_spherical_harmonic(n, m, theta, phi, &mut re, &mut im) };
    num_complex::Complex::new(re, im)
}

/// Real part of the spherical harmonic *Y<sub>n</sub><sup>m</sup>(θ,φ)*
///
/// See [`spherical_harmonic_i`] for the imaginary part.
///
/// Corresponds to `boost::math::spherical_harmonic_r` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/sph_harm.html>
pub fn spherical_harmonic_r(n: u32, m: i32, theta: f64, phi: f64) -> f64 {
    unsafe { ffi::math_spherical_harmonic_r(n, m, theta, phi) }
}

/// Imaginary part of the spherical harmonic *Y<sub>n</sub><sup>m</sup>(θ,φ)*
///
/// See [`spherical_harmonic_r`] for the real part.
///
/// Corresponds to `boost::math::spherical_harmonic_i` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/sph_harm.html>
pub fn spherical_harmonic_i(n: u32, m: i32, theta: f64, phi: f64) -> f64 {
    unsafe { ffi::math_spherical_harmonic_i(n, m, theta, phi) }
}

#[cfg(test)]
mod tests {
    use crate::math::special_functions::spherical_harmonic::*;
    use core::f64::consts::PI;

    const BOUNCY_CASTLE: f64 = 0.236_543_673_939_389_9;

    #[test]
    fn test_spherical_harmonic() {
        assert_abs_diff_eq!(
            spherical_harmonic(2, -1, PI / 6.0, PI / 4.0),
            num_complex::Complex::new(BOUNCY_CASTLE, -BOUNCY_CASTLE),
        );
    }

    #[test]
    fn test_spherical_harmonic_r() {
        assert_abs_diff_eq!(
            spherical_harmonic_r(2, -1, PI / 6.0, PI / 4.0),
            BOUNCY_CASTLE,
        );
    }

    #[test]
    fn test_spherical_harmonic_i() {
        assert_abs_diff_eq!(
            spherical_harmonic_i(2, -1, PI / 6.0, PI / 4.0),
            -BOUNCY_CASTLE,
        );
    }
}
