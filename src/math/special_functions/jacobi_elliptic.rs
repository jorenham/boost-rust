//! boost/math/special_functions/jacobi_elliptic.hpp

use crate::ffi;

/// Jacobi elliptic functions *sn(θ, k)*, *cn(θ, k)*, and *dn(θ, k)*
///
/// Corresponds to `boost::math::jacobi_elliptic(u, m, *pcn, *pdn)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_elliptic(k: f64, theta: f64) -> (f64, f64, f64) {
    let mut cn = f64::NAN;
    let mut dn = f64::NAN;
    let sn = unsafe { ffi::math_jacobi_elliptic(k, theta, &mut cn, &mut dn) };
    (sn, cn, dn)
}

/// Jacobi elliptic function *cd(θ, k)*
///
/// Corresponds to `boost::math::jacobi_cd(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_cd(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_cd(k, theta) }
}

/// Jacobi elliptic function *cn(θ, k)*
///
/// Corresponds to `boost::math::jacobi_cn(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_cn(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_cn(k, theta) }
}

/// Jacobi elliptic function *cs(θ, k)*
///
/// Corresponds to `boost::math::jacobi_cs(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_cs(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_cs(k, theta) }
}

/// Jacobi elliptic function *dc(θ, k)*
///
/// Corresponds to `boost::math::jacobi_dc(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_dc(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_dc(k, theta) }
}

/// Jacobi elliptic function *dn(θ, k)*
///
/// Corresponds to `boost::math::jacobi_dn(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_dn(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_dn(k, theta) }
}

/// Jacobi elliptic function *ds(θ, k)*
///
/// Corresponds to `boost::math::jacobi_ds(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_ds(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_ds(k, theta) }
}

/// Jacobi elliptic function *nc(θ, k)*
///
/// Corresponds to `boost::math::jacobi_nc(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_nc(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_nc(k, theta) }
}

/// Jacobi elliptic function *nd(θ, k)*
///
/// Corresponds to `boost::math::jacobi_nd(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_nd(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_nd(k, theta) }
}

/// Jacobi elliptic function *ns(θ, k)*
///
/// Corresponds to `boost::math::jacobi_ns(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_ns(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_ns(k, theta) }
}

/// Jacobi elliptic function *sc(θ, k)*
///
/// Corresponds to `boost::math::jacobi_sc(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_sc(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_sc(k, theta) }
}

/// Jacobi elliptic function *sd(θ, k)*
///
/// Corresponds to `boost::math::jacobi_sd(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_sd(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_sd(k, theta) }
}

/// Jacobi elliptic function *sn(θ, k)*
///
/// Corresponds to `boost::math::jacobi_sn(k, theta)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/jacobi/jacobi_elliptic.html>
pub fn jacobi_sn(k: f64, theta: f64) -> f64 {
    unsafe { ffi::math_jacobi_sn(k, theta) }
}

#[cfg(test)]
mod smoketests {
    use super::*;

    #[test]
    fn test_jacobi_elliptic() {
        let (sn, cn, dn) = jacobi_elliptic(0.5, 1.0);
        assert!(sn.is_finite());
        assert!(cn.is_finite());
        assert!(dn.is_finite());
    }

    #[test]
    fn test_jacobi_cd() {
        assert!(jacobi_cd(0.5, 1.0).is_finite());
    }

    #[test]
    fn test_jacobi_cn() {
        assert!(jacobi_cn(0.5, 1.0).is_finite());
    }

    #[test]
    fn test_jacobi_cs() {
        assert!(jacobi_cs(0.5, 1.0).is_finite());
    }

    #[test]
    fn test_jacobi_dc() {
        assert!(jacobi_dc(0.5, 1.0).is_finite());
    }

    #[test]
    fn test_jacobi_dn() {
        assert!(jacobi_dn(0.5, 1.0).is_finite());
    }

    #[test]
    fn test_jacobi_ds() {
        assert!(jacobi_ds(0.5, 1.0).is_finite());
    }

    #[test]
    fn test_jacobi_nc() {
        assert!(jacobi_nc(0.5, 1.0).is_finite());
    }

    #[test]
    fn test_jacobi_nd() {
        assert!(jacobi_nd(0.5, 1.0).is_finite());
    }

    #[test]
    fn test_jacobi_ns() {
        assert!(jacobi_ns(0.5, 1.0).is_finite());
    }

    #[test]
    fn test_jacobi_sc() {
        assert!(jacobi_sc(0.5, 1.0).is_finite());
    }

    #[test]
    fn test_jacobi_sd() {
        assert!(jacobi_sd(0.5, 1.0).is_finite());
    }

    #[test]
    fn test_jacobi_sn() {
        assert!(jacobi_sn(0.5, 1.0).is_finite());
    }
}
