use core::ops::{Add, Mul, Sub};

/// Computes the n-th Fibonacci number.
///
/// Translated from the Boost.Math C++ implementation of `boost::math::unchecked_fibonacci`:
/// <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/number_series/fibonacci_numbers.html>
///
/// # Examples
///
/// ```
/// use boost::math::fibonacci;
///
/// assert_eq!(fibonacci::<u32>(0), 0);
/// assert_eq!(fibonacci::<u32>(1), 1);
/// assert_eq!(fibonacci::<u32>(10), 55);
/// assert_eq!(fibonacci::<u64>(42), 267_914_296);
/// assert_eq!(fibonacci::<u128>(100), 354_224_848_179_261_915_075);
/// assert_eq!(fibonacci::<u128>(150), 9_969_216_677_189_303_386_214_405_760_200);
/// ```
#[inline]
pub fn fibonacci<T>(n: u32) -> T
where
    T: From<u8> + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    // This function is called by the rest and computes the actual nth fibonacci number
    // First few fibonacci numbers: 0 (0th), 1 (1st), 1 (2nd), 2 (3rd), ...
    if n <= 2 {
        return if n == 0 { 0 } else { 1 }.into();
    }

    // This is based on the following identities by Dijkstra:
    //   F(2*n-1) = F(n-1)^2 + F(n)^2
    //   F(2*n)   = (2*F(n-1) + F(n)) * F(n)
    // The implementation is iterative and is unrolled version of trivial recursive implementation.

    let mut mask: u32 = 1;
    while mask << 1 <= n && mask << 1 != 0 {
        mask <<= 1;
    }

    let (mut a, mut b) = (T::from(1), T::from(1));
    mask >>= 1;
    while mask != 0 {
        let t1 = a * a;
        a = (a + a) * b - t1; // 2 * a * b - t1
        b = b * b + t1;

        if mask & n != 0 {
            (b, a) = (b + a, b); // equivalent to: swap(a, b), b += a;
        }

        mask >>= 1;
    }

    a
}
