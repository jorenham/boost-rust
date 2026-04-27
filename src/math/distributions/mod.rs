//! Statistical distribution bindings for Boost Math.
//!
//! <https://boost.org/doc/libs/latest/libs/math/doc/html/dist.html>
//!
//! Each distribution is represented as a struct that implements the [`Distribution`] trait,
//! providing access to common statistical functions like [`pdf`](Distribution::pdf),
//! [`cdf`](Distribution::cdf), [`quantile`](Distribution::quantile),
//! [`mean`](Distribution::mean), [`variance`](Distribution::variance), etc.
//!
//! # Example
//!
//! ```
//! use boost::math::distributions::{Distribution, Normal};
//!
//! let normal = Normal::new(0.0, 1.0); // mean=0, std_dev=1
//! let density = normal.pdf(1.5);
//! let cumulative = normal.cdf(1.5);
//! let q = normal.quantile(0.95);
//! ```
//!
//! ## Continuous Distributions
//!
//! | Distribution | Parameters |
//! | ------------ | ---------- |
//! | [`Arcsine`] | `a`, `b` |
//! | [`BetaDist`] | `alpha`, `beta` |
//! | [`Cauchy`] | `location`, `scale` |
//! | [`ChiSquared`] | `df` |
//! | [`Exponential`] | `lambda` |
//! | [`ExtremeValue`] | `location`, `scale` |
//! | [`FisherF`] | `df1`, `df2` |
//! | [`GammaDist`] | `shape`, `scale` |
//! | [`InverseChiSquared`] | `df` |
//! | [`InverseGamma`] | `shape`, `scale` |
//! | [`InverseGaussian`] | `mean`, `scale` |
//! | [`KolmogorovSmirnov`] | `n` |
//! | [`Laplace`] | `location`, `scale` |
//! | [`Logistic`] | `location`, `scale` |
//! | [`Lognormal`] | `location`, `scale` |
//! | [`Normal`] | `location`, `scale` |
//! | [`Pareto`] | `scale`, `shape` |
//! | [`Rayleigh`] | `sigma` |
//! | [`SkewNormal`] | `location`, `scale`, `shape` |
//! | [`StudentsT`] | `df` |
//! | [`Triangular`] | `lower`, `mode`, `upper` |
//! | [`Uniform`] | `lower`, `upper` |
//! | [`Weibull`] | `shape`, `scale` |
//!
//! ## Discrete Distributions
//!
//! | Distribution | Parameters |
//! | ------------ | ---------- |
//! | [`Bernoulli`] | `p` |
//! | [`Binomial`] | `n`, `p` |
//! | [`Geometric`] | `p` |
//! | [`Hypergeometric`] | `r`, `n`, `total` |
//! | [`NegativeBinomial`] | `successes`, `p` |
//! | [`Poisson`] | `mean` |
//!
//! ## Non-Central Distributions
//!
//! | Distribution | Parameters |
//! | ------------ | ---------- |
//! | [`NonCentralBeta`] | `alpha`, `beta`, `lambda` |
//! | [`NonCentralChiSquared`] | `df`, `lambda` |
//! | [`NonCentralF`] | `df1`, `df2`, `lambda` |
//! | [`NonCentralT`] | `df`, `delta` |

/// Common interface for statistical distributions.
///
/// All distributions in the Boost Math library implement this trait, providing
/// access to standard statistical functions.
pub trait Distribution {
    /// Probability density function (PDF) for continuous distributions,
    /// or probability mass function (PMF) for discrete distributions.
    fn pdf(&self, x: f64) -> f64;

    /// Cumulative distribution function.
    fn cdf(&self, x: f64) -> f64;

    /// Survival function (complement of the CDF).
    ///
    /// Computes `1 - cdf(x)` but with greater accuracy than computing
    /// `1.0 - self.cdf(x)` directly, especially when the result is close to zero.
    ///
    /// This calls Boost's `cdf(complement(dist, x))` internally.
    fn sf(&self, x: f64) -> f64;

    /// Quantile function (inverse CDF).
    ///
    /// Returns the value `x` such that `cdf(x) == p`.
    fn quantile(&self, p: f64) -> f64;

    /// Inverse survival function.
    ///
    /// Returns the value `x` such that `sf(x) == q`, i.e. `cdf(x) == 1 - q`.
    /// More accurate than `quantile(1.0 - q)` when `q` is small.
    ///
    /// This calls Boost's `quantile(complement(dist, q))` internally.
    fn isf(&self, q: f64) -> f64;

    /// Mean (expected value) of the distribution.
    fn mean(&self) -> f64;

    /// Variance of the distribution.
    fn variance(&self) -> f64;

    /// Standard deviation of the distribution.
    fn standard_deviation(&self) -> f64;

    /// Skewness of the distribution.
    fn skewness(&self) -> f64;

    /// Kurtosis of the distribution.
    fn kurtosis(&self) -> f64;

    /// Excess kurtosis of the distribution (kurtosis - 3).
    fn kurtosis_excess(&self) -> f64;

    /// Median of the distribution.
    fn median(&self) -> f64;

    /// Mode of the distribution.
    fn mode(&self) -> f64;
}

/// Macro for defining distribution structs and their [`Distribution`] trait
/// implementations.
///
/// Each field defaults to `f64`, but a per-field type may be specified
/// (e.g. `n: u32`) for parameters that are conceptually integer-valued. The
/// FFI shims still take `f64`, so values are cast at the call site.
macro_rules! define_distribution {
    (
        $(#[$meta:meta])*
        $Name:ident { $($(#[$field_meta:meta])* $field:ident $(: $ty:ty)?),+ }
        ffi: $pdf:ident, $cdf:ident, $sf:ident, $quantile:ident, $isf:ident,
             $mean:ident, $variance:ident, $std_dev:ident,
             $skewness:ident, $kurtosis:ident, $kurtosis_excess:ident,
             $median:ident, $mode:ident
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $Name {
            $(
                $(#[$field_meta])*
                pub $field: define_distribution!(@ty $($ty)?),
            )+
        }

        impl $Name {
            /// Creates a new distribution instance.
            pub fn new($($field: define_distribution!(@ty $($ty)?)),+) -> Self {
                Self { $($field),+ }
            }
        }

        impl Distribution for $Name {
            fn pdf(&self, x: f64) -> f64 {
                unsafe { ffi::$pdf($(self.$field as f64),+, x) }
            }
            fn cdf(&self, x: f64) -> f64 {
                unsafe { ffi::$cdf($(self.$field as f64),+, x) }
            }
            fn sf(&self, x: f64) -> f64 {
                unsafe { ffi::$sf($(self.$field as f64),+, x) }
            }
            fn quantile(&self, p: f64) -> f64 {
                unsafe { ffi::$quantile($(self.$field as f64),+, p) }
            }
            fn isf(&self, q: f64) -> f64 {
                unsafe { ffi::$isf($(self.$field as f64),+, q) }
            }
            fn mean(&self) -> f64 {
                unsafe { ffi::$mean($(self.$field as f64),+) }
            }
            fn variance(&self) -> f64 {
                unsafe { ffi::$variance($(self.$field as f64),+) }
            }
            fn standard_deviation(&self) -> f64 {
                unsafe { ffi::$std_dev($(self.$field as f64),+) }
            }
            fn skewness(&self) -> f64 {
                unsafe { ffi::$skewness($(self.$field as f64),+) }
            }
            fn kurtosis(&self) -> f64 {
                unsafe { ffi::$kurtosis($(self.$field as f64),+) }
            }
            fn kurtosis_excess(&self) -> f64 {
                unsafe { ffi::$kurtosis_excess($(self.$field as f64),+) }
            }
            fn median(&self) -> f64 {
                unsafe { ffi::$median($(self.$field as f64),+) }
            }
            fn mode(&self) -> f64 {
                unsafe { ffi::$mode($(self.$field as f64),+) }
            }
        }
    };
    (@ty) => { f64 };
    (@ty $ty:ty) => { $ty };
}

mod continuous;
mod discrete;
mod non_central;

pub use continuous::*;
pub use discrete::*;
pub use non_central::*;
