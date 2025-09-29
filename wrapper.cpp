// Simple C++ wrappers for the Boost Math functions for https://github.com/jorenham/boost-rust

#ifndef BOOST_MATH_TOOLS_USER_HPP
    // https://www.boost.org/doc/libs/latest/boost/math/tools/user.hpp
    #define BOOST_MATH_TOOLS_USER_HPP
    #define BOOST_MATH_NO_LONG_DOUBLE_MATH_FUNCTIONS
    #define BOOST_MATH_DOMAIN_ERROR_POLICY errno_on_error
    #define BOOST_MATH_POLE_ERROR_POLICY errno_on_error
    #define BOOST_MATH_OVERFLOW_ERROR_POLICY errno_on_error
    #define BOOST_MATH_EVALUATION_ERROR_POLICY throw_on_error
    #define BOOST_MATH_UNDERFLOW_ERROR_POLICY ignore_error
    #define BOOST_MATH_DENORM_ERROR_POLICY ignore_error
    #define BOOST_MATH_ASSERT_UNDEFINED_POLICY true
    #define BOOST_MATH_DISABLE_FLOAT128
#endif

#include <boost/math/special_functions/bessel.hpp>
#include <boost/math/special_functions/beta.hpp>
#include <boost/math/special_functions/digamma.hpp>
#include <boost/math/special_functions/erf.hpp>
#include <boost/math/special_functions/factorials.hpp>
#include <boost/math/special_functions/gamma.hpp>
#include <boost/math/special_functions/jacobi.hpp>
#include <boost/math/special_functions/legendre.hpp>

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

// boost/math/special_functions/digamma.hpp
double math_digamma(double x) { return digamma(x); }

// boost/math/special_functions/erf.hpp
double math_erf(double x) { return erf(x); }
double math_erfc(double x) { return erfc(x); }

// boost/math/special_functions/factorials.hpp
double math_factorial(unsigned i) { return factorial<double>(i); }
double math_double_factorial(unsigned i) { return double_factorial<double>(i); }
double math_falling_factorial(double x, unsigned n) { return falling_factorial(x, n); }
double math_rising_factorial(double x, int n) { return rising_factorial(x, n); }

// boost/math/special_functions/gamma.hpp
double math_tgamma(double x) { return tgamma(x); }
double math_tgamma1pm1(double x) { return tgamma1pm1(x); }
double math_lgamma(double x, int* sign) { return lgamma(x, sign); }
double math_tgamma_lower(double a, double x) { return tgamma_lower(a, x); }
double math_tgamma_(double a, double x) { return tgamma(a, x); }
double math_gamma_p(double a, double x) { return gamma_p(a, x); }
double math_gamma_q(double a, double x) { return gamma_q(a, x); }

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
void math_legendre_p_zeros(int l, double* out) {
    // `out` must be of size `l.div_ceil(2)`
    auto vec = legendre_p_zeros<double>(l);
    for (size_t i = 0; i < vec.size(); i++) {
        out[i] = vec[i];
    }
}
double math_legendre_q(unsigned l, double x) { return legendre_q(l, x); }

} // extern "C"
