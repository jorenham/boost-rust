//! boost/math/special_functions/hankel.hpp

use crate::ffi;

/// Cyclic Hankel function of the first kind, *H<sub>ν</sub><sup>(1)</sup>(x)*
///
/// The cyclic Hankel function of the first kind is defined by
///
/// *H<sub>ν</sub><sup>(1)</sup>(x) = J<sub>ν</sub>(x) + i Y<sub>ν</sub>(x)*
///
/// where *J<sub>ν</sub>* and *y<sub>ν</sub>* are the cyclic Bessel functions of the first and
/// second kinds, [`cyl_bessel_j`](crate::math::cyl_bessel_j) and
/// [`cyl_neumann`](crate::math::cyl_neumann), respectively, and *i* is the imaginary unit.
///
/// See [`cyl_hankel_2`] for the cyclic Hankel function of the second kind.
///
/// Corresponds to `boost::math::cyl_hankel_1` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/hankel/cyl_hankel.html>
pub fn cyl_hankel_1(nu: f64, x: f64) -> num_complex::Complex<f64> {
    let mut re = f64::NAN;
    let mut im = f64::NAN;
    unsafe { ffi::math_cyl_hankel_1(nu, x, &mut re, &mut im) };
    num_complex::Complex::new(re, im)
}

/// Cyclic Hankel function of the second kind, *H<sub>ν</sub><sup>(2)</sup>(x)*
///
/// The cyclic Hankel function of the second kind is defined by
///
/// *H<sub>ν</sub><sup>(2)</sup>(x) = J<sub>ν</sub>(x) - i Y<sub>ν</sub>(x)*
///
/// where *J<sub>ν</sub>* and *y<sub>ν</sub>* are the cyclic Bessel functions of the first and
/// second kinds, [`cyl_bessel_j`](crate::math::cyl_bessel_j) and
/// [`cyl_neumann`](crate::math::cyl_neumann), respectively, and *i* is the imaginary unit.
///
/// See [`cyl_hankel_1`] for the cyclic Hankel function of the first kind.
///
/// Corresponds to `boost::math::cyl_hankel_2` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/hankel/cyl_hankel.html>
pub fn cyl_hankel_2(nu: f64, x: f64) -> num_complex::Complex<f64> {
    let mut re = f64::NAN;
    let mut im = f64::NAN;
    unsafe { ffi::math_cyl_hankel_2(nu, x, &mut re, &mut im) };
    num_complex::Complex::new(re, im)
}

/// Spherical Hankel function of the first kind, *h<sub>ν</sub><sup>(1)</sup>(x)*
///
/// The spherical Hankel function of the first kind is defined by
///
/// *h<sub>ν</sub><sup>(1)</sup>(x) = j<sub>ν</sub>(x) + i y<sub>ν</sub>(x)*
///
/// where *j<sub>ν</sub>* and *y<sub>ν</sub>* are the spherical Bessel functions of the first and
/// second kinds, [`sph_bessel`](crate::math::sph_bessel) and
/// [`sph_neumann`](crate::math::sph_neumann), respectively, and *i* is the imaginary unit.
///
/// See [`sph_hankel_2`] for the spherical Hankel function of the second kind.
///
/// Corresponds to `boost::math::sph_hankel_1` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/hankel/sph_hankel.html>
pub fn sph_hankel_1(nu: f64, x: f64) -> num_complex::Complex<f64> {
    let mut re = f64::NAN;
    let mut im = f64::NAN;
    unsafe { ffi::math_sph_hankel_1(nu, x, &mut re, &mut im) };
    num_complex::Complex::new(re, im)
}

/// Spherical Hankel function of the second kind, *h<sub>ν</sub><sup>(2)</sup>(x)*
///
/// The spherical Hankel function of the second kind is defined by
///
/// *h<sub>ν</sub><sup>(2)</sup>(x) = j<sub>ν</sub>(x) - i y<sub>ν</sub>(x)*
///
/// where *j<sub>ν</sub>* and *y<sub>ν</sub>* are the spherical Bessel functions of the first and
/// second kinds, [`sph_bessel`](crate::math::sph_bessel) and
/// [`sph_neumann`](crate::math::sph_neumann), respectively, and *i* is the imaginary unit.
///
/// See [`sph_hankel_1`] for the spherical Hankel function of the first kind.
///
/// Corresponds to `boost::math::sph_hankel_2` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/hankel/sph_hankel.html>
pub fn sph_hankel_2(nu: f64, x: f64) -> num_complex::Complex<f64> {
    let mut re = f64::NAN;
    let mut im = f64::NAN;
    unsafe { ffi::math_sph_hankel_2(nu, x, &mut re, &mut im) };
    num_complex::Complex::new(re, im)
}

#[cfg(all(feature = "num-complex", test))]
mod tests {
    use crate::math::{cyl_hankel_1, cyl_hankel_2, sph_hankel_1, sph_hankel_2};
    use num_complex::Complex;

    #[test]
    fn test_cyl_hankel_1() {
        // -0.2422684576748739 + 1.471472392670243 i
        assert_abs_diff_eq!(
            cyl_hankel_1(-1.0, 0.5),
            Complex::new(-0.242_268_457_674_873_9, 1.471_472_392_670_243),
        );
    }

    #[test]
    fn test_cyl_hankel_2() {
        // -0.2422684576748739 - 1.471472392670243 i
        assert_abs_diff_eq!(
            cyl_hankel_2(-1.0, 0.5),
            Complex::new(-0.242_268_457_674_873_9, -1.471_472_392_670_243),
        );
    }

    #[test]
    fn test_sph_hankel_1() {
        // 1.7551651237807454 + 0.958851077208406 i
        assert_abs_diff_eq!(
            sph_hankel_1(-1.0, 0.5),
            Complex::new(1.755_165_123_780_745_4, 0.958_851_077_208_406),
        );
    }

    #[test]
    fn test_sph_hankel_2() {
        // 1.7551651237807454 - 0.958851077208406 i
        assert_abs_diff_eq!(
            sph_hankel_2(-1.0, 0.5),
            Complex::new(1.755_165_123_780_745_4, -0.958_851_077_208_406),
        );
    }
}
