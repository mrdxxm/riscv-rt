use core::arch::global_asm;

/// Parse cfg attributes inside a global_asm call.
#[macro_export]
macro_rules! cfg_global_asm {
    {@inner, [$($x:tt)*], } => {
        global_asm!{$($x)*}
    };
    (@inner, [$($x:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
        #[cfg(not($meta))]
        cfg_global_asm!{@inner, [$($x)*], $($rest)*}
    };
    {@inner, [$($x:tt)*], $asm:literal, $($rest:tt)*} => {
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
    };
    {$($asms:tt)*} => {
        cfg_global_asm!{@inner, [], $($asms)*}
    };
}

// Entry point of all programs (_start). It initializes DWARF call frame information,
// the stack pointer, the frame pointer (needed for closures to work in start_rust)
// and the global pointer. Then it calls _start_rust.
cfg_global_asm!(
    r#"
    .section .init, "ax"
    .global _start

_start:"#,
    #[cfg(riscv32)]
    "lui ra, %hi(_abs_start)
     jr %lo(_abs_start)(ra)",
    #[cfg(riscv64)]
    ".option push
    .option norelax // to prevent an unsupported R_RISCV_ALIGN relocation from being generated
1:
    auipc ra, %pcrel_hi(1f)
    ld ra, %pcrel_lo(1b)(ra)
    jr ra
    .align  3
1:
    .dword _abs_start
    .option pop",
    "
_abs_start:
    .option norelax
    .cfi_startproc
    .cfi_undefined ra",
    #[cfg(feature = "s-mode")]
    "csrw sie, 0
    csrw sip, 0",
    #[cfg(not(feature = "s-mode"))]
    "csrw mie, 0
    csrw mip, 0",
    "li  x1, 0
    li  x2, 0
    li  x3, 0
    li  x4, 0
    li  x5, 0
    li  x6, 0
    li  x7, 0
    li  x8, 0
    li  x9, 0
    // a0..a2 (x10..x12) skipped
    li  x13, 0
    li  x14, 0
    li  x15, 0
    li  x16, 0
    li  x17, 0
    li  x18, 0
    li  x19, 0
    li  x20, 0
    li  x21, 0
    li  x22, 0
    li  x23, 0
    li  x24, 0
    li  x25, 0
    li  x26, 0
    li  x27, 0
    li  x28, 0
    li  x29, 0
    li  x30, 0
    li  x31, 0

    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop",
    #[cfg(feature = "s-mode")]
    "mv t2, a0 // the hartid is passed as parameter by SMODE",
    #[cfg(not(feature = "s-mode"))]
    "csrr t2, mhartid",
    "lui t0, %hi(_max_hart_id)
    add t0, t0, %lo(_max_hart_id)
    bgtu t2, t0, abort

    // Allocate stacks
    la sp, _stack_start
    lui t0, %hi(_hart_stack_size)
    add t0, t0, %lo(_hart_stack_size)",
    #[cfg(riscvm)]
    "mul t0, t2, t0",
    #[cfg(not(riscvm))]
    "beqz t2, 2f  // Jump if single-hart
    mv t1, t2
    mv t3, t0
1:
    add t0, t0, t3
    addi t1, t1, -1
    bnez t1, 1b
2:  ",
    "sub sp, sp, t0
    li t0, 0x1f
    csrw 0xbc0, t0

    ",
    "// Set frame pointer 
    add s0, sp, zero

    jal zero, _start_rust

    .cfi_endproc",
);

/// Trap entry point (_start_trap). It saves caller saved registers, calls
/// _start_trap_rust, restores caller saved registers and then returns.
/// 
/// # Usage
/// 
/// The macro takes 5 arguments:
/// - `$STORE`: the instruction used to store a register in the stack (e.g. `sd` for riscv64)
/// - `$LOAD`: the instruction used to load a register from the stack (e.g. `ld` for riscv64)
/// - `$BYTES`: the number of bytes used to store a register (e.g. 8 for riscv64)
/// - `$TRAP_SIZE`: the number of registers to store in the stack (e.g. 32 for all the user registers)
/// - list of tuples of the form `($REG, $LOCATION)`, where:
///     - `$REG`: the register to store/load
///     - `$LOCATION`: the location in the stack where to store/load the register
#[rustfmt::skip]
macro_rules! trap_handler {
    ($STORE:ident, $LOAD:ident, $BYTES:literal, $TRAP_SIZE:literal, [$(($REG:ident, $LOCATION:literal)),*]) => {
        global_asm!(
        r#"
            .section .trap, "ax"
            .weak default_start_trap
            .weak _start_trap1
            .weak _start_trap3
            .weak _start_trap2
            .weak _start_trap4
            .weak _start_trap5
            .weak _start_trap6
            .weak _start_trap7
            .weak _start_trap8
            .weak _start_trap9
            .weak _start_trap10
            .weak _start_trap11
            .weak _start_trap12
            .weak _start_trap13
            .weak _start_trap14
            .weak _start_trap15
            .weak _start_trap16
            .weak _start_trap17
            .weak _start_trap18
            .weak _start_trap19
            .weak _start_trap20
            .weak _start_trap21
            .weak _start_trap22
            .weak _start_trap23
            .weak _start_trap24
            .weak _start_trap25
            .weak _start_trap26
            .weak _start_trap27
            .weak _start_trap28
            .weak _start_trap29
            .weak _start_trap30
            .weak _start_trap31

            _start_trap1:
            _start_trap2:
            _start_trap3:
            _start_trap4:
            _start_trap5:
            _start_trap6:
            _start_trap7:
            _start_trap8:
            _start_trap9:
            _start_trap10:
            _start_trap11:
            _start_trap12:
            _start_trap13:
            _start_trap14:
            _start_trap15:
            _start_trap16:
            _start_trap17:
            _start_trap18:
            _start_trap19:
            _start_trap20:
            _start_trap21:
            _start_trap22:
            _start_trap23:
            _start_trap24:
            _start_trap25:
            _start_trap26:
            _start_trap27:
            _start_trap28:
            _start_trap29:
            _start_trap30:
            _start_trap31:
        default_start_trap:"#,
            // save space for trap handler in stack
            concat!("addi sp, sp, -", stringify!($TRAP_SIZE * $BYTES)),
            // save registers in the desired order
            $(concat!(stringify!($STORE), " ", stringify!($REG), ", ", stringify!($LOCATION * $BYTES), "(sp)"),)*
            // call rust trap handler
            "add a0, sp, zero
            jal ra, _start_trap_rust",
            // restore registers in the desired order
            $(concat!(stringify!($LOAD), " ", stringify!($REG), ", ", stringify!($LOCATION * $BYTES), "(sp)"),)*
            // free stack
            concat!("addi sp, sp, ", stringify!($TRAP_SIZE * $BYTES)),
        );
        cfg_global_asm!(
            // return from trap
            #[cfg(feature = "s-mode")]
            "sret",
            #[cfg(not(feature = "s-mode"))]
            "mret",
        );
    };
}

#[rustfmt::skip]
#[cfg(riscv32)]
trap_handler!(
    sw, lw, 4, 16,
    [(ra, 0), (t0, 1), (t1, 2), (t2, 3), (t3, 4), (t4, 5), (t5, 6), (t6, 7),
     (a0, 8), (a1, 9), (a2, 10), (a3, 11), (a4, 12), (a5, 13), (a6, 14), (a7, 15)]
);
#[rustfmt::skip]
#[cfg(riscv64)]
trap_handler!(
    sd, ld, 8, 16,
    [(ra, 0), (t0, 1), (t1, 2), (t2, 3), (t3, 4), (t4, 5), (t5, 6), (t6, 7),
     (a0, 8), (a1, 9), (a2, 10), (a3, 11), (a4, 12), (a5, 13), (a6, 14), (a7, 15)]
);

// Make sure there is an abort when linking
global_asm!(
    ".section .text.abort
     .globl abort
abort:
    j abort"
);
global_asm!(
    r#"
/*
    Interrupt vector table (_vector_table)
*/

.section .trap, "ax"
.weak _vector_table
.type _vector_table, @function

.option push
.balign 0x100
.option norelax
.option norvc

_vector_table:
    j default_start_trap
    j _start_trap1
    j _start_trap2
    j _start_trap3
    j _start_trap4
    j _start_trap5
    j _start_trap6
    j _start_trap7
    j _start_trap8
    j _start_trap9
    j _start_trap10
    j _start_trap11
    j _start_trap12
    j _start_trap13
    j _start_trap14
    j _start_trap15
    j _start_trap16
    j _start_trap17
    j _start_trap18
    j _start_trap19
    j _start_trap20
    j _start_trap21
    j _start_trap22
    j _start_trap23
    j _start_trap24
    j _start_trap25
    j _start_trap26
    j _start_trap27
    j _start_trap28
    j _start_trap29
    j _start_trap30
    j _start_trap31

.option pop
"#
);
