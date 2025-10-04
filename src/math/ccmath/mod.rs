//! Constexpr CMath
//!
//! Corresponds to the Boost Math `boost::math::ccmath` C++ namespace.
//! <https://boost.org/doc/libs/latest/libs/math/doc/html/math_toolkit/ccmath.html>
//!
//! - [x] `sqrt(x)`
//! - [ ] `logb(x)`
//! - [ ] `frexp(x, *exp)`, `ldexp(x, exp)`, `scalbn(x, exp)`
//! - [ ] `floor(x)`, `ceil(x)`, `trunc(x)`, `round(x)`
//! - [ ] `modf(x, *iptr)`
//! - [ ] `fmod(x, y)`
//! - [ ] `fdim(x, y)`
//! - [ ] `isunordered(x, y)`
//! - [ ] `div(x, y) -> {quot, rem}`
//!
//! Some functions are omitted because they are already available in `core::f64` or `boost::math`.

mod sqrt;
pub use sqrt::*;
