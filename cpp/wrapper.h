#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// <boost/math/special_functions/bessel.hpp>
double boost_math_cyl_bessel_j(double nu, double x);
double boost_math_cyl_neumann(double nu, double x);
double boost_math_cyl_bessel_i(double nu, double x);
double boost_math_cyl_bessel_k(double nu, double x);
double boost_math_sph_bessel(unsigned n, double x);
double boost_math_sph_neumann(unsigned n, double x);

// <boost/math/special_functions/beta.hpp>
double boost_math_beta(double a, double b);

// <boost/math/special_functions/digamma.hpp>
double boost_math_digamma(double x);

// <boost/math/special_functions/erf.hpp>
double boost_math_erf(double x);
double boost_math_erfc(double x);

// <boost/math/special_functions/gamma.hpp>
double boost_math_tgamma(double x);
double boost_math_lgamma(double x);
double boost_math_gamma_p(double a, double x);
double boost_math_gamma_q(double a, double x);

#ifdef __cplusplus
}
#endif
