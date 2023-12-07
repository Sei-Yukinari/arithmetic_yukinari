# arithmetic_yukinari

[![Rust Test and Publish](https://github.com/Sei-Yukinari/arithmetic_yukinari/actions/workflows/ci.yml/badge.svg)](https://github.com/Sei-Yukinari/arithmetic_yukinari/actions/workflows/ci.yml)
[![crate-name at crates.io](https://img.shields.io/crates/v/arithmetic_yukinari.svg)](https://crates.io/crates/arithmetic_yukinari)
[![crate-name at docs.rs](https://docs.rs/arithmetic_yukinari/badge.svg)](https://docs.rs/arithmetic_yukinari)

## Description
This crate is a library for arithmetic operations.

## Installation
```toml
[dependencies]
arithmetic_yukinari = "0.1"
```

## Usage
```rust
use arithmetic_yukinari::{add, subtract, multiply, divide};

fn main() {
    let a = add(1, 2);
    assert_eq!(a, 3);
    let s = subtract(1, 2);
    assert_eq!(s, -1);
    let m = multiply(2, 3);
    assert_eq!(m, 6);
    let d = divide(6, 3);
    assert_eq!(d, Some(2));
}
```