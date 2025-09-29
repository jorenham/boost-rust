//! Rust interface for the [Boost](https://github.com/boostorg/boost) C++ library

#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate approx;

mod ffi;
pub mod math;
