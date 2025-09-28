//! Rust interface for the [Boost](https://github.com/boostorg/boost) C++ library

#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
#[macro_use]
extern crate approx;

mod ffi;
pub mod math;
