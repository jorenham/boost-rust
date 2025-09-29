//! Raw FFI declarations for wrapper.cpp

use core::ffi::{c_int, c_uint};

unsafe extern "C" {
    // boost/math/special_functions/bessel.hpp
    pub fn math_cyl_bessel_j(nu: f64, x: f64) -> f64;
    pub fn math_cyl_neumann(nu: f64, x: f64) -> f64;
    pub fn math_cyl_bessel_i(nu: f64, x: f64) -> f64;
    pub fn math_cyl_bessel_k(nu: f64, x: f64) -> f64;
    pub fn math_sph_bessel(n: c_uint, x: f64) -> f64;
    pub fn math_sph_neumann(n: c_uint, x: f64) -> f64;

    // boost/math/special_functions/beta.hpp
    pub fn math_beta(a: f64, b: f64) -> f64;
    pub fn math_beta_(a: f64, b: f64, x: f64) -> f64;
    pub fn math_betac(a: f64, b: f64, x: f64) -> f64;
    pub fn math_ibeta(a: f64, b: f64, x: f64) -> f64;
    pub fn math_ibeta_derivative(a: f64, b: f64, x: f64) -> f64;
    pub fn math_ibeta_inv(a: f64, b: f64, p: f64) -> f64;
    pub fn math_ibeta_inva(b: f64, x: f64, p: f64) -> f64;
    pub fn math_ibeta_invb(a: f64, x: f64, p: f64) -> f64;
    pub fn math_ibetac(a: f64, b: f64, x: f64) -> f64;
    pub fn math_ibetac_inv(a: f64, b: f64, q: f64) -> f64;
    pub fn math_ibetac_inva(b: f64, x: f64, q: f64) -> f64;
    pub fn math_ibetac_invb(a: f64, x: f64, q: f64) -> f64;

    // boost/math/special_functions/digamma.hpp
    pub fn math_digamma(x: f64) -> f64;

    // boost/math/special_functions/erf.hpp
    pub fn math_erf(x: f64) -> f64;
    pub fn math_erfc(x: f64) -> f64;

    // boost/math/special_functions/factorials.hpp
    pub fn math_factorial(i: c_uint) -> f64;
    pub fn math_double_factorial(i: c_uint) -> f64;
    pub fn math_falling_factorial(x: f64, n: c_uint) -> f64;
    pub fn math_rising_factorial(x: f64, n: c_int) -> f64;

    // boost/math/special_functions/gamma.hpp
    pub fn math_tgamma(x: f64) -> f64;
    pub fn math_tgamma1pm1(x: f64) -> f64;
    pub fn math_lgamma(x: f64, sign: *mut c_int) -> f64;
    pub fn math_tgamma_lower(a: f64, x: f64) -> f64;
    pub fn math_tgamma_upper(a: f64, x: f64) -> f64;
    pub fn math_gamma_p(a: f64, x: f64) -> f64;
    pub fn math_gamma_q(a: f64, x: f64) -> f64;

    // boost/math/special_functions/jacobi.hpp
    pub fn math_jacobi(n: c_uint, alpha: f64, beta: f64, x: f64) -> f64;
    pub fn math_jacobi_derivative(n: c_uint, alpha: f64, beta: f64, x: f64, k: c_uint) -> f64;

    // boost/math/special_functions/legendre.hpp
    pub fn math_legendre_p(l: c_int, x: f64) -> f64;
    pub fn math_legendre_p_assoc(l: c_int, m: c_int, x: f64) -> f64;
    pub fn math_legendre_p_prime(l: c_int, x: f64) -> f64;
    pub fn math_legendre_p_zeros(l: c_int, out: *mut f64);
    pub fn math_legendre_q(l: c_uint, x: f64) -> f64;
}
