// rustup target install riscv64imac-unknown-none-elf
// cargo build -p hello-world --target riscv64imac-unknown-none-elf --release

#![no_std]
#![no_main]

use embedded_io::Write;
use panic_halt as _;
use sophgo_rom_rt::{entry, Peripherals};

#[entry]
fn main(p: Peripherals) -> ! {
    let uart0_tx = p.pads.uart0_tx.into_function(&p.pinmux);
    let uart0_rx = p.pads.uart0_rx.into_function(&p.pinmux);

    let mut serial = p.uart0.serial(Default::default(), (uart0_tx, uart0_rx));

    loop {
        writeln!(serial, "Hello World from Rust!").ok();
        riscv::asm::delay(10_000_000);
    }
}
