// rustup target install riscv64imac-unknown-none-elf
// cargo build -p hello-world --target riscv64imac-unknown-none-elf --release

#![no_std]
#![no_main]

use panic_halt as _;
use sophgo_rom_rt::{entry, Peripherals};

#[entry]
fn main(p: Peripherals) -> ! {
    loop {
        p.uart0.write(b"Hello World from Rust!\n");
        // TODO fix Uart16550 crate bug; it doesn't block when UART FIFO is not empty
    }
}
