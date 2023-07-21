#![no_std]
#![no_main]

extern crate panic_halt;

use ch32_riscv_rt::entry;

#[entry]
fn main() -> ! {
    // do something here
    loop {}
}
