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

extern "C" {

// <boost/math/special_functions/bessel.hpp>
// https://www.boost.org/doc/libs/1_89_0/libs/math/doc/html/math_toolkit/bessel.html
// TODO:
// - cyl_bessel_j_zero
// - cyl_neumann_zero

double boost_math_cyl_bessel_j(double nu, double x) {
  return boost::math::cyl_bessel_j(nu, x);
}

double boost_math_cyl_neumann(double nu, double x) {
  return boost::math::cyl_neumann(nu, x);
}

double boost_math_cyl_bessel_i(double nu, double x) {
  return boost::math::cyl_bessel_i(nu, x);
}

double boost_math_cyl_bessel_k(double nu, double x) {
  return boost::math::cyl_bessel_k(nu, x);
}

double boost_math_sph_bessel(unsigned n, double x) {
  return boost::math::sph_bessel(n, x);
}

double boost_math_sph_neumann(unsigned n, double x) {
  return boost::math::sph_neumann(n, x);
}

// <boost/math/special_functions/beta.hpp>
// https://www.boost.org/doc/libs/1_89_0/libs/math/doc/html/math_toolkit/sf_beta.html
// TODO:
// - betac
// - ibeta
// - ibetac
// - ibeta_derivative

double boost_math_beta(double a, double b) {
  try {
    return boost::math::beta(a, b);
  } catch (const std::domain_error &e) {
    return std::numeric_limits<double>::quiet_NaN();
  } catch (const std::overflow_error &e) {
    return std::numeric_limits<double>::infinity();
  } catch (const std::underflow_error &e) {
    return 0.0;
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

// <boost/math/special_functions/digamma.hpp>
// https://www.boost.org/doc/libs/1_89_0/libs/math/doc/html/math_toolkit/sf_gamma/digamma.html
double boost_math_digamma(double x) { return boost::math::digamma(x); }

// <boost/math/special_functions/erf.hpp>
// https://www.boost.org/doc/libs/1_89_0/libs/math/doc/html/math_toolkit/sf_erf.html
// TODO: (from <boost/math/special_functions/detail/erf_inv.hpp>)
// - erf_inv
// - erfc_inv

double boost_math_erf(double x) { return boost::math::erf(x); }
double boost_math_erfc(double x) { return boost::math::erfc(x); }

// <boost/math/special_functions/gamma.hpp>
// https://www.boost.org/doc/libs/1_89_0/libs/math/doc/html/math_toolkit/sf_gamma.html
// TODO:
// - tgamma1pm1
// - tgamma_lower
// - tgamma_delta_ratio
// - tgamma_ratio
// - gamma_p_derivative
// - gamma_p_inv
// - gamma_p_inva
// - gamma_q_inv
// - gamma_q_inva
double boost_math_tgamma(double x) { return boost::math::tgamma(x); }
double boost_math_lgamma(double x) { return boost::math::lgamma(x); }

double boost_math_gamma_p(double a, double x) {
  return boost::math::gamma_p(a, x);
}

double boost_math_gamma_q(double a, double x) {
  return boost::math::gamma_q(a, x);
}

} // extern "C"
