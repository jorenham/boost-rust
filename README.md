# Boost Rust Bindings

[![GitHub License](https://img.shields.io/github/license/jorenham/boost-rust?style=flat-square&color=333)](https://github.com/jorenham/boost-rust/blob/master/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/boost?style=flat-square&color=333)](https://crates.io/crates/boost)
[![docs.rs](https://img.shields.io/docsrs/boost?style=flat-square&color=333)](https://docs.rs/boost/)

Rust interface for the [Boost](https://github.com/boostorg/boost) C++ library.

> [!NOTE]
> This project is in early development and probably shouldn't be used in production.

## Project goals

Currently, this crate covers most of the Boost Math library's
[special functions](https://boost.org/doc/libs/latest/libs/math/doc/html/special.html).
In the long term, the aim is to include other Boost Math functionality, as well.
There are no plans yet for including other Boost libraries.

## Requirements

- Rust 1.85.1 or later
- C++20-compatible compiler (GCC 9+, Clang 14+, MSVC 2019+)

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## Acknowledgments

This crate provides bindings to the excellent [Boost Math library](https://github.com/boostorg/math).
All mathematical implementations are provided by Boost Math; this crate merely provides a safe Rust interface.
