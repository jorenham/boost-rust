//! Rust interface for the [Boost](https://github.com/boostorg/boost) C++ library
#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
#[macro_use]
extern crate approx;

/// Bindings for [Boost Math](https://github.com/boostorg/math) C++ library
pub mod math;

mod ffi;
