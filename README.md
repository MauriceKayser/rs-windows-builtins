Related to [GitHub: rust-lang/compiler-builtins #403](https://github.com/rust-lang/compiler-builtins/issues/403).

# Project structure

The project consists of the following three items:

## 1. `test_data.rs`

Tests the `chkstk` implementation for `x86` and `x86_64` targets, and 64-bit calculations for
`x86` targets.

## 2. `no_std`

`#[!no_std]` binary which implements the `chkstk` and all 64-bit calculation intrinsics in
`#[naked]` functions with the `asm!` macro. Executes `test_data.rs` based on these
implementations.

## 3. `std`

`std` binary which executes `test_data.rs` based on the `chkstk` and 64-bit calculation
intrinsic implementations which the CRT generates.