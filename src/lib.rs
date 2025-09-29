//! Rust interface for the [Boost](https://github.com/boostorg/boost) C++ library.
//!
//! Currently, this crate only provides bindings for the Boost Math library
//! ([repo][boost-math-repo], [docs][boost-math-docs]), which will be the main focus of this
//! project for the time being.
//!
//! See the [`math`] module for details on the available functions.
//!
//! [boost-math-repo]: https://github.com/boostorg/math
//! [boost-math-docs]: https://www.boost.org/doc/libs/latest/libs/math/doc/html/index.html

#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate approx;

mod ffi;
pub mod math;
