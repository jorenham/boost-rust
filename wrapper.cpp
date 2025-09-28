// Simple C++ wrappers for the Boost Math functions

// Boost static config, see
// https://www.boost.org/doc/libs/latest/boost/math/tools/user.hpp
#ifndef BOOST_MATH_TOOLS_USER_HPP
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
#include <boost/math/special_functions/gamma.hpp>
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

// boost/math/special_functions/digamma.hpp
double math_digamma(double x) { return digamma(x); }

// boost/math/special_functions/erf.hpp
double math_erf(double x) { return erf(x); }
double math_erfc(double x) { return erfc(x); }

// boost/math/special_functions/gamma.hpp
double math_tgamma(double x) { return tgamma(x); }
double math_lgamma(double x) { return lgamma(x); }
double math_gamma_p(double a, double x) { return gamma_p(a, x); }
double math_gamma_q(double a, double x) { return gamma_q(a, x); }

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
