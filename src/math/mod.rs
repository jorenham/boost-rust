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
//! - [x] Digamma
//!   - [`digamma`]
//! - [ ] Trigamma
//! - [ ] Polygamma
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
//! - [ ] Binomial Coefficients
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
//! - [x] Error Function *erf* and and complement *erfc*
//!   - [`erf`]
//!   - [`erfc`]
//! - [x] Error Function Inverses
//!   - [`erf_inv`]
//!   - [`erfc_inv`]
//!
//! ### Polynomials
//!
//! - [x] Legendre (and associated) polynomials
//!   - [`legendre_p`]
//!   - [`legendre_p_prime`]
//!   - [`legendre_p_zeros`]
//!   - [`legendre_p_assoc`]
//!   - [`legendre_q`]
//! - [ ] Legendre-Stieltjes Polynomials
//! - [ ] Laguerre (and Associated) Polynomials
//! - [ ] Hermite Polynomials
//! - [ ] Chebyshev Polynomials
//! - [ ] Spherical Harmonics
//! - [ ] Cardinal B-splines
//! - [ ] Gegenbauer Polynomials
//! - [x] Jacobi Polynomials
//!   - [`jacobi`]
//!   - [`jacobi_derivative`]
//!   - ~`jacobi_prime`~
//!   - ~`jacobi_double_prime`~
//!
//! ### Bessel Functions
//! - [x] Bessel Functions of the First and Second Kinds
//!   - [`cyl_bessel_j`]
//!   - [`cyl_neumann`]
//! - [x] Modified Bessel Functions of the First and Second Kinds
//!   - [`cyl_bessel_i`]
//!   - [`cyl_bessel_k`]
//! - [x] Spherical Bessel Functions of the First and Second Kinds
//!   - [`sph_bessel`]
//!   - [`sph_neumann`]
//! - [ ] Derivatives of the Bessel Functions
//!
//! <h4>Hankel Functions</h4>
//!
//! - [ ] Cyclic Hankel Functions
//! - [ ] Spherical Hankel Functions
//!
//! <h4>Airy Functions</h4>
//!
//! - [ ] Airy *Ai* Function
//! - [ ] Airy *Bi* Function
//! - [ ] Airy *Ai'* Function
//! - [ ] Airy *Bi'* Function
//! - [ ] Finding Zeros of Airy Functions
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
//! <h4>Lambert W Function</h4>
//!
//! - [ ] `lambert_w0`
//! - [ ] `lambert_wm1`
//! - [ ] `lambert_w0_prime`
//! - [ ] `lambert_wm1_prime`
//!
//! ### Zeta Functions
//!
//! - [x] Riemann zeta function
//!   - [`zeta`]
//!
//! <h4>Exponential Integrals</h4>
//!
//! - [ ] Exponential Integral *En*
//! - [ ] Exponential Integral *Ei*
//!
//! <h4>Hypergeometric Functions</h4>
//!
//! - [ ] Hypergeometric *<sub>1</sub>F<sub>0</sub>*
//! - [ ] Hypergeometric *<sub>0</sub>F<sub>1</sub>*
//! - [ ] Hypergeometric *<sub>2</sub>F<sub>0</sub>*
//! - [ ] Hypergeometric *<sub>1</sub>F<sub>1</sub>*
//! - [ ] Hypergeometric *<sub>p</sub>F<sub>q</sub>*
//!
//! <h4>Basic Functions</h4>
//!
//! - [ ] `sin_pi`
//! - [ ] `cos_pi`
//! - [ ] `log1p`
//! - [ ] `expm1`
//! - [ ] `cbrt`
//! - [ ] `sqrt1pm1`
//! - [ ] `powm1`
//! - [ ] `hypot`
//! - [ ] Reciprocal square root
//! - [ ] `logaddexp` and `logsumexp`
//!
//! <h4>Sinus Cardinal and Hyperbolic Sinus Cardinal Functions</h4>
//!
//! - [ ] `sinc_pi`
//! - [ ] `sinhc_pi`
//!
//! <h4>Inverse Hyperbolic Functions</h4>
//!
//! - [ ] `acosh`
//! - [ ] `asinh`
//! - [ ] `atanh`
//!
//! <h4>Owen's T Function</h4>
//!
//! - [ ] `owens_t`
//!

mod special_functions;

pub use special_functions::bessel::*;
pub use special_functions::beta::*;
pub use special_functions::digamma::*;
pub use special_functions::erf::*;
pub use special_functions::factorials::*;
pub use special_functions::gamma::*;
pub use special_functions::jacobi::*;
pub use special_functions::legendre::*;
pub use special_functions::prime::*;
pub use special_functions::zeta::*;
