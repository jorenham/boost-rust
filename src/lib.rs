//! # Boost Math Rust Bindings
//!
//! Minimalist Rust bindings for the Boost Math library, providing access to
//! high-quality implementations of mathematical special functions.
//!
//! ## Features
//!
//! - Gamma functions (tgamma, lgamma, digamma, gamma_p, gamma_q)
//! - Beta functions
//! - Error functions (erf, erfc)
//! - Bessel functions (cylindrical and spherical)
//! - Mathematical constants
//! - Utility functions for floating-point classification
//!
//! ## Example
//!
//! ```rust
//! // Mathematical constants
//! let pi = boost::math::constants::pi();
//! let e = boost::math::constants::e();
//!
//! // Gamma functions
//! let gamma_result = boost::math::tgamma(1.5);
//! let log_gamma_result = boost::math::lgamma(10.0);
//!
//! // Error functions
//! let erf_result = boost::math::erf(1.0);
//!
//! // Bessel functions
//! let bessel_j0 = boost::math::cyl_bessel_j(0.0, 1.0);
//! ```

#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
#[macro_use]
extern crate approx;

/// Bindings for Boost Math functions
pub mod math;
