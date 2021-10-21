#![feature(asm)]
#![no_main]
#![no_std]

extern crate builtins;

mod bootstrap;

include!("../../test_data.rs");

#[inline(never)]
fn main() -> u32 {
    test_data()
}