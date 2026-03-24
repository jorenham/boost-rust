// Simple C++ wrappers for the Boost Math functions for https://github.com/jorenham/boost-rust

#ifndef BOOST_MATH_TOOLS_USER_HPP
    // https://www.boost.org/doc/libs/latest/boost/math/tools/user.hpp
    #define BOOST_MATH_TOOLS_USER_HPP
    #define BOOST_MATH_NO_LONG_DOUBLE_MATH_FUNCTIONS
    #define BOOST_MATH_DOMAIN_ERROR_POLICY ignore_error
    #define BOOST_MATH_POLE_ERROR_POLICY ignore_error
    #define BOOST_MATH_EVALUATION_ERROR_POLICY ignore_error
    #define BOOST_MATH_OVERFLOW_ERROR_POLICY ignore_error
    #define BOOST_MATH_UNDERFLOW_ERROR_POLICY ignore_error
    #define BOOST_MATH_DENORM_ERROR_POLICY ignore_error
    // #define BOOST_MATH_PROMOTE_FLOAT_POLICY false
    #define BOOST_MATH_PROMOTE_DOUBLE_POLICY false
    #define BOOST_MATH_ASSERT_UNDEFINED_POLICY true
    #define BOOST_MATH_MAX_ROOT_ITERATION_POLICY 400
    #define BOOST_MATH_DISABLE_FLOAT128
#endif

#include <boost/math/ccmath/sqrt.hpp>
#include <boost/math/special_functions/acosh.hpp>
#include <boost/math/special_functions/airy.hpp>
#include <boost/math/special_functions/asinh.hpp>
#include <boost/math/special_functions/atanh.hpp>
#include <boost/math/special_functions/bessel.hpp>
#include <boost/math/special_functions/bessel_prime.hpp>
#include <boost/math/special_functions/beta.hpp>
#include <boost/math/special_functions/binomial.hpp>
#include <boost/math/special_functions/cardinal_b_spline.hpp>
#include <boost/math/special_functions/cbrt.hpp>
#include <boost/math/special_functions/chebyshev.hpp>
#include <boost/math/special_functions/cos_pi.hpp>
#include <boost/math/special_functions/ellint_1.hpp>
#include <boost/math/special_functions/ellint_2.hpp>
#include <boost/math/special_functions/ellint_3.hpp>
#include <boost/math/special_functions/ellint_d.hpp>
#include <boost/math/special_functions/ellint_rc.hpp>
#include <boost/math/special_functions/ellint_rd.hpp>
#include <boost/math/special_functions/ellint_rf.hpp>
#include <boost/math/special_functions/ellint_rg.hpp>
#include <boost/math/special_functions/ellint_rj.hpp>
#include <boost/math/special_functions/erf.hpp>
#include <boost/math/special_functions/expint.hpp>
#include <boost/math/special_functions/expm1.hpp>
#include <boost/math/special_functions/factorials.hpp>
#include <boost/math/special_functions/gamma.hpp>
#include <boost/math/special_functions/gegenbauer.hpp>
#include <boost/math/special_functions/hankel.hpp>
#include <boost/math/special_functions/hermite.hpp>
#include <boost/math/special_functions/heuman_lambda.hpp>
#include <boost/math/special_functions/hypergeometric_0F1.hpp>
#include <boost/math/special_functions/hypergeometric_1F0.hpp>
#include <boost/math/special_functions/hypergeometric_1F1.hpp>
#include <boost/math/special_functions/hypergeometric_2F0.hpp>
#include <boost/math/special_functions/hypot.hpp>
#include <boost/math/special_functions/jacobi.hpp>
#include <boost/math/special_functions/jacobi_elliptic.hpp>
#include <boost/math/special_functions/jacobi_theta.hpp>
#include <boost/math/special_functions/jacobi_zeta.hpp>
#include <boost/math/special_functions/laguerre.hpp>
#include <boost/math/special_functions/lambert_w.hpp>
#include <boost/math/special_functions/legendre.hpp>
#include <boost/math/special_functions/log1p.hpp>
#include <boost/math/special_functions/logsumexp.hpp>
#include <boost/math/special_functions/owens_t.hpp>
#include <boost/math/special_functions/polygamma.hpp>
#include <boost/math/special_functions/powm1.hpp>
#include <boost/math/special_functions/prime.hpp>
#include <boost/math/special_functions/rsqrt.hpp>
#include <boost/math/special_functions/sin_pi.hpp>
#include <boost/math/special_functions/sinc.hpp>
#include <boost/math/special_functions/sinhc.hpp>
#include <boost/math/special_functions/spherical_harmonic.hpp>
#include <boost/math/special_functions/sqrt1pm1.hpp>
#include <boost/math/special_functions/zeta.hpp>
#include <boost/math/tools/complex.hpp>

namespace detail {

inline double polygamma(const int n, double x) noexcept {
    // workaround for incorrect boost::math::polygamma values for infinities and NaNs
    if (std::isinf(x)) {
        if (x < 0) {
            // polygamma does not converge in the limit for negative infinity
            return std::numeric_limits<double>::quiet_NaN();
        } else if (n == 0) {
            // polygamma(0, ∞) = ∞ in the limit
            return std::numeric_limits<double>::infinity();
        } else {
            // polygamma(n, ∞) = 0 in the limit for n > 0
            if (n % 2) {
                return 0.0; // odd
            } else {
                return -0.0; // even
            }
        }
    } else if (x <= 0 && floor(x) == x) {
        if (n % 2) {
            // polygamma(n, -m) = +∞ for `n` odd and `m` a non-negative integer
            return std::numeric_limits<double>::infinity();
        } else {
            // polygamma(n, -m) is singular for `n` even and `m` a non-negative integer
            return std::numeric_limits<double>::quiet_NaN();
        }
    }

    // workaround for n>=2 throwing exceptions for NaN inputs
    if (std::isnan(x)) {
        return std::numeric_limits<double>::quiet_NaN();
    }

    if (n == -1) {
        // special case: polygamma(-1, x) = log(gamma(x))
        return boost::math::lgamma(x);
    } else if (n < -1) {
        // analytical continuation not implemented by boost::math::polygamma
        return std::numeric_limits<double>::quiet_NaN();
    }

    return boost::math::polygamma(n, x);
}

} // namespace detail

using namespace boost::math;
using cdouble = boost::math::complex<double>;

// ccmath
extern "C" {
// boost/math/ccmath/sqrt.hpp
double math_ccmath_sqrt(double x) { return ccmath::sqrt(x); }
}

// special_functions
extern "C" {
// boost/math/special_functions/acosh.hpp
double math_acosh(double x) { return acosh(x); }
// boost/math/special_functions/asinh.hpp
double math_asinh(double x) { return asinh(x); }
// boost/math/special_functions/atanh.hpp
double math_atanh(double x) { return atanh(x); }

// boost/math/special_functions/airy.hpp
double math_airy_ai(double x) { return airy_ai(x); }
double math_airy_ai_prime(double x) { return airy_ai_prime(x); }
double math_airy_ai_zero(int m) { return airy_ai_zero<double>(m); }
double math_airy_bi(double x) { return airy_bi(x); }
double math_airy_bi_prime(double x) { return airy_bi_prime(x); }
double math_airy_bi_zero(int m) { return airy_bi_zero<double>(m); }

// boost/math/special_functions/bessel.hpp
double math_cyl_bessel_j(double nu, double x) { return cyl_bessel_j(nu, x); }
double math_cyl_neumann(double nu, double x) { return cyl_neumann(nu, x); }
double math_cyl_bessel_i(double nu, double x) { return cyl_bessel_i(nu, x); }
double math_cyl_bessel_k(double nu, double x) { return cyl_bessel_k(nu, x); }
double math_sph_bessel(unsigned n, double x) { return sph_bessel(n, x); }
double math_sph_neumann(unsigned n, double x) { return sph_neumann(n, x); }
double math_cyl_bessel_j_zero(double nu, int k) { return cyl_bessel_j_zero(nu, k); }
double math_cyl_neumann_zero(double nu, int k) { return cyl_neumann_zero(nu, k); }

// boost/math/special_functions/bessel_prime.hpp
double math_cyl_bessel_j_prime(double nu, double x) { return cyl_bessel_j_prime(nu, x); }
double math_cyl_neumann_prime(double nu, double x) { return cyl_neumann_prime(nu, x); }
double math_cyl_bessel_i_prime(double nu, double x) { return cyl_bessel_i_prime(nu, x); }
double math_cyl_bessel_k_prime(double nu, double x) { return cyl_bessel_k_prime(nu, x); }
double math_sph_bessel_prime(unsigned n, double x) { return sph_bessel_prime(n, x); }
double math_sph_neumann_prime(unsigned n, double x) { return sph_neumann_prime(n, x); }

// boost/math/special_functions/beta.hpp
double math_beta(double a, double b) { return beta(a, b); }
double math_beta_(double a, double b, double x) { return beta(a, b, x); }
double math_betac(double a, double b, double x) { return betac(a, b, x); }
double math_ibeta(double a, double b, double x) { return ibeta(a, b, x); }
double math_ibeta_derivative(double a, double b, double x) { return ibeta_derivative(a, b, x); }
double math_ibeta_inv(double a, double b, double p) { return ibeta_inv(a, b, p); }
double math_ibeta_inva(double b, double x, double p) { return ibeta_inva(b, x, p); }
double math_ibeta_invb(double a, double x, double p) { return ibeta_invb(a, x, p); }
double math_ibetac(double a, double b, double x) { return ibetac(a, b, x); }
double math_ibetac_inv(double a, double b, double q) { return ibetac_inv(a, b, q); }
double math_ibetac_inva(double b, double x, double q) { return ibetac_inva(b, x, q); }
double math_ibetac_invb(double a, double x, double q) { return ibetac_invb(a, x, q); }

// boost/math/special_functions/binomial.hpp
double math_binomial_coefficient(unsigned n, unsigned k) {
    return binomial_coefficient<double>(n, k);
}

// boost/math/special_functions/cbrt.hpp
double math_cbrt(double x) { return boost::math::cbrt(x); }

// boost/math/special_functions/chebyshev.hpp
double math_chebyshev_t(unsigned n, double x) { return chebyshev_t(n, x); }
double math_chebyshev_t_prime(unsigned n, double x) { return chebyshev_t_prime(n, x); }
double math_chebyshev_u(unsigned n, double x) { return chebyshev_u(n, x); }

// boost/math/special_functions/cos_pi.hpp
double math_cos_pi(double x) { return boost::math::cos_pi(x); }

// boost/math/special_functions/ellint_1.hpp
double math_ellint_1(double k) { return ellint_1(k); }
double math_ellint_1_inc(double k, double phi) { return ellint_1(k, phi); }
// boost/math/special_functions/ellint_2.hpp
double math_ellint_2(double k) { return ellint_2(k); }
double math_ellint_2_inc(double k, double phi) { return ellint_2(k, phi); }
// boost/math/special_functions/ellint_3.hpp
double math_ellint_3(double k, double v) { return ellint_3(k, v); }
double math_ellint_3_inc(double k, double v, double phi) { return ellint_3(k, v, phi); }
// boost/math/special_functions/ellint_d.hpp
double math_ellint_d(double k) { return ellint_d(k); }
double math_ellint_d_inc(double k, double phi) { return ellint_d(k, phi); }
// boost/math/special_functions/ellint_rc.hpp
double math_ellint_rc(double x, double y) { return ellint_rc(x, y); }
// boost/math/special_functions/ellint_rd.hpp
double math_ellint_rd(double x, double y, double z) { return ellint_rd(x, y, z); }
// boost/math/special_functions/ellint_rf.hpp
double math_ellint_rf(double x, double y, double z) { return ellint_rf(x, y, z); }
// boost/math/special_functions/ellint_rg.hpp
double math_ellint_rg(double x, double y, double z) { return ellint_rg(x, y, z); }
// boost/math/special_functions/ellint_rj.hpp
double math_ellint_rj(double x, double y, double z, double p) { return ellint_rj(x, y, z, p); }

// boost/math/special_functions/erf.hpp
double math_erf(double x) { return boost::math::erf(x); }
double math_erf_inv(double p) { return erf_inv(p); }
double math_erfc(double x) { return boost::math::erfc(x); }
double math_erfc_inv(double q) { return erfc_inv(q); }

// boost/math/special_functions/expint.hpp
double math_expint_i(double x) { return expint(x); }
double math_expint_n(unsigned n, double x) { return expint(n, x); }

// boost/math/special_functions/expm1.hpp
double math_expm1(double x) { return boost::math::expm1(x); }

// boost/math/special_functions/factorials.hpp
double math_factorial(unsigned i) { return factorial<double>(i); }
double math_double_factorial(unsigned i) { return double_factorial<double>(i); }
double math_falling_factorial(double x, unsigned n) { return falling_factorial(x, n); }
double math_rising_factorial(double x, int n) { return rising_factorial(x, n); }

// boost/math/special_functions/gamma.hpp
double math_tgamma(double x) { return boost::math::tgamma(x); }
double math_tgamma_(double a, double x) { return boost::math::tgamma(a, x); }
double math_tgamma1pm1(double x) { return tgamma1pm1(x); }
double math_tgamma_lower(double a, double x) { return tgamma_lower(a, x); }
double math_tgamma_ratio(double a, double b) { return tgamma_ratio(a, b); }
double math_tgamma_delta_ratio(double x, double delta) { return tgamma_delta_ratio(x, delta); }
double math_lgamma(double x, int* sign) { return lgamma(x, sign); }
double math_gamma_q(double a, double x) { return gamma_q(a, x); }
double math_gamma_q_inv(double a, double q) { return gamma_q_inv(a, q); }
double math_gamma_q_inva(double x, double q) { return gamma_q_inva(x, q); }
double math_gamma_p(double a, double x) { return gamma_p(a, x); }
double math_gamma_p_inv(double a, double p) { return gamma_p_inv(a, p); }
double math_gamma_p_inva(double x, double p) { return gamma_p_inva(x, p); }
double math_gamma_p_derivative(double a, double x) { return gamma_p_derivative(a, x); }

// boost/math/special_functions/gegenbauer.hpp
double math_gegenbauer(unsigned n, double lambda, double x) { return gegenbauer(n, lambda, x); }
double math_gegenbauer_derivative(unsigned n, double lambda, double x, unsigned k) {
    return gegenbauer_derivative(n, lambda, x, k);
}

// boost/math/special_functions/hankel.hpp
void math_cyl_hankel_1(double nu, double x, double* out_re, double* out_im) {
    cdouble out = cyl_hankel_1(nu, x);
    *out_re = out.real();
    *out_im = out.imag();
}
void math_cyl_hankel_2(double nu, double x, double* out_re, double* out_im) {
    cdouble out = cyl_hankel_2(nu, x);
    *out_re = out.real();
    *out_im = out.imag();
}
void math_sph_hankel_1(double nu, double x, double* out_re, double* out_im) {
    cdouble out = sph_hankel_1(nu, x);
    *out_re = out.real();
    *out_im = out.imag();
}
void math_sph_hankel_2(double nu, double x, double* out_re, double* out_im) {
    cdouble out = sph_hankel_2(nu, x);
    *out_re = out.real();
    *out_im = out.imag();
}

// boost/math/special_functions/hermite.hpp
double math_hermite(unsigned n, double x) { return hermite(n, x); }

// boost/math/special_functions/heuman_lambda.hpp
double math_heuman_lambda(double k, double phi) { return heuman_lambda(k, phi); }

// boost/math/special_functions/hypergeometric_0F1.hpp
double math_hypergeometric_0F1(double b, double x) { return hypergeometric_0F1(b, x); }

// boost/math/special_functions/hypergeometric_1F0.hpp
double math_hypergeometric_1F0(double a, double x) { return hypergeometric_1F0(a, x); }

// boost/math/special_functions/hypergeometric_1F1.hpp
double math_hypergeometric_1F1(double a, double b, double x) { return hypergeometric_1F1(a, b, x); }
double math_hypergeometric_1F1_regularized(double a, double b, double x) {
    return hypergeometric_1F1_regularized(a, b, x);
}
double math_log_hypergeometric_1F1(double a, double b, double x, int* sign) {
    return log_hypergeometric_1F1(a, b, x, sign);
}

// boost/math/special_functions/hypergeometric_2F0.hpp
double math_hypergeometric_2F0(double a1, double a2, double x) {
    return hypergeometric_2F0(a1, a2, x);
}

// boost/math/special_functions/hypot.hpp
double math_hypot(double x, double y) { return boost::math::hypot(x, y); }

// boost/math/special_functions/jacobi.hpp
double math_jacobi(unsigned n, double alpha, double beta, double x) {
    return jacobi(n, alpha, beta, x);
}
double math_jacobi_derivative(unsigned n, double alpha, double beta, double x, unsigned k) {
    return jacobi_derivative(n, alpha, beta, x, k);
}

// boost/math/special_functions/jacobi_elliptic.hpp
double math_jacobi_elliptic(double k, double theta, double* pcn, double* pdn) {
    return jacobi_elliptic(k, theta, pcn, pdn);
}
double math_jacobi_cd(double k, double theta) { return jacobi_cd(k, theta); }
double math_jacobi_cn(double k, double theta) { return jacobi_cn(k, theta); }
double math_jacobi_cs(double k, double theta) { return jacobi_cs(k, theta); }
double math_jacobi_dc(double k, double theta) { return jacobi_dc(k, theta); }
double math_jacobi_dn(double k, double theta) { return jacobi_dn(k, theta); }
double math_jacobi_ds(double k, double theta) { return jacobi_ds(k, theta); }
double math_jacobi_nc(double k, double theta) { return jacobi_nc(k, theta); }
double math_jacobi_nd(double k, double theta) { return jacobi_nd(k, theta); }
double math_jacobi_ns(double k, double theta) { return jacobi_ns(k, theta); }
double math_jacobi_sc(double k, double theta) { return jacobi_sc(k, theta); }
double math_jacobi_sd(double k, double theta) { return jacobi_sd(k, theta); }
double math_jacobi_sn(double k, double theta) { return jacobi_sn(k, theta); }

// boost/math/special_functions/jacobi_theta.hpp
double math_jacobi_theta1(double x, double q) { return jacobi_theta1(x, q); }
double math_jacobi_theta1tau(double x, double tau) { return jacobi_theta1tau(x, tau); }
double math_jacobi_theta2(double x, double q) { return jacobi_theta2(x, q); }
double math_jacobi_theta2tau(double x, double tau) { return jacobi_theta2tau(x, tau); }
double math_jacobi_theta3(double x, double q) { return jacobi_theta3(x, q); }
double math_jacobi_theta3tau(double x, double tau) { return jacobi_theta3tau(x, tau); }
double math_jacobi_theta3m1(double x, double q) { return jacobi_theta3m1(x, q); }
double math_jacobi_theta3m1tau(double x, double tau) { return jacobi_theta3m1tau(x, tau); }
double math_jacobi_theta4(double x, double q) { return jacobi_theta4(x, q); }
double math_jacobi_theta4m1(double x, double q) { return jacobi_theta4m1(x, q); }
double math_jacobi_theta4tau(double x, double tau) { return jacobi_theta4tau(x, tau); }
double math_jacobi_theta4m1tau(double x, double tau) { return jacobi_theta4m1tau(x, tau); }

// boost/math/special_functions/jacobi_zeta.hpp
double math_jacobi_zeta(double k, double phi) { return jacobi_zeta(k, phi); }

// boost/math/special_functions/laguerre.hpp
double math_laguerre(unsigned n, double x) { return laguerre(n, x); }
double math_laguerre_assoc(unsigned n, unsigned m, double x) { return laguerre(n, m, x); }

// boost/math/special_functions/lambert_w.hpp
double math_lambert_w0(double x) { return lambert_w0(x); }
double math_lambert_w0_prime(double x) { return lambert_w0_prime(x); }
double math_lambert_wm1(double x) { return lambert_wm1(x); }
double math_lambert_wm1_prime(double x) { return lambert_wm1_prime(x); }

// boost/math/special_functions/legendre.hpp
double math_legendre_p(int l, double x) { return legendre_p(l, x); }
double math_legendre_p_assoc(int l, int m, double x) { return legendre_p(l, m, x); }
double math_legendre_p_prime(int l, double x) { return legendre_p_prime(l, x); }
void math_legendre_p_zeros(int l, double out[]) {
    // `out` must be of size `l.div_ceil(2)`
    auto vec = legendre_p_zeros<double>(l);
    for (size_t i = 0; i < vec.size(); i++) {
        out[i] = vec[i];
    }
}
double math_legendre_q(unsigned l, double x) { return legendre_q(l, x); }

// boost/math/special_functions/log1p.hpp
double math_log1p(double x) { return boost::math::log1p(x); }

// boost/math/special_functions/logsumexp.hpp
double math_logaddexp(double x1, double x2) { return logaddexp(x1, x2); }
double math_logsumexp(const double args[], size_t len) { return logsumexp(args, args + len); }

// boost/math/special_functions/owens_t.hpp
double math_owens_t(double h, double a) { return owens_t(h, a); }

// boost/math/special_functions/polygamma.hpp
double math_digamma(double x) { return ::detail::polygamma(0, x); }
double math_trigamma(double x) { return ::detail::polygamma(1, x); }
double math_polygamma(const int n, double x) { return ::detail::polygamma(n, x); }

// boost/math/special_functions/powm1.hpp
double math_powm1(double x, double y) { return powm1(x, y); }

// boost/math/special_functions/prime.hpp
std::uint32_t math_prime(unsigned n) { return prime(n); }

// boost/math/special_functions/rsqrt.hpp
double math_rsqrt(double x) { return rsqrt(x); }

// boost/math/special_functions/sin_pi.hpp
double math_sin_pi(double x) { return sin_pi(x); }

// boost/math/special_functions/sinc.hpp
double math_sinc_pi(double x) { return sinc_pi(x); }

// boost/math/special_functions/sinhc.hpp
double math_sinhc_pi(double x) { return sinhc_pi(x); }

// boost/math/special_functions/spherical_harmonic.hpp
void math_spherical_harmonic(
    unsigned n,
    int m,
    double theta,
    double phi,
    double* out_re,
    double* out_im
) {
    cdouble out = spherical_harmonic(n, m, theta, phi);
    *out_re = out.real();
    *out_im = out.imag();
}
double math_spherical_harmonic_r(unsigned n, int m, double theta, double phi) {
    return spherical_harmonic_r(n, m, theta, phi);
}
double math_spherical_harmonic_i(unsigned n, int m, double theta, double phi) {
    return spherical_harmonic_i(n, m, theta, phi);
}

// boost/math/special_functions/sqrt1pm1.hpp
double math_sqrt1pm1(double x) { return sqrt1pm1(x); }

// boost/math/special_functions/zeta.hpp
double math_zeta(double s) { return zeta(s); }

} // extern "C"

// distributions
#include <boost/math/distributions/arcsine.hpp>
#include <boost/math/distributions/bernoulli.hpp>
#include <boost/math/distributions/beta.hpp>
#include <boost/math/distributions/binomial.hpp>
#include <boost/math/distributions/cauchy.hpp>
#include <boost/math/distributions/chi_squared.hpp>
#include <boost/math/distributions/exponential.hpp>
#include <boost/math/distributions/extreme_value.hpp>
#include <boost/math/distributions/fisher_f.hpp>
#include <boost/math/distributions/gamma.hpp>
#include <boost/math/distributions/geometric.hpp>
#include <boost/math/distributions/hypergeometric.hpp>
#include <boost/math/distributions/inverse_chi_squared.hpp>
#include <boost/math/distributions/inverse_gamma.hpp>
#include <boost/math/distributions/inverse_gaussian.hpp>
#include <boost/math/distributions/kolmogorov_smirnov.hpp>
#include <boost/math/distributions/laplace.hpp>
#include <boost/math/distributions/logistic.hpp>
#include <boost/math/distributions/lognormal.hpp>
#include <boost/math/distributions/negative_binomial.hpp>
#include <boost/math/distributions/non_central_beta.hpp>
#include <boost/math/distributions/non_central_chi_squared.hpp>
#include <boost/math/distributions/non_central_f.hpp>
#include <boost/math/distributions/non_central_t.hpp>
#include <boost/math/distributions/normal.hpp>
#include <boost/math/distributions/pareto.hpp>
#include <boost/math/distributions/poisson.hpp>
#include <boost/math/distributions/rayleigh.hpp>
#include <boost/math/distributions/skew_normal.hpp>
#include <boost/math/distributions/students_t.hpp>
#include <boost/math/distributions/triangular.hpp>
#include <boost/math/distributions/uniform.hpp>
#include <boost/math/distributions/weibull.hpp>

extern "C" {

// Macro for distributions with 1 parameter
#define DIST_FNS_1(name, DistType) \
    double math_dist_##name##_pdf(double p1, double x) { return pdf(DistType<double>(p1), x); } \
    double math_dist_##name##_cdf(double p1, double x) { return cdf(DistType<double>(p1), x); } \
    double math_dist_##name##_quantile(double p1, double p) { \
        return quantile(DistType<double>(p1), p); \
    } \
    double math_dist_##name##_mean(double p1) { return mean(DistType<double>(p1)); } \
    double math_dist_##name##_variance(double p1) { return variance(DistType<double>(p1)); } \
    double math_dist_##name##_std_dev(double p1) { \
        return standard_deviation(DistType<double>(p1)); \
    } \
    double math_dist_##name##_skewness(double p1) { return skewness(DistType<double>(p1)); } \
    double math_dist_##name##_kurtosis(double p1) { return kurtosis(DistType<double>(p1)); } \
    double math_dist_##name##_kurtosis_excess(double p1) { \
        return kurtosis_excess(DistType<double>(p1)); \
    } \
    double math_dist_##name##_median(double p1) { return median(DistType<double>(p1)); } \
    double math_dist_##name##_mode(double p1) { return mode(DistType<double>(p1)); }

// Macro for distributions with 2 parameters
#define DIST_FNS_2(name, DistType) \
    double math_dist_##name##_pdf(double p1, double p2, double x) { \
        return pdf(DistType<double>(p1, p2), x); \
    } \
    double math_dist_##name##_cdf(double p1, double p2, double x) { \
        return cdf(DistType<double>(p1, p2), x); \
    } \
    double math_dist_##name##_quantile(double p1, double p2, double p) { \
        return quantile(DistType<double>(p1, p2), p); \
    } \
    double math_dist_##name##_mean(double p1, double p2) { \
        return mean(DistType<double>(p1, p2)); \
    } \
    double math_dist_##name##_variance(double p1, double p2) { \
        return variance(DistType<double>(p1, p2)); \
    } \
    double math_dist_##name##_std_dev(double p1, double p2) { \
        return standard_deviation(DistType<double>(p1, p2)); \
    } \
    double math_dist_##name##_skewness(double p1, double p2) { \
        return skewness(DistType<double>(p1, p2)); \
    } \
    double math_dist_##name##_kurtosis(double p1, double p2) { \
        return kurtosis(DistType<double>(p1, p2)); \
    } \
    double math_dist_##name##_kurtosis_excess(double p1, double p2) { \
        return kurtosis_excess(DistType<double>(p1, p2)); \
    } \
    double math_dist_##name##_median(double p1, double p2) { \
        return median(DistType<double>(p1, p2)); \
    } \
    double math_dist_##name##_mode(double p1, double p2) { return mode(DistType<double>(p1, p2)); }

// Macro for distributions with 3 parameters
#define DIST_FNS_3(name, DistType) \
    double math_dist_##name##_pdf(double p1, double p2, double p3, double x) { \
        return pdf(DistType<double>(p1, p2, p3), x); \
    } \
    double math_dist_##name##_cdf(double p1, double p2, double p3, double x) { \
        return cdf(DistType<double>(p1, p2, p3), x); \
    } \
    double math_dist_##name##_quantile(double p1, double p2, double p3, double p) { \
        return quantile(DistType<double>(p1, p2, p3), p); \
    } \
    double math_dist_##name##_mean(double p1, double p2, double p3) { \
        return mean(DistType<double>(p1, p2, p3)); \
    } \
    double math_dist_##name##_variance(double p1, double p2, double p3) { \
        return variance(DistType<double>(p1, p2, p3)); \
    } \
    double math_dist_##name##_std_dev(double p1, double p2, double p3) { \
        return standard_deviation(DistType<double>(p1, p2, p3)); \
    } \
    double math_dist_##name##_skewness(double p1, double p2, double p3) { \
        return skewness(DistType<double>(p1, p2, p3)); \
    } \
    double math_dist_##name##_kurtosis(double p1, double p2, double p3) { \
        return kurtosis(DistType<double>(p1, p2, p3)); \
    } \
    double math_dist_##name##_kurtosis_excess(double p1, double p2, double p3) { \
        return kurtosis_excess(DistType<double>(p1, p2, p3)); \
    } \
    double math_dist_##name##_median(double p1, double p2, double p3) { \
        return median(DistType<double>(p1, p2, p3)); \
    } \
    double math_dist_##name##_mode(double p1, double p2, double p3) { \
        return mode(DistType<double>(p1, p2, p3)); \
    }

// 1-parameter distributions
DIST_FNS_1(bernoulli, bernoulli_distribution)
DIST_FNS_1(chi_squared, chi_squared_distribution)
DIST_FNS_1(exponential, exponential_distribution)
DIST_FNS_1(geometric, geometric_distribution)
DIST_FNS_1(inverse_chi_squared, inverse_chi_squared_distribution)
DIST_FNS_1(kolmogorov_smirnov, kolmogorov_smirnov_distribution)
DIST_FNS_1(poisson, poisson_distribution)
DIST_FNS_1(rayleigh, rayleigh_distribution)
DIST_FNS_1(students_t, students_t_distribution)

// 2-parameter distributions
DIST_FNS_2(arcsine, arcsine_distribution)
DIST_FNS_2(beta, beta_distribution)
DIST_FNS_2(binomial, binomial_distribution)
// Cauchy: mean, variance, std_dev, skewness, kurtosis, kurtosis_excess are undefined
double math_dist_cauchy_pdf(double p1, double p2, double x) {
    return pdf(cauchy_distribution<double>(p1, p2), x);
}
double math_dist_cauchy_cdf(double p1, double p2, double x) {
    return cdf(cauchy_distribution<double>(p1, p2), x);
}
double math_dist_cauchy_quantile(double p1, double p2, double p) {
    return quantile(cauchy_distribution<double>(p1, p2), p);
}
double math_dist_cauchy_mean(double, double) { return std::numeric_limits<double>::quiet_NaN(); }
double math_dist_cauchy_variance(double, double) {
    return std::numeric_limits<double>::quiet_NaN();
}
double math_dist_cauchy_std_dev(double, double) { return std::numeric_limits<double>::quiet_NaN(); }
double math_dist_cauchy_skewness(double, double) {
    return std::numeric_limits<double>::quiet_NaN();
}
double math_dist_cauchy_kurtosis(double, double) {
    return std::numeric_limits<double>::quiet_NaN();
}
double math_dist_cauchy_kurtosis_excess(double, double) {
    return std::numeric_limits<double>::quiet_NaN();
}
double math_dist_cauchy_median(double p1, double p2) {
    return median(cauchy_distribution<double>(p1, p2));
}
double math_dist_cauchy_mode(double p1, double p2) {
    return mode(cauchy_distribution<double>(p1, p2));
}
DIST_FNS_2(extreme_value, extreme_value_distribution)
DIST_FNS_2(fisher_f, fisher_f_distribution)
DIST_FNS_2(gamma, gamma_distribution)
DIST_FNS_2(inverse_gamma, inverse_gamma_distribution)
DIST_FNS_2(inverse_gaussian, inverse_gaussian_distribution)
DIST_FNS_2(laplace, laplace_distribution)
DIST_FNS_2(logistic, logistic_distribution)
DIST_FNS_2(lognormal, lognormal_distribution)
DIST_FNS_2(negative_binomial, negative_binomial_distribution)
DIST_FNS_2(non_central_chi_squared, non_central_chi_squared_distribution)
DIST_FNS_2(non_central_t, non_central_t_distribution)
DIST_FNS_2(normal, normal_distribution)
DIST_FNS_2(pareto, pareto_distribution)
DIST_FNS_2(uniform, uniform_distribution)
DIST_FNS_2(weibull, weibull_distribution)

// 3-parameter distributions
// Non-central beta: skewness and kurtosis_excess are undefined
double math_dist_non_central_beta_pdf(double p1, double p2, double p3, double x) {
    return pdf(non_central_beta_distribution<double>(p1, p2, p3), x);
}
double math_dist_non_central_beta_cdf(double p1, double p2, double p3, double x) {
    return cdf(non_central_beta_distribution<double>(p1, p2, p3), x);
}
double math_dist_non_central_beta_quantile(double p1, double p2, double p3, double p) {
    return quantile(non_central_beta_distribution<double>(p1, p2, p3), p);
}
double math_dist_non_central_beta_mean(double p1, double p2, double p3) {
    return mean(non_central_beta_distribution<double>(p1, p2, p3));
}
double math_dist_non_central_beta_variance(double p1, double p2, double p3) {
    return variance(non_central_beta_distribution<double>(p1, p2, p3));
}
double math_dist_non_central_beta_std_dev(double p1, double p2, double p3) {
    return standard_deviation(non_central_beta_distribution<double>(p1, p2, p3));
}
double math_dist_non_central_beta_skewness(double, double, double) {
    return std::numeric_limits<double>::quiet_NaN();
}
double math_dist_non_central_beta_kurtosis(double, double, double) {
    return std::numeric_limits<double>::quiet_NaN();
}
double math_dist_non_central_beta_kurtosis_excess(double, double, double) {
    return std::numeric_limits<double>::quiet_NaN();
}
double math_dist_non_central_beta_median(double p1, double p2, double p3) {
    return median(non_central_beta_distribution<double>(p1, p2, p3));
}
double math_dist_non_central_beta_mode(double p1, double p2, double p3) {
    return mode(non_central_beta_distribution<double>(p1, p2, p3));
}
DIST_FNS_3(non_central_f, non_central_f_distribution)
DIST_FNS_3(skew_normal, skew_normal_distribution)
DIST_FNS_3(triangular, triangular_distribution)

// Hypergeometric distribution (unsigned parameters)
double math_dist_hypergeometric_pdf(unsigned r, unsigned n, unsigned N, double x) {
    return pdf(hypergeometric_distribution<double>(r, n, N), x);
}
double math_dist_hypergeometric_cdf(unsigned r, unsigned n, unsigned N, double x) {
    return cdf(hypergeometric_distribution<double>(r, n, N), x);
}
double math_dist_hypergeometric_quantile(unsigned r, unsigned n, unsigned N, double p) {
    return quantile(hypergeometric_distribution<double>(r, n, N), p);
}
double math_dist_hypergeometric_mean(unsigned r, unsigned n, unsigned N) {
    return mean(hypergeometric_distribution<double>(r, n, N));
}
double math_dist_hypergeometric_variance(unsigned r, unsigned n, unsigned N) {
    return variance(hypergeometric_distribution<double>(r, n, N));
}
double math_dist_hypergeometric_std_dev(unsigned r, unsigned n, unsigned N) {
    return standard_deviation(hypergeometric_distribution<double>(r, n, N));
}
double math_dist_hypergeometric_skewness(unsigned r, unsigned n, unsigned N) {
    return skewness(hypergeometric_distribution<double>(r, n, N));
}
double math_dist_hypergeometric_kurtosis(unsigned r, unsigned n, unsigned N) {
    return kurtosis(hypergeometric_distribution<double>(r, n, N));
}
double math_dist_hypergeometric_kurtosis_excess(unsigned r, unsigned n, unsigned N) {
    return kurtosis_excess(hypergeometric_distribution<double>(r, n, N));
}
double math_dist_hypergeometric_median(unsigned r, unsigned n, unsigned N) {
    return median(hypergeometric_distribution<double>(r, n, N));
}
double math_dist_hypergeometric_mode(unsigned r, unsigned n, unsigned N) {
    return mode(hypergeometric_distribution<double>(r, n, N));
}

} // extern "C" distributions
