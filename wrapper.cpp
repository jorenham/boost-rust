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

extern "C" {
// boost/math/ccmath/sqrt.hpp
double math_ccmath_sqrt(double x) { return ccmath::sqrt(x); }

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
