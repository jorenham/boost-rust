//! Bindings for [Boost Math](https://github.com/boostorg/math) C++ library

mod special_functions;

pub use special_functions::bessel::*;
pub use special_functions::beta::*;
pub use special_functions::digamma::*;
pub use special_functions::erf::*;
pub use special_functions::factorials::*;
pub use special_functions::gamma::*;
pub use special_functions::jacobi::*;
pub use special_functions::legendre::*;
pub use special_functions::prime::*;
