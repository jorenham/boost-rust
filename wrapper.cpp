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

#include <boost/math/special_functions/bessel.hpp>
#include <boost/math/special_functions/beta.hpp>
#include <boost/math/special_functions/binomial.hpp>
#include <boost/math/special_functions/cbrt.hpp>
#include <boost/math/special_functions/digamma.hpp>
#include <boost/math/special_functions/erf.hpp>
#include <boost/math/special_functions/factorials.hpp>
#include <boost/math/special_functions/gamma.hpp>
#include <boost/math/special_functions/hypergeometric_0F1.hpp>
#include <boost/math/special_functions/hypergeometric_1F0.hpp>
#include <boost/math/special_functions/hypergeometric_1F1.hpp>
#include <boost/math/special_functions/hypergeometric_2F0.hpp>
#include <boost/math/special_functions/jacobi.hpp>
#include <boost/math/special_functions/legendre.hpp>
#include <boost/math/special_functions/logsumexp.hpp>
#include <boost/math/special_functions/polygamma.hpp>
#include <boost/math/special_functions/prime.hpp>
#include <boost/math/special_functions/rsqrt.hpp>
#include <boost/math/special_functions/sqrt1pm1.hpp>
#include <boost/math/special_functions/trigamma.hpp>
#include <boost/math/special_functions/zeta.hpp>

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

extern "C" {

// boost/math/special_functions/bessel.hpp
double math_cyl_bessel_j(double nu, double x) { return cyl_bessel_j(nu, x); }
double math_cyl_neumann(double nu, double x) { return cyl_neumann(nu, x); }
double math_cyl_bessel_i(double nu, double x) { return cyl_bessel_i(nu, x); }
double math_cyl_bessel_k(double nu, double x) { return cyl_bessel_k(nu, x); }
double math_sph_bessel(unsigned n, double x) { return sph_bessel(n, x); }
double math_sph_neumann(unsigned n, double x) { return sph_neumann(n, x); }

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
double math_cbrt(double x) { return cbrt(x); }

// boost/math/special_functions/digamma.hpp
double math_digamma(double x) { return ::detail::polygamma(0, x); }

// boost/math/special_functions/erf.hpp
double math_erf(double x) { return erf(x); }
double math_erfc(double x) { return erfc(x); }
double math_erf_inv(double p) { return erf_inv(p); }
double math_erfc_inv(double q) { return erfc_inv(q); }

// boost/math/special_functions/factorials.hpp
double math_factorial(unsigned i) { return factorial<double>(i); }
double math_double_factorial(unsigned i) { return double_factorial<double>(i); }
double math_falling_factorial(double x, unsigned n) { return falling_factorial(x, n); }
double math_rising_factorial(double x, int n) { return rising_factorial(x, n); }

// boost/math/special_functions/gamma.hpp
double math_tgamma(double x) { return tgamma(x); }
double math_tgamma1pm1(double x) { return tgamma1pm1(x); }
double math_tgamma_(double a, double x) { return tgamma(a, x); }
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

// boost/math/special_functions/jacobi.hpp
double math_jacobi(unsigned n, double alpha, double beta, double x) {
    return jacobi(n, alpha, beta, x);
}
double math_jacobi_derivative(unsigned n, double alpha, double beta, double x, unsigned k) {
    return jacobi_derivative(n, alpha, beta, x, k);
}

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

// boost/math/special_functions/logsumexp.hpp
double math_logaddexp(double x1, double x2) { return logaddexp(x1, x2); }
double math_logsumexp(const double args[], size_t len) { return logsumexp(args, args + len); }

// boost/math/special_functions/polygamma.hpp
double math_polygamma(const int n, double x) { return ::detail::polygamma(n, x); }

// boost/math/special_functions/prime.hpp
std::uint32_t math_prime(unsigned n) { return prime(n); }

// boost/math/special_functions/rsqrt.hpp
double math_sqrt(double x) { return sqrt(x); }
double math_rsqrt(double x) { return rsqrt(x); }

// boost/math/special_functions/sqrt1pm1.hpp
double math_sqrt1pm1(double x) { return sqrt1pm1(x); }

// boost/math/special_functions/trigamma.hpp
double math_trigamma(double x) { return ::detail::polygamma(1, x); }

// boost/math/special_functions/zeta.hpp
double math_zeta(double s) { return zeta(s); }

} // extern "C"
