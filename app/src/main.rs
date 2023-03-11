#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

// use cortex_m_semihosting::{debug, hprintln};
use bsp::Bsp;
use interfaces_hal::ReadByte;
use interfaces_hal::WriteByte;

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    let bsp = Bsp::new();


    bsp.lpuart1.write_byte(b'h');
    bsp.lpuart1.write_byte(b'a');
    bsp.lpuart1.write_byte(b'l');
    bsp.lpuart1.write_byte(b'l');
    bsp.lpuart1.write_byte(b'o');

    loop {
        let _byte = bsp.lpuart1.read_byte();
        bsp.lpuart1.write_byte(_byte);
        asm::nop();
    }
}
