/// This function accesses `gs:[0x10]`, which is `_TEB.NtTib.StackLimit`.
/// This value is not officially documented, but:
/// - it has not changed since Win9x.
/// - the CRT generates the same `__chkstk` implementation, which means it `gs:[0x10]` has to be
///   stable and always point to `StackLimit`.
#[naked]
#[no_mangle]
unsafe extern fn __chkstk() {
    asm!(
        "
        sub   rsp, 0x10
        mov   qword ptr [rsp], r10
        mov   qword ptr [rsp + 0x8], r11

        xor   r11, r11
        lea   r10, qword ptr [rsp + 0x18]
        sub   r10, rax
        cmovb r10, r11
        mov   r11, qword ptr gs:[0x10]
        cmp   r10, r11
        jnb   3f

        and   r10w, 0xF000

        2:
        lea   r11, qword ptr [r11 - 0x1000]
        test  byte ptr [r11], r11b
        cmp   r10, r11
        jb    2b

        3:
        mov   r10, qword ptr [rsp]
        mov   r11, qword ptr [rsp + 0x8]
        add   rsp, 10h
        ret
        ",

        options(noreturn)
    );
}