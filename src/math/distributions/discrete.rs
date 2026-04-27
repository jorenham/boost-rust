use super::Distribution;
use crate::ffi;
use core::ffi::c_uint;

define_distribution! {
    /// Bernoulli distribution with success probability `p`.
    ///
    /// Corresponds to `boost::math::bernoulli_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/bernoulli_dist.html>
    Bernoulli { /// Success probability (0 <= p <= 1).
                p }
    ffi: math_dist_bernoulli_pdf, math_dist_bernoulli_cdf, math_dist_bernoulli_cdf_c,
         math_dist_bernoulli_quantile, math_dist_bernoulli_quantile_c, math_dist_bernoulli_mean,
         math_dist_bernoulli_variance, math_dist_bernoulli_std_dev, math_dist_bernoulli_skewness,
         math_dist_bernoulli_kurtosis, math_dist_bernoulli_kurtosis_excess,
         math_dist_bernoulli_median, math_dist_bernoulli_mode
}

define_distribution! {
    /// Binomial distribution with `n` trials and success probability `p`.
    ///
    /// Corresponds to `boost::math::binomial_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/binomial_dist.html>
    Binomial { /// Number of trials.
               n: u32,
               /// Success probability (0 <= p <= 1).
               p }
    ffi: math_dist_binomial_pdf, math_dist_binomial_cdf, math_dist_binomial_cdf_c,
         math_dist_binomial_quantile, math_dist_binomial_quantile_c, math_dist_binomial_mean,
         math_dist_binomial_variance, math_dist_binomial_std_dev, math_dist_binomial_skewness,
         math_dist_binomial_kurtosis, math_dist_binomial_kurtosis_excess, math_dist_binomial_median,
         math_dist_binomial_mode
}

define_distribution! {
    /// Geometric distribution with success probability `p`.
    ///
    /// Corresponds to `boost::math::geometric_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/geometric_dist.html>
    Geometric { /// Success probability (0 < p <= 1).
                p }
    ffi: math_dist_geometric_pdf, math_dist_geometric_cdf, math_dist_geometric_cdf_c,
         math_dist_geometric_quantile, math_dist_geometric_quantile_c, math_dist_geometric_mean,
         math_dist_geometric_variance, math_dist_geometric_std_dev, math_dist_geometric_skewness,
         math_dist_geometric_kurtosis, math_dist_geometric_kurtosis_excess,
         math_dist_geometric_median, math_dist_geometric_mode
}

/// Hypergeometric distribution with parameters `r`, `n`, and `N`.
///
/// Corresponds to `boost::math::hypergeometric_distribution` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/hypergeometric_dist.html>
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hypergeometric {
    /// Number of defective items in the population.
    pub r: u32,
    /// Number of items sampled.
    pub n: u32,
    /// Total population size (N).
    pub N: u32,
}

impl Hypergeometric {
    /// Creates a new hypergeometric distribution.
    #[allow(non_snake_case)]
    pub fn new(r: u32, n: u32, N: u32) -> Self {
        Self { r, n, N }
    }
}

impl Distribution for Hypergeometric {
    fn pdf(&self, x: f64) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_pdf(
                self.r as c_uint,
                self.n as c_uint,
                self.N as c_uint,
                x,
            )
        }
    }
    fn cdf(&self, x: f64) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_cdf(
                self.r as c_uint,
                self.n as c_uint,
                self.N as c_uint,
                x,
            )
        }
    }
    fn sf(&self, x: f64) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_cdf_c(
                self.r as c_uint,
                self.n as c_uint,
                self.N as c_uint,
                x,
            )
        }
    }
    fn quantile(&self, p: f64) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_quantile(
                self.r as c_uint,
                self.n as c_uint,
                self.N as c_uint,
                p,
            )
        }
    }
    fn isf(&self, q: f64) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_quantile_c(
                self.r as c_uint,
                self.n as c_uint,
                self.N as c_uint,
                q,
            )
        }
    }
    fn mean(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_mean(self.r as c_uint, self.n as c_uint, self.N as c_uint)
        }
    }
    fn variance(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_variance(
                self.r as c_uint,
                self.n as c_uint,
                self.N as c_uint,
            )
        }
    }
    fn standard_deviation(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_std_dev(
                self.r as c_uint,
                self.n as c_uint,
                self.N as c_uint,
            )
        }
    }
    fn skewness(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_skewness(
                self.r as c_uint,
                self.n as c_uint,
                self.N as c_uint,
            )
        }
    }
    fn kurtosis(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_kurtosis(
                self.r as c_uint,
                self.n as c_uint,
                self.N as c_uint,
            )
        }
    }
    fn kurtosis_excess(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_kurtosis_excess(
                self.r as c_uint,
                self.n as c_uint,
                self.N as c_uint,
            )
        }
    }
    fn median(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_median(
                self.r as c_uint,
                self.n as c_uint,
                self.N as c_uint,
            )
        }
    }
    fn mode(&self) -> f64 {
        unsafe {
            ffi::math_dist_hypergeometric_mode(self.r as c_uint, self.n as c_uint, self.N as c_uint)
        }
    }
}

define_distribution! {
    /// Negative binomial distribution with `successes` and success probability `p`.
    ///
    /// Corresponds to `boost::math::negative_binomial_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/negative_binomial_dist.html>
    NegativeBinomial { /// Number of successes.
                       successes: u32,
                       /// Success probability (0 < p <= 1).
                       p }
    ffi: math_dist_negative_binomial_pdf, math_dist_negative_binomial_cdf,
         math_dist_negative_binomial_cdf_c, math_dist_negative_binomial_quantile,
         math_dist_negative_binomial_quantile_c, math_dist_negative_binomial_mean,
         math_dist_negative_binomial_variance, math_dist_negative_binomial_std_dev,
         math_dist_negative_binomial_skewness, math_dist_negative_binomial_kurtosis,
         math_dist_negative_binomial_kurtosis_excess, math_dist_negative_binomial_median,
         math_dist_negative_binomial_mode
}

define_distribution! {
    /// Poisson distribution with given `mean`.
    ///
    /// Corresponds to `boost::math::poisson_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/poisson_dist.html>
    Poisson { /// Mean (expected value, lambda > 0).
              mean }
    ffi: math_dist_poisson_pdf, math_dist_poisson_cdf, math_dist_poisson_cdf_c,
         math_dist_poisson_quantile, math_dist_poisson_quantile_c, math_dist_poisson_mean,
         math_dist_poisson_variance, math_dist_poisson_std_dev, math_dist_poisson_skewness,
         math_dist_poisson_kurtosis, math_dist_poisson_kurtosis_excess, math_dist_poisson_median,
         math_dist_poisson_mode
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::distributions::Distribution;

    const EPS: f64 = 1e-12;

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
        let d = Binomial::new(10, 0.3);
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
    fn test_negative_binomial() {
        let d = NegativeBinomial::new(5, 0.4);
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
}
