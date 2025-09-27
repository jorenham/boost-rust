# Boost Rust Bindings

[![GitHub License](https://img.shields.io/github/license/jorenham/boost-rust?style=flat-square&color=333)](https://github.com/jorenham/boost-rust/blob/master/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/boost?style=flat-square&color=333)](https://crates.io/crates/boost)
[![docs.rs](https://img.shields.io/docsrs/boost?style=flat-square&color=333)](https://docs.rs/boost/)

Rust interface for the [Boost](https://github.com/boostorg/boost) C++ library.

> [!NOTE]
> This project is in the early development stage, and should probably not be used in production.

## Project goals

Currently, only a (small) subset of the [`boost::math`](https://github.com/boostorg/math/) API is
covered.
In the long term, the aim is to fully cover the Boost Math library, starting with the special
functions.

## Requirements

- Rust 1.85.1+
- Modern C++ compiler with C++14 support
- Boost Math library headers (tested against 1.83)

### Installing Boost

#### Ubuntu/Debian

```bash
sudo apt-get install libboost-all-dev
```

#### macOS (Homebrew)

```bash
brew install boost
```

#### Arch Linux

```bash
sudo pacman -S boost
```

#### From Source

Download and extract Boost from <https://www.boost.org/releases/latest/>.
Since Boost Math is header-only, you don't need to compile it—just make sure the headers are accessible.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
boost = "*"
```

## Build Configuration

If Boost is installed in a non-standard location, set the `BOOST_ROOT` environment variable:

```bash
export BOOST_ROOT=/path/to/boost
cargo build
```

## Testing

Run the test suite:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## Acknowledgments

This crate provides bindings to the excellent [Boost Math library](https://github.com/boostorg/math).
All mathematical implementations are provided by Boost Math; this crate merely provides a safe Rust interface.
