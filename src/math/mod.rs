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
//! - [x] Gamma
//!   - [`gamma`]
//!   - [`gamma1pm1`]
//! - [x] Log Gamma
//!   - [`lgamma`]
//! - [x] Polygamma
//!   - [`digamma`]
//!   - [`trigamma`]
//!   - [`polygamma`]
//! - [x] Ratios of Gamma Functions
//!   - [`gamma_ratio`]
//!   - [`gamma_delta_ratio`]
//! - [x] Incomplete Gamma Functions
//!   - [`gamma_q`]
//!   - [`gamma_p`]
//!   - [`gamma_upper`]
//!   - [`gamma_lower`]
//! - [x] Incomplete Gamma Function inverses
//!   - [`gamma_q_inv`]
//!   - [`gamma_q_inva`]
//!   - [`gamma_p_inv`]
//!   - [`gamma_p_inva`]
//! - [x] Derivative of the Incomplete Gamma Function
//!   - [`gamma_p_derivative`]
//!
//! ### Factorials and Binomial Coefficients
//!
//! - [x] [`factorial`]
//! - [x] [`double_factorial`]
//! - [x] [`rising_factorial`]
//! - [x] [`falling_factorial`]
//! - [x] [`binomial_coefficient`]
//!
//! ### Beta Functions
//!
//! - [x] Beta
//!   - [`beta`]
//! - [x] Incomplete Beta Functions
//!   - [`ibeta`]
//!   - [`ibetac`]
//!   - [`beta_`]
//!   - [`betac`]
//! - [x] The Incomplete Beta Function Inverses
//!   - [`ibeta_inv`]
//!   - [`ibetac_inv`]
//!   - [`ibeta_inva`]
//!   - [`ibetac_inva`]
//!   - [`ibeta_invb`]
//!   - [`ibetac_invb`]
//! - [x] Derivative of the Incomplete Beta Function
//!   - [`ibeta_derivative`]
//!
//! ### Error Functions
//!
//! - [x] Error Function *erf* and complement *erfc*
//!   - [`erf`]
//!   - [`erfc`]
//! - [x] Error Function Inverses
//!   - [`erf_inv`]
//!   - [`erfc_inv`]
//!
//! ### Polynomials
//!
//! - [x] Chebyshev Polynomials
//!   - [`chebyshev_t`]
//!   - [`chebyshev_t_prime`]
//!   - [`chebyshev_u`]
//!   - [`chebyshev_next`]
//! - [x] Legendre (and associated) polynomials
//!   - [`legendre_p`]
//!   - [`legendre_p_prime`]
//!   - [`legendre_p_zeros`]
//!   - [`legendre_q`]
//!   - [`legendre_next`]
//!   - [`legendre_p_assoc`]
//!   - [`legendre_assoc_next`]
//! - [x] Laguerre (and Associated) Polynomials
//!   - [`laguerre`]
//!   - [`laguerre_next`]
//!   - [`laguerre_assoc`]
//!   - [`laguerre_assoc_next`]
//! - [x] Hermite Polynomials
//!   - [`hermite`]
//!   - [`hermite_next`]
//! - [x] Gegenbauer Polynomials
//!   - [`gegenbauer`]
//!   - [`gegenbauer_derivative`]
//! - [x] Jacobi Polynomials
//!   - [`jacobi`]
//!   - [`jacobi_derivative`]
//! - [x] Spherical Harmonics
//!   - [`spherical_harmonic`]
//!   - [`spherical_harmonic_r`]
//!   - [`spherical_harmonic_i`]
//! - [ ] Cardinal B-splines
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
//! - [x] Airy *Ai* Function
//!   - [`airy_ai`]
//! - [x] Airy *Bi* Function
//!   - [`airy_bi`]
//! - [x] Airy *Ai'* Function
//!   - [`airy_ai_prime`]
//! - [x] Airy *Bi'* Function
//!   - [`airy_bi_prime`]
//! - [x] Finding Zeros of Airy Functions
//!   - [`airy_ai_zero`]
//!   - [`airy_bi_zero`]
//!
//! <h4>Elliptic Integrals</h4>
//!
//! - [ ] Elliptic Integrals - Carlson Form
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
//! - [x] [`lambert_w0`]
//! - [x] [`lambert_w0_prime`]
//! - [x] [`lambert_wm1`]
//! - [x] [`lambert_wm1_prime`]
//!
//! ### Zeta Functions
//!
//! - [x] Riemann zeta function
//!   - [`zeta`]
//!
//! ### Exponential Integrals
//!
//! - [x] Exponential Integral *Ei(x)*
//!   - [`expint_i`]
//! - [x] Exponential Integral *E<sub>n</sub>(x)*
//!   - [`expint_n`]
//!
//! ### Hypergeometric Functions
//!
//! - [x] Hypergeometric *<sub>1</sub>F<sub>0</sub>*
//!   - [`hypergeometric_1f0`]
//! - [x] Hypergeometric *<sub>0</sub>F<sub>1</sub>*
//!   - [`hypergeometric_0f1`]
//! - [x] Hypergeometric *<sub>2</sub>F<sub>0</sub>*
//!   - [`hypergeometric_2f0`]
//! - [x] Hypergeometric *<sub>1</sub>F<sub>1</sub>*
//!   - [`hypergeometric_1f1`]
//!   - [`hypergeometric_1f1_regularized`]
//!   - [`log_hypergeometric_1f1`]
//! - [ ] ~Hypergeometric *<sub>p</sub>F<sub>q</sub>*~ (not included due to bugs in Boost)
//!
//! ### Basic Functions
//!
//! - [x] [`sin_pi`] and [`cos_pi`]
//! - [x] [`log1p`] and [`expm1`]
//! - [x] [`powm1`]
//! - [x] [`rsqrt`]
//! - [x] [`sqrt1pm1`]
//! - [x] [`cbrt`]
//! - [x] [`hypot`]
//! - [x] [`logaddexp`] and [`logsumexp`]
//!
//! ### Sinus Cardinal and Hyperbolic Sinus Cardinal Functions
//!
//! - [x] [`sinc_pi`]
//! - [x] [`sinhc_pi`]
//!
//! ### Inverse Hyperbolic Functions
//!
//! - [x] [`acosh`]
//! - [x] [`asinh`]
//! - [x] [`atanh`]
//!
//! ### Owen's T Function
//!
//! - [x] [`owens_t`]
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
pub use special_functions::cbrt::*;
pub use special_functions::chebyshev::*;
pub use special_functions::cos_pi::*;
pub use special_functions::digamma::*;
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
