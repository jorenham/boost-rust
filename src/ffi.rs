//! Raw FFI declarations for wrapper.cpp

use core::ffi::{c_int, c_uint};

unsafe extern "C" {
    // boost/math/ccmath/sqrt.hpp
    pub fn math_ccmath_sqrt(x: f64) -> f64;

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

    // boost/math/special_functions/binomial.hpp
    pub fn math_binomial_coefficient(n: c_uint, k: c_uint) -> f64;

    // boost/math/special_functions/cbrt.hpp
    pub fn math_cbrt(x: f64) -> f64;

    // boost/math/special_functions/cos_pi.hpp
    pub fn math_cos_pi(x: f64) -> f64;

    // boost/math/special_functions/digamma.hpp
    pub fn math_digamma(x: f64) -> f64;

    // boost/math/special_functions/erf.hpp
    pub fn math_erf(x: f64) -> f64;
    pub fn math_erfc(x: f64) -> f64;
    pub fn math_erf_inv(p: f64) -> f64;
    pub fn math_erfc_inv(q: f64) -> f64;

    // boost/math/special_functions/expm1.hpp
    pub fn math_expm1(x: f64) -> f64;

    // boost/math/special_functions/factorials.hpp
    pub fn math_factorial(i: c_uint) -> f64;
    pub fn math_double_factorial(i: c_uint) -> f64;
    pub fn math_falling_factorial(x: f64, n: c_uint) -> f64;
    pub fn math_rising_factorial(x: f64, n: c_int) -> f64;

    // boost/math/special_functions/gamma.hpp
    pub fn math_tgamma(x: f64) -> f64;
    pub fn math_tgamma1pm1(x: f64) -> f64;
    pub fn math_tgamma_(a: f64, x: f64) -> f64;
    pub fn math_tgamma_lower(a: f64, x: f64) -> f64;
    pub fn math_tgamma_ratio(a: f64, b: f64) -> f64;
    pub fn math_tgamma_delta_ratio(x: f64, delta: f64) -> f64;
    pub fn math_lgamma(x: f64, sign: *mut c_int) -> f64;
    pub fn math_gamma_q(a: f64, x: f64) -> f64;
    pub fn math_gamma_q_inv(a: f64, q: f64) -> f64;
    pub fn math_gamma_q_inva(x: f64, q: f64) -> f64;
    pub fn math_gamma_p(a: f64, x: f64) -> f64;
    pub fn math_gamma_p_inv(a: f64, p: f64) -> f64;
    pub fn math_gamma_p_inva(x: f64, p: f64) -> f64;
    pub fn math_gamma_p_derivative(a: f64, x: f64) -> f64;

    // boost/math/special_functions/hermite.hpp
    pub fn math_hermite(n: c_uint, x: f64) -> f64;

    // boost/math/special_functions/hypergeometric_0F1.hpp
    pub fn math_hypergeometric_0F1(b: f64, x: f64) -> f64;

    // boost/math/special_functions/hypergeometric_1F0.hpp
    pub fn math_hypergeometric_1F0(a: f64, x: f64) -> f64;

    // boost/math/special_functions/hypergeometric_1F1.hpp
    pub fn math_hypergeometric_1F1(a: f64, b: f64, x: f64) -> f64;
    pub fn math_hypergeometric_1F1_regularized(a: f64, b: f64, x: f64) -> f64;
    pub fn math_log_hypergeometric_1F1(a: f64, b: f64, x: f64, sign: *mut c_int) -> f64;

    // boost/math/special_functions/hypergeometric_2F0.hpp
    pub fn math_hypergeometric_2F0(a1: f64, a2: f64, x: f64) -> f64;

    // boost/math/special_functions/hypot.hpp
    pub fn math_hypot(x: f64, y: f64) -> f64;

    // boost/math/special_functions/jacobi.hpp
    pub fn math_jacobi(n: c_uint, alpha: f64, beta: f64, x: f64) -> f64;
    pub fn math_jacobi_derivative(n: c_uint, alpha: f64, beta: f64, x: f64, k: c_uint) -> f64;

    // boost/math/special_functions/laguerre.hpp
    pub fn math_laguerre(n: c_uint, x: f64) -> f64;
    pub fn math_laguerre_assoc(n: c_uint, m: c_uint, x: f64) -> f64;

    // boost/math/special_functions/legendre.hpp
    pub fn math_legendre_p(l: c_int, x: f64) -> f64;
    pub fn math_legendre_p_assoc(l: c_int, m: c_int, x: f64) -> f64;
    pub fn math_legendre_p_prime(l: c_int, x: f64) -> f64;
    pub fn math_legendre_p_zeros(l: c_int, out: *mut f64);
    pub fn math_legendre_q(l: c_uint, x: f64) -> f64;

    // boost/math/special_functions/log1p.hpp
    pub fn math_log1p(x: f64) -> f64;

    // boost/math/special_functions/logsumexp.hpp
    pub fn math_logaddexp(x1: f64, x2: f64) -> f64;
    pub fn math_logsumexp(arr: *const f64, len: usize) -> f64;

    // boost/math/special_functions/polygamma.hpp
    pub fn math_polygamma(n: c_int, x: f64) -> f64;

    // boost/math/special_functions/powm1.hpp
    pub fn math_powm1(x: f64, y: f64) -> f64;

    // boost/math/special_functions/prime.hpp
    pub fn math_prime(n: c_uint) -> u32;

    // boost/math/special_functions/rsqrt.hpp
    pub fn math_rsqrt(x: f64) -> f64;

    // boost/math/special_functions/sin_pi.hpp
    pub fn math_sin_pi(x: f64) -> f64;

    // boost/math/special_functions/sinc.hpp
    pub fn math_sinc_pi(x: f64) -> f64;

    // boost/math/special_functions/sinhc.hpp
    pub fn math_sinhc_pi(x: f64) -> f64;

    // boost/math/special_functions/sqrt1pm1.hpp
    pub fn math_sqrt1pm1(x: f64) -> f64;

    // boost/math/special_functions/trigamma.hpp
    pub fn math_trigamma(x: f64) -> f64;

    // boost/math/special_functions/zeta.hpp
    pub fn math_zeta(s: f64) -> f64;
}
