//! boost/math/special_functions/owens_t.hpp

use crate::ffi;

/// Owen's *T* function
///
/// Corresponds to `boost::math::owens_t` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/owens_t.html>
pub fn owens_t(h: f64, a: f64) -> f64 {
    unsafe { ffi::math_owens_t(h, a) }
}

#[cfg(test)]
mod tests {
    use crate::math::owens_t;

    #[test]
    fn test_owens_t() {
        let r = owens_t(0.78, 3.5);
        assert_relative_eq!(r, 0.10877216734852274);
    }
}
