#include "wrapper.h"

#include <cmath>
#include <limits>
#include <stdexcept>

#include <boost/math/special_functions.hpp>

extern "C" {

// <boost/math/special_functions/bessel.hpp>
// https://www.boost.org/doc/libs/1_89_0/libs/math/doc/html/math_toolkit/bessel.html
// TODO:
// - cyl_bessel_j_zero
// - cyl_neumann_zero

double boost_math_cyl_bessel_j(double nu, double x) {
  try {
    return boost::math::cyl_bessel_j(nu, x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

double boost_math_cyl_neumann(double nu, double x) {
  try {
    return boost::math::cyl_neumann(nu, x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

double boost_math_cyl_bessel_i(double nu, double x) {
  try {
    return boost::math::cyl_bessel_i(nu, x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

double boost_math_cyl_bessel_k(double nu, double x) {
  try {
    return boost::math::cyl_bessel_k(nu, x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

double boost_math_sph_bessel(unsigned n, double x) {
  try {
    return boost::math::sph_bessel(n, x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

double boost_math_sph_neumann(unsigned n, double x) {
  try {
    return boost::math::sph_neumann(n, x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
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
double boost_math_digamma(double x) {
  try {
    return boost::math::digamma(x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

// <boost/math/special_functions/erf.hpp>
// https://www.boost.org/doc/libs/1_89_0/libs/math/doc/html/math_toolkit/sf_erf.html
// TODO: (from <boost/math/special_functions/detail/erf_inv.hpp>)
// - erf_inv
// - erfc_inv

double boost_math_erf(double x) {
  try {
    return boost::math::erf(x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

double boost_math_erfc(double x) {
  try {
    return boost::math::erfc(x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

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
double boost_math_tgamma(double x) {
  try {
    return boost::math::tgamma(x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

double boost_math_lgamma(double x) {
  try {
    return boost::math::lgamma(x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

double boost_math_gamma_p(double a, double x) {
  try {
    return boost::math::gamma_p(a, x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

double boost_math_gamma_q(double a, double x) {
  try {
    return boost::math::gamma_q(a, x);
  } catch (...) {
    return std::numeric_limits<double>::quiet_NaN();
  }
}

} // extern "C"
