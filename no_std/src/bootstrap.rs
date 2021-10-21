#[panic_handler]
fn rust_begin_unwind(_info: &core::panic::PanicInfo) -> ! {
    unsafe { TerminateProcess(-1, 0xC0000027); }
}

#[no_mangle]
extern "system" fn mainCRTStartup() -> ! {
    unsafe { TerminateProcess(-1, super::main()); }
}

#[link(name = "kernel32", kind = "dylib")]
extern "system" {
    fn TerminateProcess(process: isize, exit_code: u32) -> !;
}