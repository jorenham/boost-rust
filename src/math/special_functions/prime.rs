use crate::ffi;
use core::ffi::c_uint;

/// Corresponds to `static const unsigned max_prime` in `boost/math/special_functions/prime.hpp`
const MAX_PRIME: u32 = 9_999;

/// Fast table lookup to the first 10,000 prime numbers
///
/// The first prime number (`n=0`) is `2` (as `1` isn't terribly useful in practice).
///
/// The function will panic if `n >= 10_000`.
///
/// Corresponds to `boost::math::prime(n)` in C++.
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/number_series/primes.html>
pub fn prime(n: u32) -> u32 {
    if n > MAX_PRIME {
        panic!("Argument n out of range: got {n}");
    }
    unsafe { ffi::math_prime(n as c_uint) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime() {
        assert_eq!(prime(0), 2);
        assert_eq!(prime(54), 257);
        assert_eq!(prime(9_999), 104_729);
    }

    #[test]
    #[should_panic(expected = "Argument n out of range: got 10000")]
    fn test_prime_out_of_range() {
        let _ = prime(10_000);
    }
}
