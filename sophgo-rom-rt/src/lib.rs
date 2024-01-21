#![no_std]
#![feature(naked_functions, asm_const)]

pub use sophgo_rom_rt_macros::entry;

use base_address::{BaseAddress, Static};
use sophgo_hal::{
    gpio::{Gpio, Input},
    pad::{Floating, GpioFunc, Pad},
};

/// Peripherals available on ROM start.
pub struct Peripherals {
    pub pinmux: sophgo_hal::PINMUX<Static<0x03001000>>,
    // TODO pub pads: sophgo_hal::gpio::Pads<Static<xxxx>>,
    /// General Purpose Input/Output 0.
    pub gpio0: sophgo_hal::GPIO<Static<0x03020000>>,
    /// General Purpose Input/Output 1.
    pub gpio1: sophgo_hal::GPIO<Static<0x03021000>>,
    /// General Purpose Input/Output 2.
    pub gpio2: sophgo_hal::GPIO<Static<0x03022000>>,
    /// General Purpose Input/Output 3.
    pub gpio3: sophgo_hal::GPIO<Static<0x03023000>>,

    // TODO pub pwm0: sophgo_hal::PWM<Static<0x03060000>>,
    // TODO pub pwm1: sophgo_hal::PWM<Static<0x03061000>>,
    // TODO pub pwm2: sophgo_hal::PWM<Static<0x03062000>>,
    // TODO pub pwm3: sophgo_hal::PWM<Static<0x03063000>>,
    // TODO pub timer: sophgo_hal::Timer<Static<0x030A0000>>,
    // TODO pub i2c0: sophgo_hal::I2C<Static<0x04000000>>,
    // TODO pub i2c1: sophgo_hal::I2C<Static<0x04010000>>,
    // TODO pub i2c2: sophgo_hal::I2C<Static<0x04020000>>,
    // TODO pub i2c3: sophgo_hal::I2C<Static<0x04030000>>,
    // TODO pub i2c4: sophgo_hal::I2C<Static<0x04040000>>,
    // TODO pub spi_nand: sophgo_hal::SPINand<Static<0x04060000>>,
    // TODO pub i2s0: sophgo_hal::I2S<Static<0x04100000>>,
    // TODO pub i2s1: sophgo_hal::I2S<Static<0x04110000>>,
    // TODO pub i2s2: sophgo_hal::I2S<Static<0x04120000>>,
    // TODO pub i2s3: sophgo_hal::I2S<Static<0x04130000>>,
    /// Universal Asynchronous Receiver/Transmitter 0.
    pub uart0: sophgo_hal::UART<Static<0x04140000>>,
    /// Universal Asynchronous Receiver/Transmitter 1.
    pub uart1: sophgo_hal::UART<Static<0x04150000>>,
    /// Universal Asynchronous Receiver/Transmitter 2.
    pub uart2: sophgo_hal::UART<Static<0x04160000>>,
    /// Universal Asynchronous Receiver/Transmitter 3.
    pub uart3: sophgo_hal::UART<Static<0x04170000>>,

    // TODO spi0: sophgo_hal::SPI<Static<0x04180000>>,
    // TODO spi1: sophgo_hal::SPI<Static<0x04190000>>,
    // TODO spi2: sophgo_hal::SPI<Static<0x041A0000>>,
    // TODO spi3: sophgo_hal::SPI<Static<0x041B0000>>,
    /// Universal Asynchronous Receiver/Transmitter 4.
    pub uart4: sophgo_hal::UART<Static<0x041C0000>>,
    // TODO sd0: sophgo_hal::SD<Static<0x04310000>>,
    // TODO sd1: sophgo_hal::SD<Static<0x04320000>>,
    // TODO usb: sophgo_hal::USB<Static<0x04340000>>,
    // TODO documents
    pub pwr_gpio: GpioPort<Static<0x05021000>>,
    pub pwr_pads: PwrPads<Static<0x05027000>>,
}

pub struct GpioPort<A: BaseAddress> {
    pub a0: Gpio<A, 0, Input>,
    pub a1: Gpio<A, 1, Input>,
    pub a2: Gpio<A, 2, Input>,
    // TODO a3 to a31
    base: A,
}

impl<A: BaseAddress> core::ops::Deref for GpioPort<A> {
    type Target = sophgo_hal::gpio::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

pub struct Pads<A: BaseAddress> {
    pub sd0_clk: Pad<A, 6, ()>, // TODO sd0_clk default function
                                // TODO ...
}

pub struct PwrPads<A: BaseAddress> {
    pub gpio1: Pad<A, 48, GpioFunc<Floating>>,
    pub gpio2: Pad<A, 49, GpioFunc<Floating>>,
    // TODO ...
}

#[cfg(target_arch = "riscv64")]
use core::arch::asm;

#[cfg(target_arch = "riscv64")]
const LEN_STACK: usize = 1 * 1024;

/// RISC-V program stack.
///
/// In standard RISC-V ABI specification, the stack grows downward and
/// the stack pointer is always kept 16-byte aligned.
#[repr(align(16))]
pub struct Stack<const N: usize>([u8; N]);

#[cfg(target_arch = "riscv64")]
#[naked]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn entry() -> ! {
    #[link_section = ".bss.uninit"]
    static mut STACK: Stack<LEN_STACK> = Stack([0; LEN_STACK]);
    asm!(
        ".option push
        .option arch, -c
            j       1f
        .option pop",
        ".word   0",  // resvered
        ".word   0",  // BL2 MSID
        ".word   0",  // BL2 version
        ".word   0",  //
        ".word   0",
        ".word   0",
        ".word   0",
        "1:",
        // configure mxstatus register
        // PM = 0b11 (Current privilege mode is Machine mode)
        // THEADISAEE = 1 (Enable T-Head ISA)
        // MAEE = 1 (Enable extended MMU attributes)
        // MHRD = 0 (Disable TLB hardware refill)
        // CLINTEE = 1 (CLINT usoft and utimer can be responded)
        // UCME = 1 (Enable extended cache instructions on U-mode)
        // MM = 1 (Enable hardware unaligned memory access)
        // PMP4K = 0 (read-only, PMP granularity 4KiB)
        // PMDM = 0 (allow performance counter on M-mode)
        // PMDS = 0 (allow performance counter on S-mode)
        // PMDU = 0 (allow performance counter on U-mode)
        "   li      t0, 0xc0638000
            csrw    0x7c0, t0",
        // invalid I-cache, D-cache, BHT and BTB by writing mcor register
        "   li      t2, 0x30013
            csrw    0x7c2, t2",
        // enable I-cache, D-cache by mhcr register
        "   csrsi   0x7c1, 0x3",
        // load stack address
        "   la      sp, {stack}
            li      t0, {hart_stack_size}
            add     sp, sp, t0",
        // clear bss segment
        "	la  	t1, sbss
        	la   	t2, ebss
    	1:  bgeu 	t1, t2, 1f
        	sd   	zero, 0(t1)
        	addi 	t1, t1, 8 
        	j    	1b
    	1:",
        "   call    {main}",
        stack = sym STACK,
        hart_stack_size = const LEN_STACK,
        main = sym main,
        options(noreturn)
    )
}

#[cfg(target_arch = "riscv64")]
extern "Rust" {
    // This symbol is generated by `#[entry]` macro
    fn main() -> !;
}
