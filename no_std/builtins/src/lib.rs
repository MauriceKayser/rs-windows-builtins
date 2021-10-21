//! The implementations are copies of Windows 10 `System32/ntdll.dll` & `SysWOW64/ntdll.dll` exports,
//! but they are the same as the ones the CRT generates.

#![feature(asm, naked_functions)]
#![no_std]

#[cfg(all(windows, target_arch = "x86"))]
mod x86;

#[cfg(all(windows, target_arch = "x86_64"))]
mod x86_64;

#[no_mangle]
static _fltused: i32 = 0;