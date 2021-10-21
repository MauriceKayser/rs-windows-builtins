#![feature(asm)]

include!("../../test_data.rs");

#[inline(never)]
fn main() {
    std::process::exit(test_data() as i32);
}