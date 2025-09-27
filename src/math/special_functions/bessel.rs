use super::super::ffi;

/// Cylindrical Bessel function of the first kind J_ν(x)
pub fn cyl_bessel_j(nu: f64, x: f64) -> f64 {
    unsafe { ffi::boost_math_cyl_bessel_j(nu, x) }
}

/// Cylindrical Bessel function of the second kind Y_ν(x) (Neumann function)
pub fn cyl_neumann(nu: f64, x: f64) -> f64 {
    unsafe { ffi::boost_math_cyl_neumann(nu, x) }
}

/// Modified cylindrical Bessel function of the first kind I_ν(x)
pub fn cyl_bessel_i(nu: f64, x: f64) -> f64 {
    unsafe { ffi::boost_math_cyl_bessel_i(nu, x) }
}

/// Modified cylindrical Bessel function of the second kind K_ν(x)
pub fn cyl_bessel_k(nu: f64, x: f64) -> f64 {
    unsafe { ffi::boost_math_cyl_bessel_k(nu, x) }
}

/// Spherical Bessel function of the first kind j_n(x)
pub fn sph_bessel(n: u32, x: f64) -> f64 {
    unsafe { ffi::boost_math_sph_bessel(n, x) }
}

/// Spherical Bessel function of the second kind y_n(x) (spherical Neumann function)
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
