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
//! | [`BetaDist`] | `alpha`, `beta` |
//! | [`NonCentralBeta`] | `alpha`, `beta`, `lambda` |
//! | [`NonCentralChiSquared`] | `df`, `lambda` |
//! | [`NonCentralF`] | `df1`, `df2`, `lambda` |
//! | [`NonCentralT`] | `df`, `delta` |

use crate::ffi;
use core::ffi::c_uint;

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

/// Macro for defining distribution structs with all-f64 parameters and their
/// [`Distribution`] trait implementations.
macro_rules! define_distribution {
    (
        $(#[$meta:meta])*
        $Name:ident { $($(#[$field_meta:meta])* $field:ident),+ }
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
                pub $field: f64,
            )+
        }

        impl $Name {
            /// Creates a new distribution instance.
            pub fn new($($field: f64),+) -> Self {
                Self { $($field),+ }
            }
        }

        impl Distribution for $Name {
            fn pdf(&self, x: f64) -> f64 {
                unsafe { ffi::$pdf($(self.$field),+, x) }
            }
            fn cdf(&self, x: f64) -> f64 {
                unsafe { ffi::$cdf($(self.$field),+, x) }
            }
            fn sf(&self, x: f64) -> f64 {
                unsafe { ffi::$sf($(self.$field),+, x) }
            }
            fn quantile(&self, p: f64) -> f64 {
                unsafe { ffi::$quantile($(self.$field),+, p) }
            }
            fn isf(&self, q: f64) -> f64 {
                unsafe { ffi::$isf($(self.$field),+, q) }
            }
            fn mean(&self) -> f64 {
                unsafe { ffi::$mean($(self.$field),+) }
            }
            fn variance(&self) -> f64 {
                unsafe { ffi::$variance($(self.$field),+) }
            }
            fn standard_deviation(&self) -> f64 {
                unsafe { ffi::$std_dev($(self.$field),+) }
            }
            fn skewness(&self) -> f64 {
                unsafe { ffi::$skewness($(self.$field),+) }
            }
            fn kurtosis(&self) -> f64 {
                unsafe { ffi::$kurtosis($(self.$field),+) }
            }
            fn kurtosis_excess(&self) -> f64 {
                unsafe { ffi::$kurtosis_excess($(self.$field),+) }
            }
            fn median(&self) -> f64 {
                unsafe { ffi::$median($(self.$field),+) }
            }
            fn mode(&self) -> f64 {
                unsafe { ffi::$mode($(self.$field),+) }
            }
        }
    };
}

// 1-parameter distributions

define_distribution! {
    /// Bernoulli distribution with success probability `p`.
    ///
    /// Corresponds to `boost::math::bernoulli_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/bernoulli_dist.html>
    Bernoulli { /// Success probability (0 <= p <= 1).
                p }
    ffi: math_dist_bernoulli_pdf, math_dist_bernoulli_cdf, math_dist_bernoulli_cdf_c, math_dist_bernoulli_quantile, math_dist_bernoulli_quantile_c,
         math_dist_bernoulli_mean, math_dist_bernoulli_variance, math_dist_bernoulli_std_dev,
         math_dist_bernoulli_skewness, math_dist_bernoulli_kurtosis, math_dist_bernoulli_kurtosis_excess,
         math_dist_bernoulli_median, math_dist_bernoulli_mode
}

define_distribution! {
    /// Chi-squared distribution with `df` degrees of freedom.
    ///
    /// Corresponds to `boost::math::chi_squared_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/chi_squared_dist.html>
    ChiSquared { /// Degrees of freedom.
                 df }
    ffi: math_dist_chi_squared_pdf, math_dist_chi_squared_cdf, math_dist_chi_squared_cdf_c, math_dist_chi_squared_quantile, math_dist_chi_squared_quantile_c,
         math_dist_chi_squared_mean, math_dist_chi_squared_variance, math_dist_chi_squared_std_dev,
         math_dist_chi_squared_skewness, math_dist_chi_squared_kurtosis, math_dist_chi_squared_kurtosis_excess,
         math_dist_chi_squared_median, math_dist_chi_squared_mode
}

define_distribution! {
    /// Exponential distribution with rate parameter `lambda`.
    ///
    /// Corresponds to `boost::math::exponential_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/exp_dist.html>
    Exponential { /// Rate parameter (lambda > 0).
                  lambda }
    ffi: math_dist_exponential_pdf, math_dist_exponential_cdf, math_dist_exponential_cdf_c, math_dist_exponential_quantile, math_dist_exponential_quantile_c,
         math_dist_exponential_mean, math_dist_exponential_variance, math_dist_exponential_std_dev,
         math_dist_exponential_skewness, math_dist_exponential_kurtosis, math_dist_exponential_kurtosis_excess,
         math_dist_exponential_median, math_dist_exponential_mode
}

define_distribution! {
    /// Geometric distribution with success probability `p`.
    ///
    /// Corresponds to `boost::math::geometric_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/geometric_dist.html>
    Geometric { /// Success probability (0 < p <= 1).
                p }
    ffi: math_dist_geometric_pdf, math_dist_geometric_cdf, math_dist_geometric_cdf_c, math_dist_geometric_quantile, math_dist_geometric_quantile_c,
         math_dist_geometric_mean, math_dist_geometric_variance, math_dist_geometric_std_dev,
         math_dist_geometric_skewness, math_dist_geometric_kurtosis, math_dist_geometric_kurtosis_excess,
         math_dist_geometric_median, math_dist_geometric_mode
}

define_distribution! {
    /// Inverse chi-squared distribution with `df` degrees of freedom.
    ///
    /// Corresponds to `boost::math::inverse_chi_squared_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/inverse_chi_squared_dist.html>
    InverseChiSquared { /// Degrees of freedom.
                        df }
    ffi: math_dist_inverse_chi_squared_pdf, math_dist_inverse_chi_squared_cdf, math_dist_inverse_chi_squared_cdf_c, math_dist_inverse_chi_squared_quantile, math_dist_inverse_chi_squared_quantile_c,
         math_dist_inverse_chi_squared_mean, math_dist_inverse_chi_squared_variance, math_dist_inverse_chi_squared_std_dev,
         math_dist_inverse_chi_squared_skewness, math_dist_inverse_chi_squared_kurtosis, math_dist_inverse_chi_squared_kurtosis_excess,
         math_dist_inverse_chi_squared_median, math_dist_inverse_chi_squared_mode
}

define_distribution! {
    /// Kolmogorov-Smirnov distribution with sample size `n`.
    ///
    /// Corresponds to `boost::math::kolmogorov_smirnov_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/kolmogorov_smirnov_dist.html>
    KolmogorovSmirnov { /// Sample size.
                        n }
    ffi: math_dist_kolmogorov_smirnov_pdf, math_dist_kolmogorov_smirnov_cdf, math_dist_kolmogorov_smirnov_cdf_c, math_dist_kolmogorov_smirnov_quantile, math_dist_kolmogorov_smirnov_quantile_c,
         math_dist_kolmogorov_smirnov_mean, math_dist_kolmogorov_smirnov_variance, math_dist_kolmogorov_smirnov_std_dev,
         math_dist_kolmogorov_smirnov_skewness, math_dist_kolmogorov_smirnov_kurtosis, math_dist_kolmogorov_smirnov_kurtosis_excess,
         math_dist_kolmogorov_smirnov_median, math_dist_kolmogorov_smirnov_mode
}

define_distribution! {
    /// Poisson distribution with given `mean`.
    ///
    /// Corresponds to `boost::math::poisson_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/poisson_dist.html>
    Poisson { /// Mean (expected value, lambda > 0).
              mean }
    ffi: math_dist_poisson_pdf, math_dist_poisson_cdf, math_dist_poisson_cdf_c, math_dist_poisson_quantile, math_dist_poisson_quantile_c,
         math_dist_poisson_mean, math_dist_poisson_variance, math_dist_poisson_std_dev,
         math_dist_poisson_skewness, math_dist_poisson_kurtosis, math_dist_poisson_kurtosis_excess,
         math_dist_poisson_median, math_dist_poisson_mode
}

define_distribution! {
    /// Rayleigh distribution with scale parameter `sigma`.
    ///
    /// Corresponds to `boost::math::rayleigh_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/rayleigh.html>
    Rayleigh { /// Scale parameter (sigma > 0).
               sigma }
    ffi: math_dist_rayleigh_pdf, math_dist_rayleigh_cdf, math_dist_rayleigh_cdf_c, math_dist_rayleigh_quantile, math_dist_rayleigh_quantile_c,
         math_dist_rayleigh_mean, math_dist_rayleigh_variance, math_dist_rayleigh_std_dev,
         math_dist_rayleigh_skewness, math_dist_rayleigh_kurtosis, math_dist_rayleigh_kurtosis_excess,
         math_dist_rayleigh_median, math_dist_rayleigh_mode
}

define_distribution! {
    /// Student's t-distribution with `df` degrees of freedom.
    ///
    /// Corresponds to `boost::math::students_t_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/students_t_dist.html>
    StudentsT { /// Degrees of freedom.
                df }
    ffi: math_dist_students_t_pdf, math_dist_students_t_cdf, math_dist_students_t_cdf_c, math_dist_students_t_quantile, math_dist_students_t_quantile_c,
         math_dist_students_t_mean, math_dist_students_t_variance, math_dist_students_t_std_dev,
         math_dist_students_t_skewness, math_dist_students_t_kurtosis, math_dist_students_t_kurtosis_excess,
         math_dist_students_t_median, math_dist_students_t_mode
}

// 2-parameter distributions

define_distribution! {
    /// Arcsine distribution on the interval `[a, b]`.
    ///
    /// Corresponds to `boost::math::arcsine_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/arcsine_dist.html>
    Arcsine { /// Lower bound.
              a,
              /// Upper bound.
              b }
    ffi: math_dist_arcsine_pdf, math_dist_arcsine_cdf, math_dist_arcsine_cdf_c, math_dist_arcsine_quantile, math_dist_arcsine_quantile_c,
         math_dist_arcsine_mean, math_dist_arcsine_variance, math_dist_arcsine_std_dev,
         math_dist_arcsine_skewness, math_dist_arcsine_kurtosis, math_dist_arcsine_kurtosis_excess,
         math_dist_arcsine_median, math_dist_arcsine_mode
}

define_distribution! {
    /// Beta distribution with shape parameters `alpha` and `beta`.
    ///
    /// Corresponds to `boost::math::beta_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/beta_dist.html>
    BetaDist { /// First shape parameter (alpha > 0).
               alpha,
               /// Second shape parameter (beta > 0).
               beta }
    ffi: math_dist_beta_pdf, math_dist_beta_cdf, math_dist_beta_cdf_c, math_dist_beta_quantile, math_dist_beta_quantile_c,
         math_dist_beta_mean, math_dist_beta_variance, math_dist_beta_std_dev,
         math_dist_beta_skewness, math_dist_beta_kurtosis, math_dist_beta_kurtosis_excess,
         math_dist_beta_median, math_dist_beta_mode
}

define_distribution! {
    /// Binomial distribution with `n` trials and success probability `p`.
    ///
    /// Corresponds to `boost::math::binomial_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/binomial_dist.html>
    Binomial { /// Number of trials.
               n,
               /// Success probability (0 <= p <= 1).
               p }
    ffi: math_dist_binomial_pdf, math_dist_binomial_cdf, math_dist_binomial_cdf_c, math_dist_binomial_quantile, math_dist_binomial_quantile_c,
         math_dist_binomial_mean, math_dist_binomial_variance, math_dist_binomial_std_dev,
         math_dist_binomial_skewness, math_dist_binomial_kurtosis, math_dist_binomial_kurtosis_excess,
         math_dist_binomial_median, math_dist_binomial_mode
}

define_distribution! {
    /// Cauchy (Lorentz) distribution with `location` and `scale` parameters.
    ///
    /// Corresponds to `boost::math::cauchy_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/cauchy_dist.html>
    Cauchy { /// Location parameter.
             location,
             /// Scale parameter (scale > 0).
             scale }
    ffi: math_dist_cauchy_pdf, math_dist_cauchy_cdf, math_dist_cauchy_cdf_c, math_dist_cauchy_quantile, math_dist_cauchy_quantile_c,
         math_dist_cauchy_mean, math_dist_cauchy_variance, math_dist_cauchy_std_dev,
         math_dist_cauchy_skewness, math_dist_cauchy_kurtosis, math_dist_cauchy_kurtosis_excess,
         math_dist_cauchy_median, math_dist_cauchy_mode
}

define_distribution! {
    /// Extreme value (Gumbel type I) distribution with `location` and `scale` parameters.
    ///
    /// Corresponds to `boost::math::extreme_value_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/extreme_dist.html>
    ExtremeValue { /// Location parameter.
                   location,
                   /// Scale parameter (scale > 0).
                   scale }
    ffi: math_dist_extreme_value_pdf, math_dist_extreme_value_cdf, math_dist_extreme_value_cdf_c, math_dist_extreme_value_quantile, math_dist_extreme_value_quantile_c,
         math_dist_extreme_value_mean, math_dist_extreme_value_variance, math_dist_extreme_value_std_dev,
         math_dist_extreme_value_skewness, math_dist_extreme_value_kurtosis, math_dist_extreme_value_kurtosis_excess,
         math_dist_extreme_value_median, math_dist_extreme_value_mode
}

define_distribution! {
    /// Fisher F-distribution with `df1` and `df2` degrees of freedom.
    ///
    /// Corresponds to `boost::math::fisher_f_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/f_dist.html>
    FisherF { /// First degrees of freedom.
              df1,
              /// Second degrees of freedom.
              df2 }
    ffi: math_dist_fisher_f_pdf, math_dist_fisher_f_cdf, math_dist_fisher_f_cdf_c, math_dist_fisher_f_quantile, math_dist_fisher_f_quantile_c,
         math_dist_fisher_f_mean, math_dist_fisher_f_variance, math_dist_fisher_f_std_dev,
         math_dist_fisher_f_skewness, math_dist_fisher_f_kurtosis, math_dist_fisher_f_kurtosis_excess,
         math_dist_fisher_f_median, math_dist_fisher_f_mode
}

define_distribution! {
    /// Gamma distribution with `shape` and `scale` parameters.
    ///
    /// Named `GammaDist` to avoid conflict with the gamma special function.
    ///
    /// Corresponds to `boost::math::gamma_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/gamma_dist.html>
    GammaDist { /// Shape parameter (shape > 0).
                shape,
                /// Scale parameter (scale > 0).
                scale }
    ffi: math_dist_gamma_pdf, math_dist_gamma_cdf, math_dist_gamma_cdf_c, math_dist_gamma_quantile, math_dist_gamma_quantile_c,
         math_dist_gamma_mean, math_dist_gamma_variance, math_dist_gamma_std_dev,
         math_dist_gamma_skewness, math_dist_gamma_kurtosis, math_dist_gamma_kurtosis_excess,
         math_dist_gamma_median, math_dist_gamma_mode
}

define_distribution! {
    /// Inverse gamma distribution with `shape` and `scale` parameters.
    ///
    /// Corresponds to `boost::math::inverse_gamma_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/inverse_gamma_dist.html>
    InverseGamma { /// Shape parameter (shape > 0).
                   shape,
                   /// Scale parameter (scale > 0).
                   scale }
    ffi: math_dist_inverse_gamma_pdf, math_dist_inverse_gamma_cdf, math_dist_inverse_gamma_cdf_c, math_dist_inverse_gamma_quantile, math_dist_inverse_gamma_quantile_c,
         math_dist_inverse_gamma_mean, math_dist_inverse_gamma_variance, math_dist_inverse_gamma_std_dev,
         math_dist_inverse_gamma_skewness, math_dist_inverse_gamma_kurtosis, math_dist_inverse_gamma_kurtosis_excess,
         math_dist_inverse_gamma_median, math_dist_inverse_gamma_mode
}

define_distribution! {
    /// Inverse Gaussian (Wald) distribution with `mean` and `scale` parameters.
    ///
    /// Corresponds to `boost::math::inverse_gaussian_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/inverse_gaussian_dist.html>
    InverseGaussian { /// Mean parameter (mean > 0).
                      mean,
                      /// Scale (shape) parameter (scale > 0).
                      scale }
    ffi: math_dist_inverse_gaussian_pdf, math_dist_inverse_gaussian_cdf, math_dist_inverse_gaussian_cdf_c, math_dist_inverse_gaussian_quantile, math_dist_inverse_gaussian_quantile_c,
         math_dist_inverse_gaussian_mean, math_dist_inverse_gaussian_variance, math_dist_inverse_gaussian_std_dev,
         math_dist_inverse_gaussian_skewness, math_dist_inverse_gaussian_kurtosis, math_dist_inverse_gaussian_kurtosis_excess,
         math_dist_inverse_gaussian_median, math_dist_inverse_gaussian_mode
}

define_distribution! {
    /// Laplace (double exponential) distribution with `location` and `scale` parameters.
    ///
    /// Corresponds to `boost::math::laplace_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/laplace_dist.html>
    Laplace { /// Location parameter.
              location,
              /// Scale parameter (scale > 0).
              scale }
    ffi: math_dist_laplace_pdf, math_dist_laplace_cdf, math_dist_laplace_cdf_c, math_dist_laplace_quantile, math_dist_laplace_quantile_c,
         math_dist_laplace_mean, math_dist_laplace_variance, math_dist_laplace_std_dev,
         math_dist_laplace_skewness, math_dist_laplace_kurtosis, math_dist_laplace_kurtosis_excess,
         math_dist_laplace_median, math_dist_laplace_mode
}

define_distribution! {
    /// Logistic distribution with `location` and `scale` parameters.
    ///
    /// Corresponds to `boost::math::logistic_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/logistic_dist.html>
    Logistic { /// Location parameter.
               location,
               /// Scale parameter (scale > 0).
               scale }
    ffi: math_dist_logistic_pdf, math_dist_logistic_cdf, math_dist_logistic_cdf_c, math_dist_logistic_quantile, math_dist_logistic_quantile_c,
         math_dist_logistic_mean, math_dist_logistic_variance, math_dist_logistic_std_dev,
         math_dist_logistic_skewness, math_dist_logistic_kurtosis, math_dist_logistic_kurtosis_excess,
         math_dist_logistic_median, math_dist_logistic_mode
}

define_distribution! {
    /// Log-normal distribution with `location` and `scale` parameters.
    ///
    /// Corresponds to `boost::math::lognormal_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/lognormal_dist.html>
    Lognormal { /// Location parameter (mean of log).
                location,
                /// Scale parameter (std dev of log, scale > 0).
                scale }
    ffi: math_dist_lognormal_pdf, math_dist_lognormal_cdf, math_dist_lognormal_cdf_c, math_dist_lognormal_quantile, math_dist_lognormal_quantile_c,
         math_dist_lognormal_mean, math_dist_lognormal_variance, math_dist_lognormal_std_dev,
         math_dist_lognormal_skewness, math_dist_lognormal_kurtosis, math_dist_lognormal_kurtosis_excess,
         math_dist_lognormal_median, math_dist_lognormal_mode
}

define_distribution! {
    /// Negative binomial distribution with `successes` and success probability `p`.
    ///
    /// Corresponds to `boost::math::negative_binomial_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/negative_binomial_dist.html>
    NegativeBinomial { /// Number of successes.
                       successes,
                       /// Success probability (0 < p <= 1).
                       p }
    ffi: math_dist_negative_binomial_pdf, math_dist_negative_binomial_cdf, math_dist_negative_binomial_cdf_c, math_dist_negative_binomial_quantile, math_dist_negative_binomial_quantile_c,
         math_dist_negative_binomial_mean, math_dist_negative_binomial_variance, math_dist_negative_binomial_std_dev,
         math_dist_negative_binomial_skewness, math_dist_negative_binomial_kurtosis, math_dist_negative_binomial_kurtosis_excess,
         math_dist_negative_binomial_median, math_dist_negative_binomial_mode
}

define_distribution! {
    /// Non-central chi-squared distribution with `df` degrees of freedom and
    /// non-centrality parameter `lambda`.
    ///
    /// Corresponds to `boost::math::non_central_chi_squared_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/nc_chi_squared_dist.html>
    NonCentralChiSquared { /// Degrees of freedom.
                           df,
                           /// Non-centrality parameter (lambda >= 0).
                           lambda }
    ffi: math_dist_non_central_chi_squared_pdf, math_dist_non_central_chi_squared_cdf, math_dist_non_central_chi_squared_cdf_c, math_dist_non_central_chi_squared_quantile, math_dist_non_central_chi_squared_quantile_c,
         math_dist_non_central_chi_squared_mean, math_dist_non_central_chi_squared_variance, math_dist_non_central_chi_squared_std_dev,
         math_dist_non_central_chi_squared_skewness, math_dist_non_central_chi_squared_kurtosis, math_dist_non_central_chi_squared_kurtosis_excess,
         math_dist_non_central_chi_squared_median, math_dist_non_central_chi_squared_mode
}

define_distribution! {
    /// Non-central t-distribution with `df` degrees of freedom and non-centrality
    /// parameter `delta`.
    ///
    /// Corresponds to `boost::math::non_central_t_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/nc_t_dist.html>
    NonCentralT { /// Degrees of freedom.
                  df,
                  /// Non-centrality parameter.
                  delta }
    ffi: math_dist_non_central_t_pdf, math_dist_non_central_t_cdf, math_dist_non_central_t_cdf_c, math_dist_non_central_t_quantile, math_dist_non_central_t_quantile_c,
         math_dist_non_central_t_mean, math_dist_non_central_t_variance, math_dist_non_central_t_std_dev,
         math_dist_non_central_t_skewness, math_dist_non_central_t_kurtosis, math_dist_non_central_t_kurtosis_excess,
         math_dist_non_central_t_median, math_dist_non_central_t_mode
}

define_distribution! {
    /// Normal (Gaussian) distribution with `location` (mean) and `scale` (standard deviation).
    ///
    /// Corresponds to `boost::math::normal_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/normal_dist.html>
    Normal { /// Location parameter (mean).
             location,
             /// Scale parameter (standard deviation, scale > 0).
             scale }
    ffi: math_dist_normal_pdf, math_dist_normal_cdf, math_dist_normal_cdf_c, math_dist_normal_quantile, math_dist_normal_quantile_c,
         math_dist_normal_mean, math_dist_normal_variance, math_dist_normal_std_dev,
         math_dist_normal_skewness, math_dist_normal_kurtosis, math_dist_normal_kurtosis_excess,
         math_dist_normal_median, math_dist_normal_mode
}

define_distribution! {
    /// Pareto distribution with `scale` and `shape` parameters.
    ///
    /// Corresponds to `boost::math::pareto_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/pareto.html>
    Pareto { /// Scale parameter (scale > 0).
             scale,
             /// Shape parameter (shape > 0).
             shape }
    ffi: math_dist_pareto_pdf, math_dist_pareto_cdf, math_dist_pareto_cdf_c, math_dist_pareto_quantile, math_dist_pareto_quantile_c,
         math_dist_pareto_mean, math_dist_pareto_variance, math_dist_pareto_std_dev,
         math_dist_pareto_skewness, math_dist_pareto_kurtosis, math_dist_pareto_kurtosis_excess,
         math_dist_pareto_median, math_dist_pareto_mode
}

define_distribution! {
    /// Continuous uniform distribution on the interval `[lower, upper]`.
    ///
    /// Corresponds to `boost::math::uniform_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/uniform_dist.html>
    Uniform { /// Lower bound.
              lower,
              /// Upper bound.
              upper }
    ffi: math_dist_uniform_pdf, math_dist_uniform_cdf, math_dist_uniform_cdf_c, math_dist_uniform_quantile, math_dist_uniform_quantile_c,
         math_dist_uniform_mean, math_dist_uniform_variance, math_dist_uniform_std_dev,
         math_dist_uniform_skewness, math_dist_uniform_kurtosis, math_dist_uniform_kurtosis_excess,
         math_dist_uniform_median, math_dist_uniform_mode
}

define_distribution! {
    /// Weibull distribution with `shape` and `scale` parameters.
    ///
    /// Corresponds to `boost::math::weibull_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/weibull_dist.html>
    Weibull { /// Shape parameter (shape > 0).
              shape,
              /// Scale parameter (scale > 0).
              scale }
    ffi: math_dist_weibull_pdf, math_dist_weibull_cdf, math_dist_weibull_cdf_c, math_dist_weibull_quantile, math_dist_weibull_quantile_c,
         math_dist_weibull_mean, math_dist_weibull_variance, math_dist_weibull_std_dev,
         math_dist_weibull_skewness, math_dist_weibull_kurtosis, math_dist_weibull_kurtosis_excess,
         math_dist_weibull_median, math_dist_weibull_mode
}

// 3-parameter distributions

define_distribution! {
    /// Non-central beta distribution with shape parameters `alpha`, `beta` and
    /// non-centrality parameter `lambda`.
    ///
    /// Corresponds to `boost::math::non_central_beta_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/nc_beta_dist.html>
    NonCentralBeta { /// First shape parameter (alpha > 0).
                     alpha,
                     /// Second shape parameter (beta > 0).
                     beta,
                     /// Non-centrality parameter (lambda >= 0).
                     lambda }
    ffi: math_dist_non_central_beta_pdf, math_dist_non_central_beta_cdf, math_dist_non_central_beta_cdf_c, math_dist_non_central_beta_quantile, math_dist_non_central_beta_quantile_c,
         math_dist_non_central_beta_mean, math_dist_non_central_beta_variance, math_dist_non_central_beta_std_dev,
         math_dist_non_central_beta_skewness, math_dist_non_central_beta_kurtosis, math_dist_non_central_beta_kurtosis_excess,
         math_dist_non_central_beta_median, math_dist_non_central_beta_mode
}

define_distribution! {
    /// Non-central F-distribution with `df1`, `df2` degrees of freedom and
    /// non-centrality parameter `lambda`.
    ///
    /// Corresponds to `boost::math::non_central_f_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/nc_f_dist.html>
    NonCentralF { /// First degrees of freedom.
                  df1,
                  /// Second degrees of freedom.
                  df2,
                  /// Non-centrality parameter (lambda >= 0).
                  lambda }
    ffi: math_dist_non_central_f_pdf, math_dist_non_central_f_cdf, math_dist_non_central_f_cdf_c, math_dist_non_central_f_quantile, math_dist_non_central_f_quantile_c,
         math_dist_non_central_f_mean, math_dist_non_central_f_variance, math_dist_non_central_f_std_dev,
         math_dist_non_central_f_skewness, math_dist_non_central_f_kurtosis, math_dist_non_central_f_kurtosis_excess,
         math_dist_non_central_f_median, math_dist_non_central_f_mode
}

define_distribution! {
    /// Skew normal distribution with `location`, `scale`, and `shape` parameters.
    ///
    /// Corresponds to `boost::math::skew_normal_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/skew_normal_dist.html>
    SkewNormal { /// Location parameter.
                 location,
                 /// Scale parameter (scale > 0).
                 scale,
                 /// Shape (skewness) parameter.
                 shape }
    ffi: math_dist_skew_normal_pdf, math_dist_skew_normal_cdf, math_dist_skew_normal_cdf_c, math_dist_skew_normal_quantile, math_dist_skew_normal_quantile_c,
         math_dist_skew_normal_mean, math_dist_skew_normal_variance, math_dist_skew_normal_std_dev,
         math_dist_skew_normal_skewness, math_dist_skew_normal_kurtosis, math_dist_skew_normal_kurtosis_excess,
         math_dist_skew_normal_median, math_dist_skew_normal_mode
}

define_distribution! {
    /// Triangular distribution on `[lower, upper]` with given `mode`.
    ///
    /// Corresponds to `boost::math::triangular_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/triangular_dist.html>
    Triangular { /// Lower bound.
                 lower,
                 /// Mode (peak).
                 mode,
                 /// Upper bound.
                 upper }
    ffi: math_dist_triangular_pdf, math_dist_triangular_cdf, math_dist_triangular_cdf_c, math_dist_triangular_quantile, math_dist_triangular_quantile_c,
         math_dist_triangular_mean, math_dist_triangular_variance, math_dist_triangular_std_dev,
         math_dist_triangular_skewness, math_dist_triangular_kurtosis, math_dist_triangular_kurtosis_excess,
         math_dist_triangular_median, math_dist_triangular_mode
}

// Hypergeometric distribution (unsigned parameters, special case)

/// Hypergeometric distribution with parameters `r`, `n`, and `total` (N).
///
/// Corresponds to `boost::math::hypergeometric_distribution` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/hypergeometric_dist.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hypergeometric {
    /// Number of defective items in the population.
    pub r: u32,
    /// Number of items sampled.
    pub n: u32,
    /// Total population size (N).
    pub total: u32,
}

impl Hypergeometric {
    /// Creates a new hypergeometric distribution.
    pub fn new(r: u32, n: u32, total: u32) -> Self {
        Self { r, n, total }
    }
}

impl Distribution for Hypergeometric {
    fn pdf(&self, x: f64) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_pdf(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
                x,
            )
        }
    }
    fn cdf(&self, x: f64) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_cdf(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
                x,
            )
        }
    }
    fn sf(&self, x: f64) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_cdf_c(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
                x,
            )
        }
    }
    fn quantile(&self, p: f64) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_quantile(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
                p,
            )
        }
    }
    fn isf(&self, q: f64) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_quantile_c(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
                q,
            )
        }
    }
    fn mean(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_mean(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
            )
        }
    }
    fn variance(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_variance(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
            )
        }
    }
    fn standard_deviation(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_std_dev(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
            )
        }
    }
    fn skewness(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_skewness(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
            )
        }
    }
    fn kurtosis(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_kurtosis(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
            )
        }
    }
    fn kurtosis_excess(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_kurtosis_excess(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
            )
        }
    }
    fn median(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_median(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
            )
        }
    }
    fn mode(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_mode(
                self.r as c_uint,
                self.n as c_uint,
                self.total as c_uint,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::{E, FRAC_1_PI, PI};

    const EPS: f64 = 1e-12;

    #[test]
    fn test_normal() {
        let d = Normal::new(0.0, 1.0);
        assert_relative_eq!(d.mean(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.variance(), 1.0, epsilon = EPS);
        assert_relative_eq!(d.skewness(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.kurtosis_excess(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.median(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.mode(), 0.0, epsilon = EPS);
        // pdf(0) = 1/sqrt(2*pi)
        assert_relative_eq!(d.pdf(0.0), 1.0 / (2.0 * PI).sqrt(), epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.cdf(1.0), 0.8413447460685429, epsilon = EPS);
        assert_relative_eq!(d.quantile(0.975), 1.959963984540054, epsilon = EPS);
        // sf(x) = 1 - cdf(x)
        assert_relative_eq!(d.sf(0.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.sf(1.0), 1.0 - 0.8413447460685429, epsilon = EPS);
        // isf is inverse of sf
        assert_relative_eq!(d.isf(0.025), 1.959963984540054, epsilon = EPS);
        assert_relative_eq!(d.isf(0.5), 0.0, epsilon = EPS);

        // Non-standard normal
        let d2 = Normal::new(5.0, 2.0);
        assert_relative_eq!(d2.mean(), 5.0, epsilon = EPS);
        assert_relative_eq!(d2.standard_deviation(), 2.0, epsilon = EPS);
    }

    #[test]
    fn test_students_t() {
        let d = StudentsT::new(10.0);
        assert_relative_eq!(d.mean(), 0.0, epsilon = EPS);
        // variance = df / (df - 2) = 10/8
        assert_relative_eq!(d.variance(), 1.25, epsilon = EPS);
        assert_relative_eq!(d.skewness(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.median(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.mode(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.sf(0.0), 0.5, epsilon = EPS);
        // kurtosis_excess = 6 / (df - 4)
        assert_relative_eq!(d.kurtosis_excess(), 1.0, epsilon = EPS);
    }

    #[test]
    fn test_chi_squared() {
        let d = ChiSquared::new(5.0);
        assert_relative_eq!(d.mean(), 5.0, epsilon = EPS);
        assert_relative_eq!(d.variance(), 10.0, epsilon = EPS);
        // skewness = sqrt(8/df)
        assert_relative_eq!(d.skewness(), (8.0 / 5.0_f64).sqrt(), epsilon = EPS);
        assert_relative_eq!(d.mode(), 3.0, epsilon = EPS);
        // kurtosis_excess = 12/df
        assert_relative_eq!(d.kurtosis_excess(), 2.4, epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.0, epsilon = EPS);
        assert_relative_eq!(d.sf(0.0), 1.0, epsilon = EPS);
    }

    #[test]
    fn test_exponential() {
        let d = Exponential::new(2.0);
        // mean = 1/lambda
        assert_relative_eq!(d.mean(), 0.5, epsilon = EPS);
        // variance = 1/lambda^2
        assert_relative_eq!(d.variance(), 0.25, epsilon = EPS);
        assert_relative_eq!(d.skewness(), 2.0, epsilon = EPS);
        assert_relative_eq!(d.kurtosis_excess(), 6.0, epsilon = EPS);
        assert_relative_eq!(d.mode(), 0.0, epsilon = EPS);
        // median = ln(2)/lambda
        assert_relative_eq!(d.median(), 2.0_f64.ln() / 2.0, epsilon = EPS);
        // cdf(x) = 1 - exp(-lambda*x)
        assert_relative_eq!(d.cdf(1.0), 1.0 - (-2.0_f64).exp(), epsilon = EPS);
        // sf(x) = exp(-lambda*x)
        assert_relative_eq!(d.sf(1.0), (-2.0_f64).exp(), epsilon = EPS);
        // pdf(x) = lambda * exp(-lambda*x)
        assert_relative_eq!(d.pdf(0.5), 2.0 * (-1.0_f64).exp(), epsilon = EPS);
        // isf(q) = -ln(q) / lambda
        assert_relative_eq!(d.isf(0.5), 2.0_f64.ln() / 2.0, epsilon = EPS);
    }

    #[test]
    fn test_gamma_dist() {
        let d = GammaDist::new(2.0, 3.0);
        // mean = shape * scale
        assert_relative_eq!(d.mean(), 6.0, epsilon = EPS);
        // variance = shape * scale^2
        assert_relative_eq!(d.variance(), 18.0, epsilon = EPS);
        // skewness = 2/sqrt(shape)
        assert_relative_eq!(d.skewness(), 2.0_f64.sqrt(), epsilon = EPS);
        // kurtosis_excess = 6/shape
        assert_relative_eq!(d.kurtosis_excess(), 3.0, epsilon = EPS);
        // mode = (shape-1)*scale
        assert_relative_eq!(d.mode(), 3.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.0, epsilon = EPS);
        assert_relative_eq!(d.sf(0.0), 1.0, epsilon = EPS);
    }

    #[test]
    fn test_beta_dist() {
        let d = BetaDist::new(2.0, 5.0);
        // mean = alpha / (alpha + beta) = 2/7
        assert_relative_eq!(d.mean(), 2.0 / 7.0, epsilon = EPS);
        // variance = alpha*beta / ((a+b)^2 * (a+b+1))
        assert_relative_eq!(d.variance(), 10.0 / 392.0, epsilon = EPS);
        // mode = (alpha-1)/(alpha+beta-2) = 1/5
        assert_relative_eq!(d.mode(), 0.2, epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(1.0), 1.0, epsilon = EPS);
        assert_relative_eq!(d.sf(0.0), 1.0, epsilon = EPS);
        assert_relative_eq!(d.sf(1.0), 0.0, epsilon = EPS);
        // skewness = 2*(b-a)*sqrt(a+b+1) / ((a+b+2)*sqrt(a*b))
        let (a, b): (f64, f64) = (2.0, 5.0);
        let expected_skew = 2.0 * (b - a) * (a + b + 1.0).sqrt() / ((a + b + 2.0) * (a * b).sqrt());
        assert_relative_eq!(d.skewness(), expected_skew, epsilon = EPS);
    }

    #[test]
    fn test_uniform() {
        let d = Uniform::new(2.0, 8.0);
        assert_relative_eq!(d.mean(), 5.0, epsilon = EPS);
        assert_relative_eq!(d.median(), 5.0, epsilon = EPS);
        // variance = (b-a)^2 / 12
        assert_relative_eq!(d.variance(), 3.0, epsilon = EPS);
        assert_relative_eq!(d.skewness(), 0.0, epsilon = EPS);
        // kurtosis_excess = -6/5
        assert_relative_eq!(d.kurtosis_excess(), -1.2, epsilon = EPS);
        // pdf = 1/(b-a) = 1/6
        assert_relative_eq!(d.pdf(5.0), 1.0 / 6.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(5.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.sf(5.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.quantile(0.25), 3.5, epsilon = EPS);
        assert_relative_eq!(d.quantile(0.75), 6.5, epsilon = EPS);
        assert_relative_eq!(d.isf(0.75), 3.5, epsilon = EPS);
        assert_relative_eq!(d.isf(0.25), 6.5, epsilon = EPS);
    }

    #[test]
    fn test_poisson() {
        let d = Poisson::new(4.0);
        assert_relative_eq!(d.mean(), 4.0, epsilon = EPS);
        assert_relative_eq!(d.variance(), 4.0, epsilon = EPS);
        // skewness = 1/sqrt(mean)
        assert_relative_eq!(d.skewness(), 0.5, epsilon = EPS);
        // kurtosis_excess = 1/mean
        assert_relative_eq!(d.kurtosis_excess(), 0.25, epsilon = EPS);
        assert_relative_eq!(d.mode(), 4.0, epsilon = EPS);
        // pmf(0) = e^(-4)
        assert_relative_eq!(d.pdf(0.0), (-4.0_f64).exp(), epsilon = EPS);
        // pmf(4) = e^(-4) * 4^4 / 4!
        assert_relative_eq!(d.pdf(4.0), (-4.0_f64).exp() * 256.0 / 24.0, epsilon = EPS);
        // sf + cdf = 1
        let cdf_3 = d.cdf(3.0);
        assert_relative_eq!(d.sf(3.0), 1.0 - cdf_3, epsilon = EPS);
    }

    #[test]
    fn test_binomial() {
        let d = Binomial::new(10.0, 0.3);
        assert_relative_eq!(d.mean(), 3.0, epsilon = EPS);
        assert_relative_eq!(d.variance(), 2.1, epsilon = EPS);
        // skewness = (1 - 2p) / sqrt(n*p*(1-p))
        assert_relative_eq!(d.skewness(), 0.4 / 2.1_f64.sqrt(), epsilon = EPS);
        assert_relative_eq!(d.mode(), 3.0, epsilon = EPS);
        // pmf(0) = (1-p)^n
        assert_relative_eq!(d.pdf(0.0), 0.7_f64.powi(10), epsilon = EPS);
        // pmf(n) = p^n
        assert_relative_eq!(d.pdf(10.0), 0.3_f64.powi(10), epsilon = EPS);
        // sf + cdf = 1
        let cdf_3 = d.cdf(3.0);
        assert_relative_eq!(d.sf(3.0), 1.0 - cdf_3, epsilon = EPS);
    }

    #[test]
    fn test_bernoulli() {
        let d = Bernoulli::new(0.7);
        assert_relative_eq!(d.mean(), 0.7, epsilon = EPS);
        // variance = p*(1-p)
        assert_relative_eq!(d.variance(), 0.21, epsilon = EPS);
        // skewness = (1 - 2p) / sqrt(p*(1-p))
        assert_relative_eq!(d.skewness(), -0.4 / 0.21_f64.sqrt(), epsilon = EPS);
        assert_relative_eq!(d.mode(), 1.0, epsilon = EPS);
        assert_relative_eq!(d.pdf(0.0), 0.3, epsilon = EPS);
        assert_relative_eq!(d.pdf(1.0), 0.7, epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.3, epsilon = EPS);
        assert_relative_eq!(d.cdf(1.0), 1.0, epsilon = EPS);
        assert_relative_eq!(d.sf(0.0), 0.7, epsilon = EPS);
        assert_relative_eq!(d.sf(1.0), 0.0, epsilon = EPS);
    }

    #[test]
    fn test_geometric() {
        let d = Geometric::new(0.25);
        // mean = (1-p)/p
        assert_relative_eq!(d.mean(), 3.0, epsilon = EPS);
        // variance = (1-p)/p^2
        assert_relative_eq!(d.variance(), 12.0, epsilon = EPS);
        assert_relative_eq!(d.mode(), 0.0, epsilon = EPS);
        // skewness = (2-p)/sqrt(1-p)
        assert_relative_eq!(d.skewness(), 1.75 / 0.75_f64.sqrt(), epsilon = EPS);
        // pmf(0) = p
        assert_relative_eq!(d.pdf(0.0), 0.25, epsilon = EPS);
        // pmf(1) = p*(1-p)
        assert_relative_eq!(d.pdf(1.0), 0.1875, epsilon = EPS);
        // sf + cdf = 1
        let cdf_2 = d.cdf(2.0);
        assert_relative_eq!(d.sf(2.0), 1.0 - cdf_2, epsilon = EPS);
    }

    #[test]
    fn test_cauchy() {
        let d = Cauchy::new(2.0, 3.0);
        // Cauchy has no mean/variance/skewness/kurtosis
        assert!(d.mean().is_nan());
        assert!(d.variance().is_nan());
        assert!(d.skewness().is_nan());
        assert!(d.kurtosis().is_nan());
        assert!(d.kurtosis_excess().is_nan());
        assert_relative_eq!(d.median(), 2.0, epsilon = EPS);
        assert_relative_eq!(d.mode(), 2.0, epsilon = EPS);
        // pdf(location) = 1/(pi*scale)
        assert_relative_eq!(d.pdf(2.0), 1.0 / (PI * 3.0), epsilon = EPS);
        assert_relative_eq!(d.cdf(2.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.sf(2.0), 0.5, epsilon = EPS);
        // cdf(x) = 0.5 + arctan((x-loc)/scale)/pi
        assert_relative_eq!(
            d.cdf(5.0),
            0.5 + ((5.0 - 2.0) / 3.0_f64).atan() * FRAC_1_PI,
            epsilon = EPS
        );
        assert_relative_eq!(d.quantile(0.75), 5.0, epsilon = EPS);
        assert_relative_eq!(d.isf(0.25), 5.0, epsilon = EPS);
    }

    #[test]
    fn test_weibull() {
        // shape=1, scale=2 => exponential(1/2)
        let d = Weibull::new(1.0, 2.0);
        assert_relative_eq!(d.mean(), 2.0, epsilon = EPS);
        assert_relative_eq!(d.mode(), 0.0, epsilon = EPS);
        // cdf(x) = 1 - exp(-(x/scale)^shape)
        assert_relative_eq!(d.cdf(2.0), 1.0 - (-1.0_f64).exp(), epsilon = EPS);
        // sf(x) = exp(-(x/scale)^shape)
        assert_relative_eq!(d.sf(2.0), (-1.0_f64).exp(), epsilon = EPS);
        // pdf(x) = (shape/scale) * (x/scale)^(shape-1) * exp(-(x/scale)^shape)
        assert_relative_eq!(d.pdf(1.0), 0.5 * (-0.5_f64).exp(), epsilon = EPS);
        // median = scale * (ln 2)^(1/shape)
        assert_relative_eq!(d.median(), 2.0 * 2.0_f64.ln(), epsilon = EPS);

        // shape=2, scale=1: variance = Gamma(2) - Gamma(1.5)^2 = 1 - pi/4
        let d2 = Weibull::new(2.0, 1.0);
        assert_relative_eq!(d2.variance(), 1.0 - PI / 4.0, epsilon = EPS);
    }

    #[test]
    fn test_lognormal() {
        let d = Lognormal::new(0.0, 1.0);
        // mean = exp(mu + sigma^2/2)
        assert_relative_eq!(d.mean(), (0.5_f64).exp(), epsilon = EPS);
        // variance = (exp(sigma^2) - 1) * exp(2*mu + sigma^2)
        assert_relative_eq!(d.variance(), (E - 1.0) * E, epsilon = EPS);
        // median = exp(mu) = 1
        assert_relative_eq!(d.median(), 1.0, epsilon = EPS);
        // mode = exp(mu - sigma^2) = exp(-1)
        assert_relative_eq!(d.mode(), (-1.0_f64).exp(), epsilon = EPS);
        assert_relative_eq!(d.cdf(1.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.sf(1.0), 0.5, epsilon = EPS);
    }

    #[test]
    fn test_fisher_f() {
        let d = FisherF::new(6.0, 10.0);
        // mean = d2/(d2-2)
        assert_relative_eq!(d.mean(), 1.25, epsilon = EPS);
        // variance = 2*d2^2*(d1+d2-2) / (d1*(d2-2)^2*(d2-4))
        assert_relative_eq!(d.variance(), 2800.0 / 2304.0, epsilon = EPS);
        // mode = ((d1-2)/d1) * (d2/(d2+2)) = 5/9
        assert_relative_eq!(d.mode(), 5.0 / 9.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.0, epsilon = EPS);
        assert_relative_eq!(d.sf(0.0), 1.0, epsilon = EPS);
    }

    #[test]
    fn test_negative_binomial() {
        let d = NegativeBinomial::new(5.0, 0.4);
        // mean = r*(1-p)/p
        assert_relative_eq!(d.mean(), 7.5, epsilon = EPS);
        // variance = r*(1-p)/p^2
        assert_relative_eq!(d.variance(), 18.75, epsilon = EPS);
        assert_relative_eq!(d.mode(), 5.0, epsilon = EPS);
        // skewness = (2-p)/sqrt(r*(1-p))
        assert_relative_eq!(d.skewness(), 1.6 / (5.0 * 0.6_f64).sqrt(), epsilon = EPS);
        // sf + cdf = 1
        let cdf_5 = d.cdf(5.0);
        assert_relative_eq!(d.sf(5.0), 1.0 - cdf_5, epsilon = EPS);
    }

    #[test]
    fn test_hypergeometric() {
        // r=30 defective, n=10 sampled, total=100
        let d = Hypergeometric::new(30, 10, 100);
        // mean = n*r/N
        assert_relative_eq!(d.mean(), 3.0, epsilon = EPS);
        // variance = n*r*(N-r)*(N-n) / (N^2*(N-1))
        assert_relative_eq!(d.variance(), 1890000.0 / 990000.0, epsilon = EPS);
        // CDF monotonicity
        assert!(d.cdf(2.0) < d.cdf(3.0));
        assert!(d.cdf(3.0) < d.cdf(4.0));
        // sf monotonicity (reverse of cdf)
        assert!(d.sf(2.0) > d.sf(3.0));
        assert!(d.sf(3.0) > d.sf(4.0));
    }

    #[test]
    fn test_rayleigh() {
        let d = Rayleigh::new(2.0);
        // mean = sigma * sqrt(pi/2)
        assert_relative_eq!(d.mean(), 2.0 * (PI / 2.0).sqrt(), epsilon = EPS);
        // variance = (4 - pi)/2 * sigma^2
        assert_relative_eq!(d.variance(), (4.0 - PI) / 2.0 * 4.0, epsilon = EPS);
        assert_relative_eq!(d.mode(), 2.0, epsilon = EPS);
        // median = sigma * sqrt(2*ln2)
        assert_relative_eq!(d.median(), 2.0 * (2.0 * 2.0_f64.ln()).sqrt(), epsilon = EPS);
        // pdf(x) = (x/sigma^2) * exp(-x^2/(2*sigma^2))
        assert_relative_eq!(d.pdf(2.0), 0.5 * (-0.5_f64).exp(), epsilon = EPS);
        // cdf(x) = 1 - exp(-x^2/(2*sigma^2))
        assert_relative_eq!(d.cdf(2.0), 1.0 - (-0.5_f64).exp(), epsilon = EPS);
        // sf(x) = exp(-x^2/(2*sigma^2))
        assert_relative_eq!(d.sf(2.0), (-0.5_f64).exp(), epsilon = EPS);
    }

    #[test]
    fn test_laplace() {
        let d = Laplace::new(3.0, 2.0);
        assert_relative_eq!(d.mean(), 3.0, epsilon = EPS);
        assert_relative_eq!(d.median(), 3.0, epsilon = EPS);
        assert_relative_eq!(d.mode(), 3.0, epsilon = EPS);
        // variance = 2*b^2
        assert_relative_eq!(d.variance(), 8.0, epsilon = EPS);
        assert_relative_eq!(d.skewness(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.kurtosis_excess(), 3.0, epsilon = EPS);
        // pdf(location) = 1/(2*b)
        assert_relative_eq!(d.pdf(3.0), 0.25, epsilon = EPS);
        // cdf(x) for x < mu = 0.5*exp((x-mu)/b)
        assert_relative_eq!(d.cdf(1.0), 0.5 * (-1.0_f64).exp(), epsilon = EPS);
        // sf(x) for x < mu = 1 - 0.5*exp((x-mu)/b)
        assert_relative_eq!(d.sf(1.0), 1.0 - 0.5 * (-1.0_f64).exp(), epsilon = EPS);
    }

    #[test]
    fn test_triangular() {
        let d = Triangular::new(0.0, 3.0, 6.0);
        // mean = (a+b+c)/3
        assert_relative_eq!(d.mean(), 3.0, epsilon = EPS);
        // variance = (a^2+b^2+c^2-ab-ac-bc)/18
        assert_relative_eq!(d.variance(), 1.5, epsilon = EPS);
        assert_relative_eq!(d.mode(), 3.0, epsilon = EPS);
        // pdf at mode = 2/(c-a)
        assert_relative_eq!(d.pdf(3.0), 1.0 / 3.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(3.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.cdf(6.0), 1.0, epsilon = EPS);
        assert_relative_eq!(d.sf(0.0), 1.0, epsilon = EPS);
        assert_relative_eq!(d.sf(3.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.sf(6.0), 0.0, epsilon = EPS);
    }

    #[test]
    fn test_extreme_value() {
        let d = ExtremeValue::new(0.0, 1.0);
        let euler = 0.5772156649015329;
        assert_relative_eq!(d.mean(), euler, epsilon = EPS);
        // variance = pi^2/6
        assert_relative_eq!(d.variance(), PI * PI / 6.0, epsilon = EPS);
        assert_relative_eq!(d.mode(), 0.0, epsilon = EPS);
        // skewness ≈ 1.13955
        assert_relative_eq!(d.skewness(), 1.1395470994046488, epsilon = EPS);
        // cdf(0) = exp(-1)
        assert_relative_eq!(d.cdf(0.0), (-1.0_f64).exp(), epsilon = EPS);
        // sf(0) = 1 - exp(-1)
        assert_relative_eq!(d.sf(0.0), 1.0 - (-1.0_f64).exp(), epsilon = EPS);
        // pdf(0) = exp(-1)
        assert_relative_eq!(d.pdf(0.0), (-1.0_f64).exp(), epsilon = EPS);
    }

    #[test]
    fn test_logistic() {
        let d = Logistic::new(2.0, 3.0);
        assert_relative_eq!(d.mean(), 2.0, epsilon = EPS);
        assert_relative_eq!(d.median(), 2.0, epsilon = EPS);
        assert_relative_eq!(d.mode(), 2.0, epsilon = EPS);
        assert_relative_eq!(d.skewness(), 0.0, epsilon = EPS);
        // variance = pi^2 * s^2 / 3
        assert_relative_eq!(d.variance(), PI * PI * 9.0 / 3.0, epsilon = EPS);
        assert_relative_eq!(d.kurtosis_excess(), 1.2, epsilon = EPS);
        // pdf(mu) = 1/(4*s)
        assert_relative_eq!(d.pdf(2.0), 1.0 / 12.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(2.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.sf(2.0), 0.5, epsilon = EPS);
    }

    #[test]
    fn test_pareto() {
        let d = Pareto::new(1.0, 3.0);
        // mean = alpha*x_m / (alpha-1)
        assert_relative_eq!(d.mean(), 1.5, epsilon = EPS);
        assert_relative_eq!(d.variance(), 0.75, epsilon = EPS);
        assert_relative_eq!(d.mode(), 1.0, epsilon = EPS);
        // median = x_m * 2^(1/alpha)
        assert_relative_eq!(d.median(), 2.0_f64.powf(1.0 / 3.0), epsilon = EPS);
        // pdf(2) = alpha / x^(alpha+1) = 3/16
        assert_relative_eq!(d.pdf(2.0), 3.0 / 16.0, epsilon = EPS);
        // cdf(2) = 1 - (1/2)^3 = 7/8
        assert_relative_eq!(d.cdf(2.0), 0.875, epsilon = EPS);
        // sf(2) = (1/2)^3 = 1/8
        assert_relative_eq!(d.sf(2.0), 0.125, epsilon = EPS);

        // alpha=5 for skewness check (requires alpha > 3)
        let d2 = Pareto::new(1.0, 5.0);
        let expected_skew = 2.0 * 6.0 / (2.0 * (5.0 / 3.0_f64).sqrt());
        assert_relative_eq!(d2.skewness(), expected_skew, epsilon = EPS);
    }

    #[test]
    fn test_inverse_gaussian() {
        let d = InverseGaussian::new(2.0, 3.0);
        assert_relative_eq!(d.mean(), 2.0, epsilon = EPS);
        // variance = mu^3 / lambda
        assert_relative_eq!(d.variance(), 8.0 / 3.0, epsilon = EPS);
        // skewness = 3*sqrt(mu/lambda)
        assert_relative_eq!(d.skewness(), 3.0 * (2.0 / 3.0_f64).sqrt(), epsilon = EPS);
        // kurtosis_excess = 15*mu/lambda
        assert_relative_eq!(d.kurtosis_excess(), 10.0, epsilon = EPS);
        // sf + cdf = 1
        let cdf_2 = d.cdf(2.0);
        assert_relative_eq!(d.sf(2.0), 1.0 - cdf_2, epsilon = EPS);
    }

    #[test]
    fn test_inverse_chi_squared() {
        let d = InverseChiSquared::new(6.0);
        // mean = 1/(df-2)
        assert_relative_eq!(d.mean(), 0.25, epsilon = EPS);
        // variance = 2 / ((df-2)^2 * (df-4))
        assert_relative_eq!(d.variance(), 1.0 / 16.0, epsilon = EPS);
        // mode = 1/(df+2)
        assert_relative_eq!(d.mode(), 0.125, epsilon = EPS);
        // sf + cdf = 1
        let cdf_02 = d.cdf(0.2);
        assert_relative_eq!(d.sf(0.2), 1.0 - cdf_02, epsilon = EPS);
    }

    #[test]
    fn test_inverse_gamma() {
        let d = InverseGamma::new(4.0, 2.0);
        // mean = beta/(alpha-1)
        assert_relative_eq!(d.mean(), 2.0 / 3.0, epsilon = EPS);
        // variance = beta^2 / ((alpha-1)^2 * (alpha-2))
        assert_relative_eq!(d.variance(), 2.0 / 9.0, epsilon = EPS);
        // mode = beta/(alpha+1)
        assert_relative_eq!(d.mode(), 0.4, epsilon = EPS);
        // skewness = 4*sqrt(alpha-2)/(alpha-3)
        assert_relative_eq!(d.skewness(), 4.0 * 2.0_f64.sqrt(), epsilon = EPS);
        // sf + cdf = 1
        let cdf_05 = d.cdf(0.5);
        assert_relative_eq!(d.sf(0.5), 1.0 - cdf_05, epsilon = EPS);
    }

    #[test]
    fn test_arcsine() {
        let d = Arcsine::new(0.0, 1.0);
        assert_relative_eq!(d.mean(), 0.5, epsilon = EPS);
        assert_relative_eq!(d.median(), 0.5, epsilon = EPS);
        // variance = (b-a)^2 / 8
        assert_relative_eq!(d.variance(), 0.125, epsilon = EPS);
        assert_relative_eq!(d.skewness(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.kurtosis_excess(), -1.5, epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(1.0), 1.0, epsilon = EPS);
        // sf(0.5) = 1 - cdf(0.5) = 0.5 (by symmetry)
        assert_relative_eq!(d.sf(0.5), 0.5, epsilon = EPS);
    }

    #[test]
    fn test_skew_normal() {
        // shape=0 reduces to standard normal
        let d = SkewNormal::new(0.0, 1.0, 0.0);
        assert_relative_eq!(d.mean(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.variance(), 1.0, epsilon = EPS);
        assert_relative_eq!(d.skewness(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.sf(0.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.pdf(0.0), 1.0 / (2.0 * PI).sqrt(), epsilon = EPS);

        // Positive shape shifts mean right and adds positive skew
        let d2 = SkewNormal::new(0.0, 1.0, 5.0);
        assert!(d2.mean() > 0.0);
        assert!(d2.skewness() > 0.0);
    }

    #[test]
    fn test_non_central_chi_squared() {
        let d = NonCentralChiSquared::new(4.0, 2.0);
        // mean = df + lambda
        assert_relative_eq!(d.mean(), 6.0, epsilon = EPS);
        // variance = 2*(df + 2*lambda)
        assert_relative_eq!(d.variance(), 16.0, epsilon = EPS);
        // skewness = 2^(3/2) * (df + 3*lambda) / (df + 2*lambda)^(3/2)
        let expected_skew = 2.0_f64.sqrt() * 2.0 * 10.0 / 8.0_f64.powf(1.5);
        assert_relative_eq!(d.skewness(), expected_skew, epsilon = EPS);
        // sf + cdf = 1
        let cdf_5 = d.cdf(5.0);
        assert_relative_eq!(d.sf(5.0), 1.0 - cdf_5, epsilon = EPS);
    }

    #[test]
    fn test_non_central_t() {
        // delta=0 reduces to Student's t
        let d = NonCentralT::new(10.0, 0.0);
        assert_relative_eq!(d.mean(), 0.0, epsilon = EPS);
        assert_relative_eq!(d.variance(), 1.25, epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.5, epsilon = EPS);
        assert_relative_eq!(d.sf(0.0), 0.5, epsilon = EPS);

        // Positive delta shifts distribution right
        let d2 = NonCentralT::new(10.0, 2.0);
        assert!(d2.mean() > 0.0);
    }

    #[test]
    fn test_non_central_beta() {
        // lambda=0 reduces to Beta(2,5)
        let d = NonCentralBeta::new(2.0, 5.0, 0.0);
        assert_relative_eq!(d.mean(), 2.0 / 7.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(0.0), 0.0, epsilon = EPS);
        assert_relative_eq!(d.cdf(1.0), 1.0, epsilon = EPS);
        assert_relative_eq!(d.sf(0.0), 1.0, epsilon = EPS);
        assert_relative_eq!(d.sf(1.0), 0.0, epsilon = EPS);
        // Skewness/kurtosis are undefined (returns NaN)
        assert!(d.skewness().is_nan());
        assert!(d.kurtosis_excess().is_nan());

        // Positive lambda increases mean
        let d2 = NonCentralBeta::new(2.0, 5.0, 3.0);
        assert!(d2.mean() > 2.0 / 7.0);
    }

    #[test]
    fn test_non_central_f() {
        // lambda=0 reduces to F(6,10)
        let d = NonCentralF::new(6.0, 10.0, 0.0);
        assert_relative_eq!(d.mean(), 1.25, epsilon = EPS);

        // mean = d2*(d1+lambda) / (d1*(d2-2))
        let d2 = NonCentralF::new(6.0, 10.0, 2.0);
        assert_relative_eq!(d2.mean(), 10.0 * 8.0 / (6.0 * 8.0), epsilon = EPS);
        // sf + cdf = 1
        let cdf_1 = d2.cdf(1.0);
        assert_relative_eq!(d2.sf(1.0), 1.0 - cdf_1, epsilon = EPS);
    }

    #[test]
    fn test_kolmogorov_smirnov() {
        let d = KolmogorovSmirnov::new(100.0);
        assert_relative_eq!(d.cdf(0.0), 0.0, epsilon = EPS);
        assert_relative_eq!(d.sf(0.0), 1.0, epsilon = EPS);
        // CDF is monotonically increasing
        assert!(d.cdf(0.1) < d.cdf(0.2));
        assert!(d.cdf(0.2) < d.cdf(0.3));
        // sf is monotonically decreasing
        assert!(d.sf(0.1) > d.sf(0.2));
        assert!(d.sf(0.2) > d.sf(0.3));
        // pdf is positive in the interior
        assert!(d.pdf(0.1) > 0.0);
    }
}
