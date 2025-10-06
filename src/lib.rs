#![no_std]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

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

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate approx;

mod ffi;
pub mod math;
