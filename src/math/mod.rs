//! Bindings for Boost Math C++ library
//!
//! See [github.com/boostorg/math](https://github.com/boostorg/math) for more details.
//!
//! ## Special Functions
//!
//! <https://boost.org/doc/libs/latest/libs/math/doc/html/special.html>
//!
//! ### Number Series
//!
//! - [ ] Bernoulli numbers
//! - [ ] Tangent numbers
//! - [x] Prime numbers
//!   - [`prime`]
//! - [ ] Fibonacci numbers
//!
//! ### Gamma Functions
//!
//! - Gamma
//!   - [`gamma`]
//!   - [`gamma1pm1`]
//! - Log Gamma
//!   - [`lgamma`]
//! - Polygamma
//!   - [`digamma`]
//!   - [`trigamma`]
//!   - [`polygamma`]
//! - Ratios of Gamma Functions
//!   - [`gamma_ratio`]
//!   - [`gamma_delta_ratio`]
//! - Incomplete Gamma Functions
//!   - [`gamma_q`]
//!   - [`gamma_p`]
//!   - [`gamma_upper`]
//!   - [`gamma_lower`]
//! - Incomplete Gamma Function inverses
//!   - [`gamma_q_inv`]
//!   - [`gamma_q_inva`]
//!   - [`gamma_p_inv`]
//!   - [`gamma_p_inva`]
//! - Derivative of the Incomplete Gamma Function
//!   - [`gamma_p_derivative`]
//!
//! ### Factorials and Binomial Coefficients
//!
//! - [`factorial`]
//! - [`double_factorial`]
//! - [`rising_factorial`]
//! - [`falling_factorial`]
//! - [`binomial_coefficient`]
//!
//! ### Beta Functions
//!
//! - Beta function
//!   - [`beta`]
//! - Incomplete beta functions
//!   - [`beta_`]
//!   - [`betac`]
//! - Regularized incomplete beta functions
//!   - [`ibeta`]
//!   - [`ibetac`]
//! - Regularized incomplete beta function inverses
//!   - [`ibeta_inv`]
//!   - [`ibeta_inva`]
//!   - [`ibeta_invb`]
//!   - [`ibetac_inv`]
//!   - [`ibetac_inva`]
//!   - [`ibetac_invb`]
//! - Regularized incomplete beta function derivative
//!   - [`ibeta_derivative`]
//!
//! ### Error Functions
//!
//! | Error function | Inverse function |
//! | -------------- | ---------------- |
//! | [`erf`]        | [`erf_inv`]      |
//! | [`erfc`]       | [`erfc_inv`]     |
//!
//! ### Polynomials
//!
//! - Chebyshev Polynomials
//!   - [`chebyshev_t`]
//!   - [`chebyshev_t_prime`]
//!   - [`chebyshev_u`]
//!   - [`chebyshev_next`]
//! - Legendre (and associated) polynomials
//!   - [`legendre_p`]
//!   - [`legendre_p_prime`]
//!   - [`legendre_p_zeros`]
//!   - [`legendre_q`]
//!   - [`legendre_next`]
//!   - [`legendre_p_assoc`]
//!   - [`legendre_assoc_next`]
//! - Laguerre (and Associated) Polynomials
//!   - [`laguerre`]
//!   - [`laguerre_next`]
//!   - [`laguerre_assoc`]
//!   - [`laguerre_assoc_next`]
//! - Hermite Polynomials
//!   - [`hermite`]
//!   - [`hermite_next`]
//! - Gegenbauer Polynomials
//!   - [`gegenbauer`]
//!   - [`gegenbauer_derivative`]
//! - Jacobi Polynomials
//!   - [`jacobi`]
//!   - [`jacobi_derivative`]
//! - Spherical Harmonics
//!   - [`spherical_harmonic`]
//!   - [`spherical_harmonic_r`]
//!   - [`spherical_harmonic_i`]
//! - Cardinal B-splines
//!   - [`cardinal_b_spline`]
//!   - [`cardinal_b_spline_prime`]
//!   - [`cardinal_b_spline_double_prime`]
//!   - [`forward_cardinal_b_spline`]
//!
//! ### Bessel and Hankel Functions
//!
//! |                   Type | First Kind       | Second Kind      |
//! | ---------------------: | ---------------- | ---------------- |
//! |          Cyclic Bessel | [`cyl_bessel_j`] | [`cyl_neumann`]  |
//! | Modified Cyclic Bessel | [`cyl_bessel_i`] | [`cyl_bessel_k`] |
//! |       Spherical Bessel | [`sph_bessel`]   | [`sph_neumann`]  |
//! |          Cyclic Hankel | [`cyl_hankel_1`] | [`cyl_hankel_2`] |
//! |       Spherical Hankel | [`sph_hankel_1`] | [`sph_hankel_2`] |
//!
//! Note that the Hankel functions require the `num-complex` feature to be enabled.
//!
//! #### Zeros
//!
//! |                   Type | First Kind            | Second Kind      |
//! | ---------------------: | --------------------- | ---------------- |
//! |          Cyclic Bessel | [`cyl_bessel_j_zero`] | [`cyl_neumann_zero`]  |
//!
//! #### Derivatives
//!
//! |                   Type | First Kind             | Second Kind            |
//! | ---------------------: | ---------------------- | ---------------------- |
//! |          Cyclic Bessel | [`cyl_bessel_j_prime`] | [`cyl_neumann_prime`]  |
//! | Modified Cyclic Bessel | [`cyl_bessel_i_prime`] | [`cyl_bessel_k_prime`] |
//! |       Spherical Bessel | [`sph_bessel_prime`]   | [`sph_neumann_prime`]  |
//!
//! ### Airy Functions
//!
//! | Notation | Function    | First derivative  | Zeros            |
//! | -------- | ----------- | ----------------- | ---------------- |
//! | *Ai*     | [`airy_ai`] | [`airy_ai_prime`] | [`airy_ai_zero`] |
//! | *Bi*     | [`airy_bi`] | [`airy_bi_prime`] | [`airy_bi_zero`] |
//!
//! <h4>Elliptic Integrals</h4>
//!
//! - [x] Elliptic Integrals - Carlson Form
//!   - [`ellint_rc`](ellint_rc) - *R<sub>C</sub>(x,y)*
//!   - [`ellint_rd`](ellint_rd) - *R<sub>D</sub>(x,y,z)*
//!   - [`ellint_rf`](ellint_rf) - *R<sub>F</sub>(x,y,z)*
//!   - [`ellint_rg`](ellint_rg) - *R<sub>G</sub>(x,y,z)*
//!   - [`ellint_rj`](ellint_rj) - *R<sub>J</sub>(x,y,z,p)*
//! - [ ] Elliptic Integrals of the First Kind - Legendre Form
//! - [ ] Elliptic Integrals of the Second Kind - Legendre Form
//! - [ ] Elliptic Integrals of the Third Kind - Legendre Form
//! - [ ] Elliptic Integral *D* - Legendre Form
//! - [ ] Jacobi Zeta Function
//! - [ ] Heuman Lambda Function
//!
//! <h4>Jacobi Elliptic Functions</h4>
//!
//! - [ ] Jacobi Elliptic *SN*, *CN* and *DN*
//! - [ ] Jacobi Elliptic Function *cd*
//! - [ ] Jacobi Elliptic Function *cn*
//! - [ ] Jacobi Elliptic Function *cs*
//! - [ ] Jacobi Elliptic Function *dc*
//! - [ ] Jacobi Elliptic Function *dn*
//! - [ ] Jacobi Elliptic Function *ds*
//! - [ ] Jacobi Elliptic Function *nc*
//! - [ ] Jacobi Elliptic Function *nd*
//! - [ ] Jacobi Elliptic Function *ns*
//! - [ ] Jacobi Elliptic Function *sc*
//! - [ ] Jacobi Elliptic Function *sd*
//! - [ ] Jacobi Elliptic Function *sn*
//!
//! <h4>Jacobi Theta Functions</h4>
//!
//! - [ ] Jacobi Theta Function *θ<sub>1</sub>*
//! - [ ] Jacobi Theta Function *θ<sub>2</sub>*
//! - [ ] Jacobi Theta Function *θ<sub>3</sub>*
//! - [ ] Jacobi Theta Function *θ<sub>4</sub>*
//!
//! ### Lambert W Function
//!
//! | Branch | Function        | First derivative      |
//! | -----: | --------------- | --------------------- |
//! |      0 | [`lambert_w0`]  | [`lambert_w0_prime`]  |
//! |     -1 | [`lambert_wm1`] | [`lambert_wm1_prime`] |
//!
//! ### Zeta Functions
//!
//! - Riemann zeta function
//!   - [`zeta`]
//!
//! ### Exponential Integrals
//!
//! - Exponential Integral *Ei(x)*
//!   - [`expint_i`]
//! - Exponential Integral *E<sub>n</sub>(x)*
//!   - [`expint_n`]
//!
//! ### Hypergeometric Functions
//!
//! - Hypergeometric *<sub>1</sub>F<sub>0</sub>*
//!   - [`hypergeometric_1f0`]
//! - Hypergeometric *<sub>0</sub>F<sub>1</sub>*
//!   - [`hypergeometric_0f1`]
//! - Hypergeometric *<sub>2</sub>F<sub>0</sub>*
//!   - [`hypergeometric_2f0`]
//! - Hypergeometric *<sub>1</sub>F<sub>1</sub>*
//!   - [`hypergeometric_1f1`]
//!   - [`hypergeometric_1f1_regularized`]
//!   - [`log_hypergeometric_1f1`]
//! - ~Hypergeometric *<sub>p</sub>F<sub>q</sub>*~ (not included due to bugs in Boost)
//!
//! ### Basic Functions
//!
//! - [`sin_pi`] and [`cos_pi`]
//! - [`log1p`] and [`expm1`]
//! - [`powm1`]
//! - [`rsqrt`]
//! - [`sqrt1pm1`]
//! - [`cbrt`]
//! - [`hypot`]
//! - [`logaddexp`] and [`logsumexp`]
//!
//! ### Sinus Cardinal and Hyperbolic Sinus Cardinal Functions
//!
//! - [`sinc_pi`]
//! - [`sinhc_pi`]
//!
//! ### Inverse Hyperbolic Functions
//!
//! - [`acosh`]
//! - [`asinh`]
//! - [`atanh`]
//!
//! ### Owen's T Function
//!
//! - [`owens_t`]
//!
//! ## Constexpr CMath
//!
//! See [`ccmath`]
//!

pub mod ccmath;
mod special_functions;

pub use special_functions::acosh::*;
pub use special_functions::airy::*;
pub use special_functions::asinh::*;
pub use special_functions::atanh::*;
pub use special_functions::bessel::*;
pub use special_functions::bessel_prime::*;
pub use special_functions::beta::*;
pub use special_functions::binomial::*;
pub use special_functions::cardinal_b_spline::*;
pub use special_functions::cbrt::*;
pub use special_functions::chebyshev::*;
pub use special_functions::cos_pi::*;
pub use special_functions::digamma::*;
pub use special_functions::ellint_r::*;
pub use special_functions::erf::*;
pub use special_functions::expint::*;
pub use special_functions::expm1::*;
pub use special_functions::factorials::*;
pub use special_functions::gamma::*;
pub use special_functions::gegenbauer::*;
#[cfg(feature = "num-complex")]
pub use special_functions::hankel::*;
pub use special_functions::hermite::*;
pub use special_functions::hypergeometric::*;
pub use special_functions::hypot::*;
pub use special_functions::jacobi::*;
pub use special_functions::laguerre::*;
pub use special_functions::lambert_w::*;
pub use special_functions::legendre::*;
pub use special_functions::log1p::*;
pub use special_functions::logsumexp::*;
pub use special_functions::owens_t::*;
pub use special_functions::polygamma::*;
pub use special_functions::powm1::*;
pub use special_functions::prime::*;
pub use special_functions::rsqrt::*;
pub use special_functions::sin_pi::*;
pub use special_functions::sinc::*;
pub use special_functions::sinhc::*;
pub use special_functions::spherical_harmonic::*;
pub use special_functions::sqrt1pm1::*;
pub use special_functions::trigamma::*;
pub use special_functions::zeta::*;
