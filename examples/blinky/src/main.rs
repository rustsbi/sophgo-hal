// rustup target install riscv64imac-unknown-none-elf
// cargo build -p blinky --target riscv64imac-unknown-none-elf --release

#![no_std]
#![no_main]

use core::arch::asm;

use panic_halt as _;
use sophgo_rom_rt::{entry, Peripherals};

#[entry]
fn main(p: Peripherals) -> ! {
    unsafe {
        p.rtc_gpio
            .direction
            .write(p.rtc_gpio.direction.read().set_output(2));
    }
    loop {
        unsafe {
            p.rtc_gpio.data.write(p.rtc_gpio.data.read() | (1 << 2));
        }
        for _ in 0..=1_000_000 {
            unsafe { asm!("nop") };
        }
        unsafe {
            p.rtc_gpio.data.write(p.rtc_gpio.data.read() & !(1 << 2));
        }
        for _ in 0..=1_000_000 {
            unsafe { asm!("nop") };
        }
    }
}
