//! Raw FFI declarations for cpp/wrapper.h

use core::ffi::c_uint;

unsafe extern "C" {
    // <boost/math/special_functions/bessel.hpp>
    pub fn math_cyl_bessel_j(nu: f64, x: f64) -> f64;
    pub fn math_cyl_neumann(nu: f64, x: f64) -> f64;
    pub fn math_cyl_bessel_i(nu: f64, x: f64) -> f64;
    pub fn math_cyl_bessel_k(nu: f64, x: f64) -> f64;
    pub fn math_sph_bessel(n: c_uint, x: f64) -> f64;
    pub fn math_sph_neumann(n: c_uint, x: f64) -> f64;

    // <boost/math/special_functions/beta.hpp>
    pub fn math_beta(a: f64, b: f64) -> f64;

    // <boost/math/special_functions/digamma.hpp>
    pub fn math_digamma(x: f64) -> f64;

    // <boost/math/special_functions/erf.hpp>
    pub fn math_erf(x: f64) -> f64;
    pub fn math_erfc(x: f64) -> f64;

    // <boost/math/special_functions/gamma.hpp>
    pub fn math_tgamma(x: f64) -> f64;
    pub fn math_lgamma(x: f64) -> f64;
    pub fn math_gamma_p(a: f64, x: f64) -> f64;
    pub fn math_gamma_q(a: f64, x: f64) -> f64;
}
