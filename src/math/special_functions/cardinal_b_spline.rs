//! Pure rust reimplementation of boost/math/special_functions/cardinal_b_spline.hpp

mod detail {
    /// same as `boost::math::detail::B1` in `boost/math/special_functions/cardinal_b_spline.hpp`
    #[inline(always)]
    #[allow(non_snake_case)]
    pub(super) fn B1(x: f64) -> f64 {
        if x < 0.0 {
            B1(-x)
        } else if x < 1.0 {
            1.0 - x
        } else {
            0.0
        }
    }
}

/// Cardinal B-spline *B<sub>N</sub>(x)*
///
/// See [`forward_cardinal_b_spline`] for a version with shifted support.
///
/// Pure rust reimplementation of the `boost::math::cardinal_b_spline` C++ function.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/cardinal_b_splines.html>
pub fn cardinal_b_spline<const N: usize>(x: f64) -> f64 {
    if x < 0.0 {
        // All B-splines are even functions:
        return cardinal_b_spline::<N>(-x);
    }

    if N == 0 {
        return if x < 0.5 {
            1.0
        } else if x == 0.5 {
            0.5
        } else {
            0.0
        };
    }

    if N == 1 {
        return detail::B1(x);
    }

    let supp_max = ((N + 1) as f64) / 2.0;
    if x >= supp_max {
        return 0.0;
    }

    // Fill v with values of B1:
    // At most two of these terms are nonzero, and at least 1.
    // There is only one non-zero term when n is odd and x = 0.
    let mut v = [0.0; N];
    let mut z = x + 1.0 - supp_max;
    #[allow(clippy::needless_range_loop)]
    for i in 0..N {
        v[i] = detail::B1(z);
        z += 1.0;
    }

    let smx = supp_max - x;
    for j in 2..=N {
        let mut a = (j + 1) as f64 - smx;
        let mut b = smx;
        for k in 0..=N - j {
            v[k] = (a * v[k + 1] + b * v[k]) / (j as f64);
            a += 1.0;
            b -= 1.0;
        }
    }

    v[0]
}

/// Cardinal B-spline derivative *B'<sub>N</sub>(x)*
///
/// See [`cardinal_b_spline`] for the original, and [`cardinal_b_spline_double_prime`] for the
/// second derivative.
///
/// Pure rust reimplementation of the `boost::math::cardinal_b_spline_prime` C++ function.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/cardinal_b_splines.html>
pub fn cardinal_b_spline_prime<const N: usize>(x: f64) -> f64 {
    if x < 0.0 {
        // All B-splines are even functions, so derivatives are odd:
        return -cardinal_b_spline_prime::<N>(-x);
    }

    if N == 0 {
        // Kinda crazy but you get what you ask for!
        return if x == 0.5 { f64::INFINITY } else { 0.0 };
    }

    if N == 1 {
        return if x == 0.0 {
            0.0
        } else if x == 1.0 {
            -0.5
        } else {
            -1.0
        };
    }

    let supp_max = ((N + 1) as f64) / 2.0;
    if x >= supp_max {
        return 0.0;
    }

    // Now we want to evaluate B_{n}(x), but stop at the second to last step and collect
    // B_{n-1}(x+1/2) and B_{n-1}(x-1/2):
    let mut v = [0.0; N];
    let mut z = x + 1.0 - supp_max;
    #[allow(clippy::needless_range_loop)]
    for i in 0..N {
        v[i] = detail::B1(z);
        z += 1.0;
    }

    let smx = supp_max - x;
    for j in 2..=(N - 1) {
        let mut a = (j + 1) as f64 - smx;
        let mut b = smx;
        for k in 0..=N - j {
            v[k] = (a * v[k + 1] + b * v[k]) / (j as f64);
            a += 1.0;
            b -= 1.0;
        }
    }

    v[1] - v[0]
}

/// Cardinal B-spline second derivative *B''<sub>N</sub>(x)*
///
/// See [`cardinal_b_spline`] for the non-derivative, and [`cardinal_b_spline_prime`] for
/// the first derivative.
///
/// Pure rust reimplementation of the `boost::math::cardinal_b_spline_double_prime` C++ function.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/cardinal_b_splines.html>
pub fn cardinal_b_spline_double_prime<const N: usize>(x: f64) -> f64 {
    assert!(
        N >= 3,
        "n>=3 for second derivatives of cardinal B-splines is required."
    );

    if x < 0.0 {
        // All B-splines are even functions, so second derivatives are even:
        return cardinal_b_spline_double_prime::<N>(-x);
    }

    let supp_max = ((N + 1) as f64) / 2.0;
    if x >= supp_max {
        return 0.0;
    }

    // Now we want to evaluate B_{n}(x), but stop at the second to last step and collect
    // B_{n-1}(x+1/2) and B_{n-1}(x-1/2):
    let mut v = [0.0; N];
    let mut z = x + 1.0 - supp_max;
    #[allow(clippy::needless_range_loop)]
    for i in 0..N {
        v[i] = detail::B1(z);
        z += 1.0;
    }

    let smx = supp_max - x;
    for j in 2..=(N - 2) {
        let mut a = (j + 1) as f64 - smx;
        let mut b = smx;
        for k in 0..=N - j {
            v[k] = (a * v[k + 1] + b * v[k]) / (j as f64);
            a += 1.0;
            b -= 1.0;
        }
    }

    v[2] - 2.0 * v[1] + v[0]
}

/// Forward shifted Cardinal B-spline *B<sub>N</sub>(x - (N+1)/2)*
///
/// Support is shifted to [0, N+1].
///
/// See [`cardinal_b_spline`] for the original.
///
/// Pure rust reimplementation of the `boost::math::forward_cardinal_b_spline` C++ function.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/sf_poly/cardinal_b_splines.html>
#[inline(always)]
pub fn forward_cardinal_b_spline<const N: usize>(x: f64) -> f64 {
    cardinal_b_spline::<N>(x - (N + 1) as f64 / 2.0)
}

#[cfg(test)]
mod tests {
    use crate::math::{
        cardinal_b_spline, cardinal_b_spline_double_prime, cardinal_b_spline_prime,
        forward_cardinal_b_spline,
    };

    // These tests are based on math/tests/cardinal_b_spline_test.cpp

    #[test]
    fn test_box() {
        // Test outside support
        assert_ulps_eq!(cardinal_b_spline::<0>(1.1), 0.0);
        assert_ulps_eq!(cardinal_b_spline_prime::<0>(1.1), 0.0);
        assert_ulps_eq!(cardinal_b_spline::<0>(-1.1), 0.0);

        // Test at boundary
        assert_eq!(cardinal_b_spline::<0>(0.5), 0.5);
        assert_eq!(cardinal_b_spline_prime::<0>(0.5), f64::INFINITY);

        // Test inside support
        let h = 1.0f64 / 256.0;
        let mut t = -0.5f64 + h;
        while t < 0.5 {
            assert_ulps_eq!(cardinal_b_spline::<0>(t), 1.0);
            assert_ulps_eq!(cardinal_b_spline_prime::<0>(t), 0.0);
            t += h;
        }

        // Test forward cardinal B-spline
        t = h;
        while t < 1.0f64 {
            assert_ulps_eq!(forward_cardinal_b_spline::<0>(t), 1.0);
            t += h;
        }
    }

    #[test]
    fn test_hat() {
        // Test outside support
        assert_ulps_eq!(cardinal_b_spline::<1>(2.1), 0.0);
        assert_ulps_eq!(cardinal_b_spline::<1>(-2.1), 0.0);

        let h = 1.0f64 / 256.0;
        let mut t = -1.0f64;
        while t <= 1.0 {
            let expected = 1.0 - t.abs();
            assert_ulps_eq!(cardinal_b_spline::<1>(t), expected);

            // Test derivative
            let expected_prime = if t == -1.0 {
                0.5
            } else if t == 1.0 {
                -0.5
            } else if t < 0.0 {
                1.0
            } else if t == 0.0 {
                0.0
            } else {
                -1.0
            };
            assert_ulps_eq!(cardinal_b_spline_prime::<1>(t), expected_prime);

            t += h;
        }

        // Test forward cardinal B-spline
        t = 0.0f64;
        while t < 2.0 {
            let expected = 1.0 - (t - 1.0).abs();
            assert_ulps_eq!(forward_cardinal_b_spline::<1>(t), expected);
            t += h;
        }
    }

    #[test]
    fn test_quadratic() {
        fn b2(x: f64) -> f64 {
            let absx = x.abs();
            if absx >= 1.5 {
                0.0
            } else if absx >= 0.5 {
                let t = absx - 1.5;
                t * t / 2.0
            } else {
                let t1 = absx - 0.5;
                let t2 = absx + 0.5;
                (2.0 - t1 * t1 - t2 * t2) / 2.0
            }
        }

        fn b2_prime(x: f64) -> f64 {
            let absx = x.abs();
            let signx = if x < 0.0 { -1.0 } else { 1.0 };
            if absx >= 1.5 {
                0.0
            } else if absx >= 0.5 {
                (absx - 1.5) * signx
            } else {
                -2.0 * absx * signx
            }
        }

        let h = 1.0f64 / 256.0;
        let mut t = -5.0f64;
        while t <= 5.0 {
            let expected = b2(t);
            assert_ulps_eq!(cardinal_b_spline::<2>(t), expected);

            let expected_prime = b2_prime(t);
            assert_ulps_eq!(cardinal_b_spline_prime::<2>(t), expected_prime);

            t += h;
        }
    }

    #[test]
    fn test_cubic() {
        assert_ulps_eq!(cardinal_b_spline::<3>(0.0), 2.0 / 3.0);
        assert_ulps_eq!(cardinal_b_spline::<3>(1.0), 1.0 / 6.0);
        assert_ulps_eq!(cardinal_b_spline::<3>(2.0), 0.0);
    }

    #[test]
    fn test_quintic() {
        assert_ulps_eq!(cardinal_b_spline::<5>(0.0), 11.0 / 20.0);
        assert_relative_eq!(cardinal_b_spline::<5>(1.0), 13.0 / 60.0, epsilon = 1e-15);
        assert_ulps_eq!(cardinal_b_spline::<5>(2.0), 1.0 / 120.0);
        assert_ulps_eq!(cardinal_b_spline::<5>(3.0), 0.0);
    }

    #[test]
    fn test_b_spline_derivatives_n3() {
        let h = 1.0f64 / 256.0;
        let supp = 2.0f64; // (3+1)/2 = 2
        let mut t = -supp - 1.0;
        while t <= supp + 1.0 {
            // B'_n(x) = B_{n-1}(x+1/2) - B_{n-1}(x-1/2)
            let expected = cardinal_b_spline::<2>(t + 0.5) - cardinal_b_spline::<2>(t - 0.5);
            let computed = cardinal_b_spline_prime::<3>(t);
            assert_abs_diff_eq!(expected, computed, epsilon = f64::EPSILON);

            // B''_n(x) = B_{n-2}(x+1) - 2*B_{n-2}(x) + B_{n-2}(x-1)
            let expected_2nd = cardinal_b_spline::<1>(t + 1.0) - 2.0 * cardinal_b_spline::<1>(t)
                + cardinal_b_spline::<1>(t - 1.0);
            let computed_2nd = cardinal_b_spline_double_prime::<3>(t);
            assert_abs_diff_eq!(expected_2nd, computed_2nd, epsilon = 2.0 * f64::EPSILON);

            t += h;
        }
    }

    #[test]
    fn test_b_spline_derivatives_n4() {
        let h = 1.0f64 / 256.0;
        let supp = 2.5f64; // (4+1)/2 = 2.5
        let mut t = -supp - 1.0;
        while t <= supp + 1.0 {
            // B'_n(x) = B_{n-1}(x+1/2) - B_{n-1}(x-1/2)
            let expected = cardinal_b_spline::<3>(t + 0.5) - cardinal_b_spline::<3>(t - 0.5);
            let computed = cardinal_b_spline_prime::<4>(t);
            assert_abs_diff_eq!(expected, computed, epsilon = f64::EPSILON);

            // B''_n(x) = B_{n-2}(x+1) - 2*B_{n-2}(x) + B_{n-2}(x-1)
            let expected_2nd = cardinal_b_spline::<2>(t + 1.0) - 2.0 * cardinal_b_spline::<2>(t)
                + cardinal_b_spline::<2>(t - 1.0);
            let computed_2nd = cardinal_b_spline_double_prime::<4>(t);
            assert_abs_diff_eq!(expected_2nd, computed_2nd, epsilon = 2.0 * f64::EPSILON);

            t += h;
        }
    }

    #[test]
    fn test_b_spline_derivatives_n5() {
        let h = 1.0f64 / 256.0;
        let supp = 3.0f64; // (5+1)/2 = 3
        let mut t = -supp - 1.0;
        while t <= supp + 1.0 {
            // B'_n(x) = B_{n-1}(x+1/2) - B_{n-1}(x-1/2)
            let expected = cardinal_b_spline::<4>(t + 0.5) - cardinal_b_spline::<4>(t - 0.5);
            let computed = cardinal_b_spline_prime::<5>(t);
            assert_abs_diff_eq!(expected, computed, epsilon = f64::EPSILON);

            // B''_n(x) = B_{n-2}(x+1) - 2*B_{n-2}(x) + B_{n-2}(x-1)
            let expected_2nd = cardinal_b_spline::<3>(t + 1.0) - 2.0 * cardinal_b_spline::<3>(t)
                + cardinal_b_spline::<3>(t - 1.0);
            let computed_2nd = cardinal_b_spline_double_prime::<5>(t);
            assert_abs_diff_eq!(expected_2nd, computed_2nd, epsilon = 2.0 * f64::EPSILON);

            t += h;
        }
    }
    #[test]
    fn test_partition_of_unity() {
        // Test for different orders similar to C++ version

        // Order 1: test several starting points in range [-supp, -supp+1)
        let supp1 = 1.0f64; // (1+1)/2 = 1
        for &x_start in &[-0.9, -0.5, -0.1] {
            let mut x = x_start;
            let mut sum = 0.0;
            while x < supp1 {
                sum += cardinal_b_spline::<1>(x);
                x += 1.0;
            }
            assert_relative_eq!(sum, 1.0, epsilon = 1e-14);
        }

        // Order 2: test several starting points in range [-supp, -supp+1)
        let supp2 = 1.5f64; // (2+1)/2 = 1.5
        for &x_start in &[-1.4, -1.0, -0.6] {
            let mut x = x_start;
            let mut sum = 0.0;
            while x < supp2 {
                sum += cardinal_b_spline::<2>(x);
                x += 1.0;
            }
            assert_relative_eq!(sum, 1.0, epsilon = 1e-14);
        }

        // Order 3: test several starting points in range [-supp, -supp+1)
        let supp3 = 2.0f64; // (3+1)/2 = 2
        for &x_start in &[-1.9, -1.5, -1.1] {
            let mut x = x_start;
            let mut sum = 0.0;
            while x < supp3 {
                sum += cardinal_b_spline::<3>(x);
                x += 1.0;
            }
            assert_relative_eq!(sum, 1.0, epsilon = 1e-14);
        }

        // Order 4: test several starting points in range [-supp, -supp+1)
        let supp4 = 2.5f64; // (4+1)/2 = 2.5
        for &x_start in &[-2.4, -2.0, -1.6] {
            let mut x = x_start;
            let mut sum = 0.0;
            while x < supp4 {
                sum += cardinal_b_spline::<4>(x);
                x += 1.0;
            }
            assert_relative_eq!(sum, 1.0, epsilon = 1e-14);
        }
    }

    #[test]
    fn test_partition_of_unity_n2() {
        let supp = 1.5f64; // (2+1)/2 = 1.5

        for &x_start in &[-1.2, -0.5, 0.0, 0.3, 1.0] {
            let mut x = x_start;
            let mut sum = 0.0;
            while x < supp {
                sum += cardinal_b_spline::<2>(x);
                x += 1.0;
            }
            assert_relative_eq!(sum, 1.0, epsilon = 2.0);
        }
    }

    #[test]
    fn test_partition_of_unity_n3() {
        let supp = 2.0f64; // (3+1)/2 = 2

        for &x_start in &[-1.5, -0.7, 0.0, 0.4, 1.2] {
            let mut x = x_start;
            let mut sum = 0.0;
            while x < supp {
                sum += cardinal_b_spline::<3>(x);
                x += 1.0;
            }
            assert_relative_eq!(sum, 1.0, epsilon = 3.0);
        }
    }
}
