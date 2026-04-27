use super::Distribution;
use crate::ffi;

define_distribution! {
    /// Non-central beta distribution with shape parameters `alpha`, `beta` and
    /// non-centrality parameter `lambda`.
    ///
    /// [`skewness`](Distribution::skewness), [`kurtosis`](Distribution::kurtosis), and
    /// [`kurtosis_excess`](Distribution::kurtosis_excess) are not implemented in Boost
    /// for this distribution and return [`f64::NAN`].
    ///
    /// Corresponds to `boost::math::non_central_beta_distribution` in C++.
    /// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/dist_ref/dists/nc_beta_dist.html>
    NonCentralBeta { /// First shape parameter (alpha > 0).
                     alpha,
                     /// Second shape parameter (beta > 0).
                     beta,
                     /// Non-centrality parameter (lambda >= 0).
                     lambda }
    ffi: math_dist_non_central_beta_pdf, math_dist_non_central_beta_cdf,
         math_dist_non_central_beta_cdf_c, math_dist_non_central_beta_quantile,
         math_dist_non_central_beta_quantile_c, math_dist_non_central_beta_mean,
         math_dist_non_central_beta_variance, math_dist_non_central_beta_std_dev,
         math_dist_non_central_beta_skewness, math_dist_non_central_beta_kurtosis,
         math_dist_non_central_beta_kurtosis_excess, math_dist_non_central_beta_median,
         math_dist_non_central_beta_mode
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
    ffi: math_dist_non_central_chi_squared_pdf, math_dist_non_central_chi_squared_cdf,
         math_dist_non_central_chi_squared_cdf_c, math_dist_non_central_chi_squared_quantile,
         math_dist_non_central_chi_squared_quantile_c, math_dist_non_central_chi_squared_mean,
         math_dist_non_central_chi_squared_variance, math_dist_non_central_chi_squared_std_dev,
         math_dist_non_central_chi_squared_skewness, math_dist_non_central_chi_squared_kurtosis,
         math_dist_non_central_chi_squared_kurtosis_excess,
         math_dist_non_central_chi_squared_median, math_dist_non_central_chi_squared_mode
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
    ffi: math_dist_non_central_f_pdf, math_dist_non_central_f_cdf, math_dist_non_central_f_cdf_c,
         math_dist_non_central_f_quantile, math_dist_non_central_f_quantile_c,
         math_dist_non_central_f_mean, math_dist_non_central_f_variance,
         math_dist_non_central_f_std_dev, math_dist_non_central_f_skewness,
         math_dist_non_central_f_kurtosis, math_dist_non_central_f_kurtosis_excess,
         math_dist_non_central_f_median, math_dist_non_central_f_mode
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
    ffi: math_dist_non_central_t_pdf, math_dist_non_central_t_cdf, math_dist_non_central_t_cdf_c,
         math_dist_non_central_t_quantile, math_dist_non_central_t_quantile_c,
         math_dist_non_central_t_mean, math_dist_non_central_t_variance,
         math_dist_non_central_t_std_dev, math_dist_non_central_t_skewness,
         math_dist_non_central_t_kurtosis, math_dist_non_central_t_kurtosis_excess,
         math_dist_non_central_t_median, math_dist_non_central_t_mode
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::distributions::Distribution;

    const EPS: f64 = 1e-12;

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
}
