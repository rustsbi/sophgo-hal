#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use panic_halt as _;
use sophgo_rom_rt::{entry, Peripherals};

#[entry]
fn main(p: Peripherals) -> ! {
    let pad_led = p.pwr_pads.gpio2.into_function(&p.pinmux);
    let mut led = p.pwr_gpio.a2.into_pull_up_output(pad_led);

    loop {
        led.set_high().unwrap();
        riscv::asm::delay(10_000_000);
        led.set_low().unwrap();
        riscv::asm::delay(10_000_000);
    }
}
