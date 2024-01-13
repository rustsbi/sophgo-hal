// rustup target install riscv64imac-unknown-none-elf
// cargo build -p hello-world --target riscv64imac-unknown-none-elf --release

#![no_std]
#![no_main]

use panic_halt as _;
use sophgo_rom_rt::entry;

#[entry]
fn main() -> ! {
    let uart = unsafe { &*(0x04140000 as *const uart16550::Uart16550<u32>) };
    loop {
        uart.write(b"Hello World from Rust!\n");
    }
}
