//! Raw FFI declarations for cpp/wrapper.h

use core::ffi::c_uint;

unsafe extern "C" {

    // <boost/math/special_functions/bessel.hpp>
    pub fn boost_math_cyl_bessel_j(nu: f64, x: f64) -> f64;
    pub fn boost_math_cyl_neumann(nu: f64, x: f64) -> f64;
    pub fn boost_math_cyl_bessel_i(nu: f64, x: f64) -> f64;
    pub fn boost_math_cyl_bessel_k(nu: f64, x: f64) -> f64;
    pub fn boost_math_sph_bessel(n: c_uint, x: f64) -> f64;
    pub fn boost_math_sph_neumann(n: c_uint, x: f64) -> f64;

    // <boost/math/special_functions/beta.hpp>
    pub fn boost_math_beta(a: f64, b: f64) -> f64;

    // <boost/math/special_functions/digamma.hpp>
    pub fn boost_math_digamma(x: f64) -> f64;

    // <boost/math/special_functions/erf.hpp>
    pub fn boost_math_erf(x: f64) -> f64;
    pub fn boost_math_erfc(x: f64) -> f64;

    // <boost/math/special_functions/gamma.hpp>
    pub fn boost_math_tgamma(x: f64) -> f64;
    pub fn boost_math_lgamma(x: f64) -> f64;
    pub fn boost_math_gamma_p(a: f64, x: f64) -> f64;
    pub fn boost_math_gamma_q(a: f64, x: f64) -> f64;

}
