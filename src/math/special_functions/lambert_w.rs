//! boost/math/special_functions/lambert_w.hpp

use crate::ffi;

/// Lambert W function for the principal branch *k=0*
///
/// See [`lambert_wm1`] for principal branch *k=-1*.
///
/// Corresponds to `boost::math::lambert_w0` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/lambert_w.html>
///
/// # Examples
///
/// ```
/// # use approx::assert_relative_eq;
/// # use boost::math::lambert_w0;
/// let x = 1.0;
/// let w = lambert_w0(x);
/// println!("w = {w}");
/// assert_relative_eq!(w * w.exp(), x);
/// ```
pub fn lambert_w0(x: f64) -> f64 {
    unsafe { ffi::math_lambert_w0(x) }
}

/// Derivative of [`lambert_w0`]
///
/// Corresponds to `boost::math::lambert_w0_prime` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/lambert_w.html>
pub fn lambert_w0_prime(x: f64) -> f64 {
    unsafe { ffi::math_lambert_w0_prime(x) }
}

/// Lambert W function for the principal branch *k=-1*
///
/// See [`lambert_w0`] for principal branch *k=0*.
///
/// Corresponds to `boost::math::lambert_wm1` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/lambert_w.html>
///
/// # Examples
///
/// ```
/// # use approx::assert_relative_eq;
/// # use boost::math::lambert_wm1;
/// let x = -0.2;
/// let w = lambert_wm1(x);
/// println!("w = {w}");
/// assert_relative_eq!(w * w.exp(), x);
/// ```
pub fn lambert_wm1(x: f64) -> f64 {
    unsafe { ffi::math_lambert_wm1(x) }
}

/// Derivative of [`lambert_wm1`]
///
/// Corresponds to `boost::math::lambert_wm1_prime` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/lambert_w.html>
pub fn lambert_wm1_prime(x: f64) -> f64 {
    unsafe { ffi::math_lambert_wm1_prime(x) }
}
