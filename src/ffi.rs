//! Raw FFI declarations for wrapper.cpp

use core::ffi::{c_int, c_uint};

unsafe extern "C-unwind" {
    // boost/math/ccmath/sqrt.hpp
    pub(crate) fn math_ccmath_sqrt(x: f64) -> f64;

    // boost/math/special_functions/acosh.hpp
    pub(crate) fn math_acosh(x: f64) -> f64;
    // boost/math/special_functions/asinh.hpp
    pub(crate) fn math_asinh(x: f64) -> f64;
    // boost/math/special_functions/atanh.hpp
    pub(crate) fn math_atanh(x: f64) -> f64;

    // boost/math/special_functions/airy.hpp
    pub(crate) fn math_airy_ai(x: f64) -> f64;
    pub(crate) fn math_airy_ai_prime(x: f64) -> f64;
    pub(crate) fn math_airy_ai_zero(m: c_int) -> f64;
    pub(crate) fn math_airy_bi(x: f64) -> f64;
    pub(crate) fn math_airy_bi_prime(x: f64) -> f64;
    pub(crate) fn math_airy_bi_zero(m: c_int) -> f64;

    // boost/math/special_functions/bessel.hpp
    pub(crate) fn math_cyl_bessel_j(nu: f64, x: f64) -> f64;
    pub(crate) fn math_cyl_neumann(nu: f64, x: f64) -> f64;
    pub(crate) fn math_cyl_bessel_i(nu: f64, x: f64) -> f64;
    pub(crate) fn math_cyl_bessel_k(nu: f64, x: f64) -> f64;
    pub(crate) fn math_sph_bessel(n: c_uint, x: f64) -> f64;
    pub(crate) fn math_sph_neumann(n: c_uint, x: f64) -> f64;
    pub(crate) fn math_cyl_bessel_j_zero(nu: f64, k: c_int) -> f64;
    pub(crate) fn math_cyl_neumann_zero(nu: f64, k: c_int) -> f64;

    // boost/math/special_functions/bessel_prime.hpp
    pub(crate) fn math_cyl_bessel_j_prime(nu: f64, x: f64) -> f64;
    pub(crate) fn math_cyl_neumann_prime(nu: f64, x: f64) -> f64;
    pub(crate) fn math_cyl_bessel_i_prime(nu: f64, x: f64) -> f64;
    pub(crate) fn math_cyl_bessel_k_prime(nu: f64, x: f64) -> f64;
    pub(crate) fn math_sph_bessel_prime(n: c_uint, x: f64) -> f64;
    pub(crate) fn math_sph_neumann_prime(n: c_uint, x: f64) -> f64;

    // boost/math/special_functions/beta.hpp
    pub(crate) fn math_beta(a: f64, b: f64) -> f64;
    pub(crate) fn math_beta_(a: f64, b: f64, x: f64) -> f64;
    pub(crate) fn math_betac(a: f64, b: f64, x: f64) -> f64;
    pub(crate) fn math_ibeta(a: f64, b: f64, x: f64) -> f64;
    pub(crate) fn math_ibeta_derivative(a: f64, b: f64, x: f64) -> f64;
    pub(crate) fn math_ibeta_inv(a: f64, b: f64, p: f64) -> f64;
    pub(crate) fn math_ibeta_inva(b: f64, x: f64, p: f64) -> f64;
    pub(crate) fn math_ibeta_invb(a: f64, x: f64, p: f64) -> f64;
    pub(crate) fn math_ibetac(a: f64, b: f64, x: f64) -> f64;
    pub(crate) fn math_ibetac_inv(a: f64, b: f64, q: f64) -> f64;
    pub(crate) fn math_ibetac_inva(b: f64, x: f64, q: f64) -> f64;
    pub(crate) fn math_ibetac_invb(a: f64, x: f64, q: f64) -> f64;

    // boost/math/special_functions/binomial.hpp
    pub(crate) fn math_binomial_coefficient(n: c_uint, k: c_uint) -> f64;

    // boost/math/special_functions/cbrt.hpp
    pub(crate) fn math_cbrt(x: f64) -> f64;

    // boost/math/special_functions/chebyshev.hpp
    pub(crate) fn math_chebyshev_t(n: c_uint, x: f64) -> f64;
    pub(crate) fn math_chebyshev_t_prime(n: c_uint, x: f64) -> f64;
    pub(crate) fn math_chebyshev_u(n: c_uint, x: f64) -> f64;

    // boost/math/special_functions/cos_pi.hpp
    pub(crate) fn math_cos_pi(x: f64) -> f64;

    // boost/math/special_functions/digamma.hpp
    pub(crate) fn math_digamma(x: f64) -> f64;

    // boost/math/special_functions/ellint_1.hpp
    pub(crate) fn math_ellint_1(k: f64) -> f64;
    pub(crate) fn math_ellint_1_inc(k: f64, phi: f64) -> f64;

    // boost/math/special_functions/ellint_2.hpp
    pub(crate) fn math_ellint_2(k: f64) -> f64;
    pub(crate) fn math_ellint_2_inc(k: f64, phi: f64) -> f64;

    // boost/math/special_functions/ellint_3.hpp
    pub(crate) fn math_ellint_3(k: f64, v: f64) -> f64;
    pub(crate) fn math_ellint_3_inc(k: f64, v: f64, phi: f64) -> f64;

    // boost/math/special_functions/ellint_d.hpp
    pub(crate) fn math_ellint_d(k: f64) -> f64;
    pub(crate) fn math_ellint_d_inc(k: f64, phi: f64) -> f64;

    // boost/math/special_functions/ellint_rc.hpp
    pub(crate) fn math_ellint_rc(x: f64, y: f64) -> f64;
    // boost/math/special_functions/ellint_rd.hpp
    pub(crate) fn math_ellint_rd(x: f64, y: f64, z: f64) -> f64;
    // boost/math/special_functions/ellint_rf.hpp
    pub(crate) fn math_ellint_rf(x: f64, y: f64, z: f64) -> f64;
    // boost/math/special_functions/ellint_rg.hpp
    pub(crate) fn math_ellint_rg(x: f64, y: f64, z: f64) -> f64;
    // boost/math/special_functions/ellint_rj.hpp
    pub(crate) fn math_ellint_rj(x: f64, y: f64, z: f64, p: f64) -> f64;

    // boost/math/special_functions/erf.hpp
    pub(crate) fn math_erf(x: f64) -> f64;
    pub(crate) fn math_erfc(x: f64) -> f64;
    pub(crate) fn math_erf_inv(p: f64) -> f64;
    pub(crate) fn math_erfc_inv(q: f64) -> f64;

    // boost/math/special_functions/expint.hpp
    pub(crate) fn math_expint_i(x: f64) -> f64;
    pub(crate) fn math_expint_n(n: c_uint, x: f64) -> f64;

    // boost/math/special_functions/expm1.hpp
    pub(crate) fn math_expm1(x: f64) -> f64;

    // boost/math/special_functions/factorials.hpp
    pub(crate) fn math_factorial(i: c_uint) -> f64;
    pub(crate) fn math_double_factorial(i: c_uint) -> f64;
    pub(crate) fn math_falling_factorial(x: f64, n: c_uint) -> f64;
    pub(crate) fn math_rising_factorial(x: f64, n: c_int) -> f64;

    // boost/math/special_functions/gamma.hpp
    pub(crate) fn math_tgamma(x: f64) -> f64;
    pub(crate) fn math_tgamma1pm1(x: f64) -> f64;
    pub(crate) fn math_tgamma_(a: f64, x: f64) -> f64;
    pub(crate) fn math_tgamma_lower(a: f64, x: f64) -> f64;
    pub(crate) fn math_tgamma_ratio(a: f64, b: f64) -> f64;
    pub(crate) fn math_tgamma_delta_ratio(x: f64, delta: f64) -> f64;
    pub(crate) fn math_lgamma(x: f64, sign: *mut c_int) -> f64;
    pub(crate) fn math_gamma_q(a: f64, x: f64) -> f64;
    pub(crate) fn math_gamma_q_inv(a: f64, q: f64) -> f64;
    pub(crate) fn math_gamma_q_inva(x: f64, q: f64) -> f64;
    pub(crate) fn math_gamma_p(a: f64, x: f64) -> f64;
    pub(crate) fn math_gamma_p_inv(a: f64, p: f64) -> f64;
    pub(crate) fn math_gamma_p_inva(x: f64, p: f64) -> f64;
    pub(crate) fn math_gamma_p_derivative(a: f64, x: f64) -> f64;

    // boost/math/special_functions/gegenbauer.hpp
    pub(crate) fn math_gegenbauer(n: c_uint, lambda: f64, x: f64) -> f64;
    pub(crate) fn math_gegenbauer_derivative(n: c_uint, lambda: f64, x: f64, k: c_uint) -> f64;

    // boost/math/special_functions/hermite.hpp
    #[cfg(test)]
    pub(crate) fn math_hermite(n: c_uint, x: f64) -> f64;

    // boost/math/special_functions/heuman_lambda.hpp
    pub(crate) fn math_heuman_lambda(k: f64, phi: f64) -> f64;

    // boost/math/special_functions/hypergeometric_0F1.hpp
    pub(crate) fn math_hypergeometric_0F1(b: f64, x: f64) -> f64;
    // boost/math/special_functions/hypergeometric_1F0.hpp
    pub(crate) fn math_hypergeometric_1F0(a: f64, x: f64) -> f64;
    // boost/math/special_functions/hypergeometric_1F1.hpp
    pub(crate) fn math_hypergeometric_1F1(a: f64, b: f64, x: f64) -> f64;
    pub(crate) fn math_hypergeometric_1F1_regularized(a: f64, b: f64, x: f64) -> f64;
    pub(crate) fn math_log_hypergeometric_1F1(a: f64, b: f64, x: f64, sign: *mut c_int) -> f64;
    // boost/math/special_functions/hypergeometric_2F0.hpp
    pub(crate) fn math_hypergeometric_2F0(a1: f64, a2: f64, x: f64) -> f64;

    // boost/math/special_functions/hypot.hpp
    pub(crate) fn math_hypot(x: f64, y: f64) -> f64;

    // boost/math/special_functions/jacobi.hpp
    pub(crate) fn math_jacobi(n: c_uint, alpha: f64, beta: f64, x: f64) -> f64;
    pub(crate) fn math_jacobi_derivative(
        n: c_uint,
        alpha: f64,
        beta: f64,
        x: f64,
        k: c_uint,
    ) -> f64;

    // boost/math/special_functions/jacobi_elliptic.hpp
    pub(crate) fn math_jacobi_elliptic(k: f64, theta: f64, pcn: *mut f64, pdn: *mut f64) -> f64;
    pub(crate) fn math_jacobi_cd(k: f64, theta: f64) -> f64;
    pub(crate) fn math_jacobi_cn(k: f64, theta: f64) -> f64;
    pub(crate) fn math_jacobi_cs(k: f64, theta: f64) -> f64;
    pub(crate) fn math_jacobi_dc(k: f64, theta: f64) -> f64;
    pub(crate) fn math_jacobi_dn(k: f64, theta: f64) -> f64;
    pub(crate) fn math_jacobi_ds(k: f64, theta: f64) -> f64;
    pub(crate) fn math_jacobi_nc(k: f64, theta: f64) -> f64;
    pub(crate) fn math_jacobi_nd(k: f64, theta: f64) -> f64;
    pub(crate) fn math_jacobi_ns(k: f64, theta: f64) -> f64;
    pub(crate) fn math_jacobi_sc(k: f64, theta: f64) -> f64;
    pub(crate) fn math_jacobi_sd(k: f64, theta: f64) -> f64;
    pub(crate) fn math_jacobi_sn(k: f64, theta: f64) -> f64;

    // boost/math/special_functions/jacobi_theta.hpp
    pub(crate) fn math_jacobi_theta1(x: f64, q: f64) -> f64;
    pub(crate) fn math_jacobi_theta1tau(x: f64, tau: f64) -> f64;
    pub(crate) fn math_jacobi_theta2(x: f64, q: f64) -> f64;
    pub(crate) fn math_jacobi_theta2tau(x: f64, tau: f64) -> f64;
    pub(crate) fn math_jacobi_theta3(x: f64, q: f64) -> f64;
    pub(crate) fn math_jacobi_theta3tau(x: f64, tau: f64) -> f64;
    pub(crate) fn math_jacobi_theta3m1(x: f64, q: f64) -> f64;
    pub(crate) fn math_jacobi_theta3m1tau(x: f64, tau: f64) -> f64;
    pub(crate) fn math_jacobi_theta4(x: f64, q: f64) -> f64;
    pub(crate) fn math_jacobi_theta4tau(x: f64, tau: f64) -> f64;
    pub(crate) fn math_jacobi_theta4m1(x: f64, q: f64) -> f64;
    pub(crate) fn math_jacobi_theta4m1tau(x: f64, tau: f64) -> f64;

    // boost/math/special_functions/jacobi_zeta.hpp
    pub(crate) fn math_jacobi_zeta(k: f64, phi: f64) -> f64;

    // boost/math/special_functions/laguerre.hpp
    pub(crate) fn math_laguerre(n: c_uint, x: f64) -> f64;
    pub(crate) fn math_laguerre_assoc(n: c_uint, m: c_uint, x: f64) -> f64;

    // boost/math/special_functions/lambert_w.hpp
    pub(crate) fn math_lambert_w0(x: f64) -> f64;
    pub(crate) fn math_lambert_w0_prime(x: f64) -> f64;
    pub(crate) fn math_lambert_wm1(x: f64) -> f64;
    pub(crate) fn math_lambert_wm1_prime(x: f64) -> f64;

    // boost/math/special_functions/legendre.hpp
    pub(crate) fn math_legendre_p(l: c_int, x: f64) -> f64;
    pub(crate) fn math_legendre_p_assoc(l: c_int, m: c_int, x: f64) -> f64;
    pub(crate) fn math_legendre_p_prime(l: c_int, x: f64) -> f64;
    pub(crate) fn math_legendre_p_zeros(l: c_int, out: *mut f64);
    pub(crate) fn math_legendre_q(l: c_uint, x: f64) -> f64;

    // boost/math/special_functions/log1p.hpp
    pub(crate) fn math_log1p(x: f64) -> f64;

    // boost/math/special_functions/logsumexp.hpp
    pub(crate) fn math_logaddexp(x1: f64, x2: f64) -> f64;
    pub(crate) fn math_logsumexp(arr: *const f64, len: usize) -> f64;

    // boost/math/special_functions/owens_t.hpp
    pub(crate) fn math_owens_t(h: f64, a: f64) -> f64;

    // boost/math/special_functions/polygamma.hpp
    pub(crate) fn math_polygamma(n: c_int, x: f64) -> f64;

    // boost/math/special_functions/powm1.hpp
    pub(crate) fn math_powm1(x: f64, y: f64) -> f64;

    // boost/math/special_functions/prime.hpp
    pub(crate) fn math_prime(n: c_uint) -> u32;

    // boost/math/special_functions/rsqrt.hpp
    pub(crate) fn math_rsqrt(x: f64) -> f64;

    // boost/math/special_functions/sin_pi.hpp
    pub(crate) fn math_sin_pi(x: f64) -> f64;

    // boost/math/special_functions/sinc.hpp
    pub(crate) fn math_sinc_pi(x: f64) -> f64;

    // boost/math/special_functions/sinhc.hpp
    pub(crate) fn math_sinhc_pi(x: f64) -> f64;

    // boost/math/special_functions/spherical_harmonic.hpp
    #[cfg(feature = "num-complex")]
    pub(crate) fn math_spherical_harmonic(
        n: c_uint,
        m: c_int,
        theta: f64,
        phi: f64,
        out_re: *mut f64,
        out_im: *mut f64,
    );
    pub(crate) fn math_spherical_harmonic_r(n: c_uint, m: c_int, theta: f64, phi: f64) -> f64;
    pub(crate) fn math_spherical_harmonic_i(n: c_uint, m: c_int, theta: f64, phi: f64) -> f64;

    // boost/math/special_functions/sqrt1pm1.hpp
    pub(crate) fn math_sqrt1pm1(x: f64) -> f64;

    // boost/math/special_functions/trigamma.hpp
    pub(crate) fn math_trigamma(x: f64) -> f64;

    // boost/math/special_functions/zeta.hpp
    pub(crate) fn math_zeta(s: f64) -> f64;
}

#[cfg(feature = "num-complex")]
unsafe extern "C-unwind" {
    // boost/math/special_functions/hankel.hpp
    pub(crate) fn math_cyl_hankel_1(nu: f64, x: f64, out_re: *mut f64, out_im: *mut f64);
    pub(crate) fn math_cyl_hankel_2(nu: f64, x: f64, out_re: *mut f64, out_im: *mut f64);
    pub(crate) fn math_sph_hankel_1(nu: f64, x: f64, out_re: *mut f64, out_im: *mut f64);
    pub(crate) fn math_sph_hankel_2(nu: f64, x: f64, out_re: *mut f64, out_im: *mut f64);
}
