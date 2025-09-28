use crate::ffi;

/// Cylindrical Bessel function of the 1st kind *J<sub>ν</sub>(x)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/bessel_first.html>
pub fn cyl_bessel_j(nu: f64, x: f64) -> f64 {
    unsafe { ffi::boost_math_cyl_bessel_j(nu, x) }
}

/// Cylindrical Bessel function of the 2nd kind *Y<sub>ν</sub>(x)* (Neumann function)
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/bessel_first.html>
#[doc(alias = "cyl_bessel_y")]
pub fn cyl_neumann(nu: f64, x: f64) -> f64 {
    unsafe { ffi::boost_math_cyl_neumann(nu, x) }
}

/// Modified cylindrical Bessel function of the 1st kind *I<sub>ν</sub>(x)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/mbessel.html>
pub fn cyl_bessel_i(nu: f64, x: f64) -> f64 {
    unsafe { ffi::boost_math_cyl_bessel_i(nu, x) }
}

/// Modified cylindrical Bessel function of the 2nd kind *K<sub>ν</sub>(x)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/mbessel.html>
pub fn cyl_bessel_k(nu: f64, x: f64) -> f64 {
    unsafe { ffi::boost_math_cyl_bessel_k(nu, x) }
}

/// Spherical Bessel function of the 1st kind *j<sub>n</sub>(x)*
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/sph_bessel.html>
#[doc(alias = "sph_bessel_j")]
pub fn sph_bessel(n: u32, x: f64) -> f64 {
    unsafe { ffi::boost_math_sph_bessel(n, x) }
}

/// Spherical Bessel function of the 2nd kind *y<sub>n</sub>(x)* (spherical Neumann function)
///
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/bessel/sph_bessel.html>
#[doc(alias = "sph_bessel_y")]
pub fn sph_neumann(n: u32, x: f64) -> f64 {
    unsafe { ffi::boost_math_sph_neumann(n, x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyl_bessel_j() {
        assert_eq!(cyl_bessel_j(0.0, 0.0), 1.0);
    }

    #[test]
    fn test_cyl_neumann() {
        assert_abs_diff_eq!(cyl_neumann(-1.0, 0.781078), 1.0, epsilon = 1e-7);
    }

    #[test]
    fn test_cyl_bessel_i() {
        assert_eq!(cyl_bessel_i(0.0, 0.0), 1.0);
    }

    #[test]
    fn test_cyl_bessel_k() {
        assert_abs_diff_eq!(cyl_bessel_k(-1.0, 0.7240853), 1.0, epsilon = 1e-7);
    }

    #[test]
    fn test_sph_bessel() {
        assert_eq!(sph_bessel(0, 0.0), 1.0);
    }

    #[test]
    fn test_sph_neumann() {
        assert_abs_diff_eq!(sph_neumann(0, 0.7390851), -1.0, epsilon = 1e-7);
    }
}
