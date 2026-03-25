//! Raw FFI declarations for wrapper.cpp

use core::ffi::{c_int, c_uint};

// ccmath
unsafe extern "C-unwind" {
    // boost/math/ccmath/sqrt.hpp
    pub(crate) fn math_ccmath_sqrt(x: f64) -> f64;
}

// special_functions
unsafe extern "C-unwind" {
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

// distributions
unsafe extern "C-unwind" {
    // 1-parameter distributions

    // boost/math/distributions/bernoulli.hpp
    pub(crate) fn math_dist_bernoulli_pdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_cdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_cdf_c(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_quantile(p1: f64, p: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_quantile_c(p1: f64, q: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_mean(p1: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_variance(p1: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_std_dev(p1: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_skewness(p1: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_kurtosis(p1: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_kurtosis_excess(p1: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_median(p1: f64) -> f64;
    pub(crate) fn math_dist_bernoulli_mode(p1: f64) -> f64;

    // boost/math/distributions/chi_squared.hpp
    pub(crate) fn math_dist_chi_squared_pdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_cdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_cdf_c(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_quantile(p1: f64, p: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_quantile_c(p1: f64, q: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_mean(p1: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_variance(p1: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_std_dev(p1: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_skewness(p1: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_kurtosis(p1: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_kurtosis_excess(p1: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_median(p1: f64) -> f64;
    pub(crate) fn math_dist_chi_squared_mode(p1: f64) -> f64;

    // boost/math/distributions/exponential.hpp
    pub(crate) fn math_dist_exponential_pdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_exponential_cdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_exponential_cdf_c(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_exponential_quantile(p1: f64, p: f64) -> f64;
    pub(crate) fn math_dist_exponential_quantile_c(p1: f64, q: f64) -> f64;
    pub(crate) fn math_dist_exponential_mean(p1: f64) -> f64;
    pub(crate) fn math_dist_exponential_variance(p1: f64) -> f64;
    pub(crate) fn math_dist_exponential_std_dev(p1: f64) -> f64;
    pub(crate) fn math_dist_exponential_skewness(p1: f64) -> f64;
    pub(crate) fn math_dist_exponential_kurtosis(p1: f64) -> f64;
    pub(crate) fn math_dist_exponential_kurtosis_excess(p1: f64) -> f64;
    pub(crate) fn math_dist_exponential_median(p1: f64) -> f64;
    pub(crate) fn math_dist_exponential_mode(p1: f64) -> f64;

    // boost/math/distributions/geometric.hpp
    pub(crate) fn math_dist_geometric_pdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_geometric_cdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_geometric_cdf_c(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_geometric_quantile(p1: f64, p: f64) -> f64;
    pub(crate) fn math_dist_geometric_quantile_c(p1: f64, q: f64) -> f64;
    pub(crate) fn math_dist_geometric_mean(p1: f64) -> f64;
    pub(crate) fn math_dist_geometric_variance(p1: f64) -> f64;
    pub(crate) fn math_dist_geometric_std_dev(p1: f64) -> f64;
    pub(crate) fn math_dist_geometric_skewness(p1: f64) -> f64;
    pub(crate) fn math_dist_geometric_kurtosis(p1: f64) -> f64;
    pub(crate) fn math_dist_geometric_kurtosis_excess(p1: f64) -> f64;
    pub(crate) fn math_dist_geometric_median(p1: f64) -> f64;
    pub(crate) fn math_dist_geometric_mode(p1: f64) -> f64;

    // boost/math/distributions/inverse_chi_squared.hpp
    pub(crate) fn math_dist_inverse_chi_squared_pdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_cdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_cdf_c(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_quantile(p1: f64, p: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_quantile_c(p1: f64, q: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_mean(p1: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_variance(p1: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_std_dev(p1: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_skewness(p1: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_kurtosis(p1: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_kurtosis_excess(p1: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_median(p1: f64) -> f64;
    pub(crate) fn math_dist_inverse_chi_squared_mode(p1: f64) -> f64;

    // boost/math/distributions/kolmogorov_smirnov.hpp
    pub(crate) fn math_dist_kolmogorov_smirnov_pdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_cdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_cdf_c(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_quantile(p1: f64, p: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_quantile_c(p1: f64, q: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_mean(p1: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_variance(p1: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_std_dev(p1: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_skewness(p1: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_kurtosis(p1: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_kurtosis_excess(p1: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_median(p1: f64) -> f64;
    pub(crate) fn math_dist_kolmogorov_smirnov_mode(p1: f64) -> f64;

    // boost/math/distributions/poisson.hpp
    pub(crate) fn math_dist_poisson_pdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_poisson_cdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_poisson_cdf_c(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_poisson_quantile(p1: f64, p: f64) -> f64;
    pub(crate) fn math_dist_poisson_quantile_c(p1: f64, q: f64) -> f64;
    pub(crate) fn math_dist_poisson_mean(p1: f64) -> f64;
    pub(crate) fn math_dist_poisson_variance(p1: f64) -> f64;
    pub(crate) fn math_dist_poisson_std_dev(p1: f64) -> f64;
    pub(crate) fn math_dist_poisson_skewness(p1: f64) -> f64;
    pub(crate) fn math_dist_poisson_kurtosis(p1: f64) -> f64;
    pub(crate) fn math_dist_poisson_kurtosis_excess(p1: f64) -> f64;
    pub(crate) fn math_dist_poisson_median(p1: f64) -> f64;
    pub(crate) fn math_dist_poisson_mode(p1: f64) -> f64;

    // boost/math/distributions/rayleigh.hpp
    pub(crate) fn math_dist_rayleigh_pdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_cdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_cdf_c(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_quantile(p1: f64, p: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_quantile_c(p1: f64, q: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_mean(p1: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_variance(p1: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_std_dev(p1: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_skewness(p1: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_kurtosis(p1: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_kurtosis_excess(p1: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_median(p1: f64) -> f64;
    pub(crate) fn math_dist_rayleigh_mode(p1: f64) -> f64;

    // boost/math/distributions/students_t.hpp
    pub(crate) fn math_dist_students_t_pdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_students_t_cdf(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_students_t_cdf_c(p1: f64, x: f64) -> f64;
    pub(crate) fn math_dist_students_t_quantile(p1: f64, p: f64) -> f64;
    pub(crate) fn math_dist_students_t_quantile_c(p1: f64, q: f64) -> f64;
    pub(crate) fn math_dist_students_t_mean(p1: f64) -> f64;
    pub(crate) fn math_dist_students_t_variance(p1: f64) -> f64;
    pub(crate) fn math_dist_students_t_std_dev(p1: f64) -> f64;
    pub(crate) fn math_dist_students_t_skewness(p1: f64) -> f64;
    pub(crate) fn math_dist_students_t_kurtosis(p1: f64) -> f64;
    pub(crate) fn math_dist_students_t_kurtosis_excess(p1: f64) -> f64;
    pub(crate) fn math_dist_students_t_median(p1: f64) -> f64;
    pub(crate) fn math_dist_students_t_mode(p1: f64) -> f64;

    // 2-parameter distributions

    // boost/math/distributions/arcsine.hpp
    pub(crate) fn math_dist_arcsine_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_arcsine_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_arcsine_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_arcsine_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_arcsine_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_arcsine_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_arcsine_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_arcsine_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_arcsine_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_arcsine_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_arcsine_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_arcsine_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_arcsine_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/beta.hpp
    pub(crate) fn math_dist_beta_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_beta_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_beta_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_beta_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_beta_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_beta_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_beta_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_beta_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_beta_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_beta_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_beta_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_beta_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_beta_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/binomial.hpp
    pub(crate) fn math_dist_binomial_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_binomial_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_binomial_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_binomial_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_binomial_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_binomial_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_binomial_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_binomial_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_binomial_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_binomial_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_binomial_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_binomial_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_binomial_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/cauchy.hpp
    pub(crate) fn math_dist_cauchy_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_cauchy_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_cauchy_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_cauchy_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_cauchy_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_cauchy_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_cauchy_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_cauchy_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_cauchy_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_cauchy_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_cauchy_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_cauchy_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_cauchy_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/extreme_value.hpp
    pub(crate) fn math_dist_extreme_value_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_extreme_value_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/fisher_f.hpp
    pub(crate) fn math_dist_fisher_f_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_fisher_f_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/gamma.hpp
    pub(crate) fn math_dist_gamma_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_gamma_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_gamma_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_gamma_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_gamma_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_gamma_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_gamma_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_gamma_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_gamma_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_gamma_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_gamma_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_gamma_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_gamma_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/inverse_gamma.hpp
    pub(crate) fn math_dist_inverse_gamma_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gamma_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/inverse_gaussian.hpp
    pub(crate) fn math_dist_inverse_gaussian_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_inverse_gaussian_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/laplace.hpp
    pub(crate) fn math_dist_laplace_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_laplace_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_laplace_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_laplace_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_laplace_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_laplace_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_laplace_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_laplace_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_laplace_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_laplace_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_laplace_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_laplace_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_laplace_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/logistic.hpp
    pub(crate) fn math_dist_logistic_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_logistic_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_logistic_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_logistic_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_logistic_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_logistic_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_logistic_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_logistic_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_logistic_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_logistic_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_logistic_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_logistic_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_logistic_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/lognormal.hpp
    pub(crate) fn math_dist_lognormal_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_lognormal_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_lognormal_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_lognormal_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_lognormal_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_lognormal_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_lognormal_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_lognormal_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_lognormal_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_lognormal_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_lognormal_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_lognormal_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_lognormal_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/negative_binomial.hpp
    pub(crate) fn math_dist_negative_binomial_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_negative_binomial_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/non_central_chi_squared.hpp
    pub(crate) fn math_dist_non_central_chi_squared_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_chi_squared_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/non_central_t.hpp
    pub(crate) fn math_dist_non_central_t_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_non_central_t_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/normal.hpp
    pub(crate) fn math_dist_normal_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_normal_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_normal_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_normal_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_normal_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_normal_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_normal_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_normal_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_normal_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_normal_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_normal_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_normal_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_normal_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/pareto.hpp
    pub(crate) fn math_dist_pareto_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_pareto_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_pareto_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_pareto_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_pareto_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_pareto_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_pareto_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_pareto_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_pareto_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_pareto_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_pareto_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_pareto_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_pareto_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/uniform.hpp
    pub(crate) fn math_dist_uniform_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_uniform_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_uniform_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_uniform_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_uniform_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_uniform_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_uniform_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_uniform_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_uniform_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_uniform_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_uniform_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_uniform_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_uniform_mode(p1: f64, p2: f64) -> f64;

    // boost/math/distributions/weibull.hpp
    pub(crate) fn math_dist_weibull_pdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_weibull_cdf(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_weibull_cdf_c(p1: f64, p2: f64, x: f64) -> f64;
    pub(crate) fn math_dist_weibull_quantile(p1: f64, p2: f64, p: f64) -> f64;
    pub(crate) fn math_dist_weibull_quantile_c(p1: f64, p2: f64, q: f64) -> f64;
    pub(crate) fn math_dist_weibull_mean(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_weibull_variance(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_weibull_std_dev(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_weibull_skewness(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_weibull_kurtosis(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_weibull_kurtosis_excess(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_weibull_median(p1: f64, p2: f64) -> f64;
    pub(crate) fn math_dist_weibull_mode(p1: f64, p2: f64) -> f64;

    // 3-parameter distributions

    // boost/math/distributions/non_central_beta.hpp
    pub(crate) fn math_dist_non_central_beta_pdf(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_cdf(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_cdf_c(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_quantile(p1: f64, p2: f64, p3: f64, p: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_quantile_c(p1: f64, p2: f64, p3: f64, q: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_mean(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_variance(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_std_dev(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_skewness(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_kurtosis(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_kurtosis_excess(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_median(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_beta_mode(p1: f64, p2: f64, p3: f64) -> f64;

    // boost/math/distributions/non_central_f.hpp
    pub(crate) fn math_dist_non_central_f_pdf(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_cdf(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_cdf_c(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_quantile(p1: f64, p2: f64, p3: f64, p: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_quantile_c(p1: f64, p2: f64, p3: f64, q: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_mean(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_variance(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_std_dev(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_skewness(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_kurtosis(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_kurtosis_excess(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_median(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_non_central_f_mode(p1: f64, p2: f64, p3: f64) -> f64;

    // boost/math/distributions/skew_normal.hpp
    pub(crate) fn math_dist_skew_normal_pdf(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_cdf(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_cdf_c(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_quantile(p1: f64, p2: f64, p3: f64, p: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_quantile_c(p1: f64, p2: f64, p3: f64, q: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_mean(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_variance(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_std_dev(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_skewness(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_kurtosis(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_kurtosis_excess(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_median(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_skew_normal_mode(p1: f64, p2: f64, p3: f64) -> f64;

    // boost/math/distributions/triangular.hpp
    pub(crate) fn math_dist_triangular_pdf(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_triangular_cdf(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_triangular_cdf_c(p1: f64, p2: f64, p3: f64, x: f64) -> f64;
    pub(crate) fn math_dist_triangular_quantile(p1: f64, p2: f64, p3: f64, p: f64) -> f64;
    pub(crate) fn math_dist_triangular_quantile_c(p1: f64, p2: f64, p3: f64, q: f64) -> f64;
    pub(crate) fn math_dist_triangular_mean(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_triangular_variance(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_triangular_std_dev(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_triangular_skewness(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_triangular_kurtosis(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_triangular_kurtosis_excess(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_triangular_median(p1: f64, p2: f64, p3: f64) -> f64;
    pub(crate) fn math_dist_triangular_mode(p1: f64, p2: f64, p3: f64) -> f64;

    // boost/math/distributions/hypergeometric.hpp (unsigned parameters)
    pub(crate) fn math_dist_hypergeometric_pdf(
        r: c_uint,
        n: c_uint,
        upper_n: c_uint,
        x: f64,
    ) -> f64;
    pub(crate) fn math_dist_hypergeometric_cdf(
        r: c_uint,
        n: c_uint,
        upper_n: c_uint,
        x: f64,
    ) -> f64;
    pub(crate) fn math_dist_hypergeometric_quantile(
        r: c_uint,
        n: c_uint,
        upper_n: c_uint,
        p: f64,
    ) -> f64;
    pub(crate) fn math_dist_hypergeometric_cdf_c(
        r: c_uint,
        n: c_uint,
        upper_n: c_uint,
        x: f64,
    ) -> f64;
    pub(crate) fn math_dist_hypergeometric_quantile_c(
        r: c_uint,
        n: c_uint,
        upper_n: c_uint,
        q: f64,
    ) -> f64;
    pub(crate) fn math_dist_hypergeometric_mean(r: c_uint, n: c_uint, upper_n: c_uint) -> f64;
    pub(crate) fn math_dist_hypergeometric_variance(r: c_uint, n: c_uint, upper_n: c_uint) -> f64;
    pub(crate) fn math_dist_hypergeometric_std_dev(r: c_uint, n: c_uint, upper_n: c_uint) -> f64;
    pub(crate) fn math_dist_hypergeometric_skewness(r: c_uint, n: c_uint, upper_n: c_uint) -> f64;
    pub(crate) fn math_dist_hypergeometric_kurtosis(r: c_uint, n: c_uint, upper_n: c_uint) -> f64;
    pub(crate) fn math_dist_hypergeometric_kurtosis_excess(
        r: c_uint,
        n: c_uint,
        upper_n: c_uint,
    ) -> f64;
    pub(crate) fn math_dist_hypergeometric_median(r: c_uint, n: c_uint, upper_n: c_uint) -> f64;
    pub(crate) fn math_dist_hypergeometric_mode(r: c_uint, n: c_uint, upper_n: c_uint) -> f64;
}
