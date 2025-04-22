#![no_std]

pub use sophgo_rom_rt_macros::entry;

/// Rom runtime prelude.
pub mod prelude {
    pub use crate::{entry, Peripherals};
    pub use sophgo_hal::prelude::*;
}

#[macro_use]
mod macros;
use sophgo_hal::{
    gpio::{Gpio, Input},
    pad::{FMux, Floating, GpioFunc, Pad, PadConfigs, PinMux, PwrPadConfigs, UartFunc},
};

/// Peripherals available on ROM start.
pub struct Peripherals {
    /// Pad function multiplexer peripheral.
    pub pinmux: PINMUX,
    // TODO pub pads: sophgo_hal::gpio::Pads<Static<xxxx>>,
    /// General Purpose Input/Output 0.
    pub gpio0: GPIO0,
    /// General Purpose Input/Output 1.
    pub gpio1: GPIO1,
    /// General Purpose Input/Output 2.
    pub gpio2: GPIO2,
    /// General Purpose Input/Output 3.
    pub gpio3: GPIO3,

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
    pub uart0: UART0,
    /// Universal Asynchronous Receiver/Transmitter 1.
    pub uart1: UART1,
    /// Universal Asynchronous Receiver/Transmitter 2.
    pub uart2: UART2,
    /// Universal Asynchronous Receiver/Transmitter 3.
    pub uart3: UART3,

    // TODO spi0: sophgo_hal::SPI<Static<0x04180000>>,
    // TODO spi1: sophgo_hal::SPI<Static<0x04190000>>,
    // TODO spi2: sophgo_hal::SPI<Static<0x041A0000>>,
    // TODO spi3: sophgo_hal::SPI<Static<0x041B0000>>,
    /// Universal Asynchronous Receiver/Transmitter 4.
    pub uart4: UART4,
    // TODO sd0: sophgo_hal::SD<Static<0x04310000>>,
    // TODO sd1: sophgo_hal::SD<Static<0x04320000>>,
    // TODO usb: sophgo_hal::USB<Static<0x04340000>>,
    // TODO documents
    /// SoC pads.
    pub pads: Pads<PINMUX>,
    /// Low-power Domain General Purpose Input/Output signal port.
    pub pwr_gpio: GpioPort<PWR_GPIO>,
    /// Low-power Domain SoC pads.
    pub pwr_pads: PwrPads<PWR_PINMUX>,
}

soc! {
    /// Pad function multiplexer peripheral.
    pub struct PINMUX => 0x03001000, PinMux;
    /// General Purpose Input/Output peripheral 0.
    pub struct GPIO0 => 0x03020000, sophgo_hal::gpio::RegisterBlock;
    /// General Purpose Input/Output peripheral 1.
    pub struct GPIO1 => 0x03021000, sophgo_hal::gpio::RegisterBlock;
    /// General Purpose Input/Output peripheral 2.
    pub struct GPIO2 => 0x03022000, sophgo_hal::gpio::RegisterBlock;
    /// General Purpose Input/Output peripheral 3.
    pub struct GPIO3 => 0x03023000, sophgo_hal::gpio::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter peripheral 0.
    pub struct UART0 => 0x04140000, sophgo_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter peripheral 1.
    pub struct UART1 => 0x04150000, sophgo_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter peripheral 2.
    pub struct UART2 => 0x04160000, sophgo_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter peripheral 3.
    pub struct UART3 => 0x04170000, sophgo_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter peripheral 4.
    pub struct UART4 => 0x041C0000, sophgo_hal::uart::RegisterBlock;
    /// Low-power Domain General Purpose Input/Output peripheral.
    pub struct PWR_GPIO => 0x05021000, sophgo_hal::gpio::RegisterBlock;
    /// Low-power Domain pad configuration peripheral.
    pub struct PWR_PINMUX => 0x05027000, PadConfigs, PwrPadConfigs;
}

impl AsRef<FMux> for PINMUX {
    #[inline(always)]
    fn as_ref(&self) -> &FMux {
        &<Self as AsRef<PinMux>>::as_ref(self).fmux
    }
}

impl AsRef<PadConfigs> for PINMUX {
    #[inline(always)]
    fn as_ref(&self) -> &PadConfigs {
        &<Self as AsRef<PinMux>>::as_ref(self).config
    }
}

/// General Purpose Input/Output signal port.
pub struct GpioPort<T> {
    pub a0: Gpio<T, 0, Input>,
    pub a1: Gpio<T, 1, Input>,
    pub a2: Gpio<T, 2, Input>,
    // TODO a3 to a31
}

/// SoC pads.
pub struct Pads<T> {
    pub sd0_clk: Pad<T, 6, ()>, // TODO sd0_clk default function
    pub uart0_tx: Pad<T, 18, UartFunc<0>>,
    pub uart0_rx: Pad<T, 19, UartFunc<0>>,
    pub i2c0_scl: Pad<T, 28, ()>,
    pub i2c0_sda: Pad<T, 29, ()>,
    // TODO ...
}

/// Low-power Domain SoC pads.
pub struct PwrPads<T> {
    pub gpio1: Pad<T, 48, GpioFunc<Floating>>,
    pub gpio2: Pad<T, 49, GpioFunc<Floating>>,
    // TODO ...
}

impl sophgo_hal::uart::UartExt<0> for UART0 {}
impl sophgo_hal::uart::UartExt<1> for UART1 {}
impl sophgo_hal::uart::UartExt<2> for UART2 {}
impl sophgo_hal::uart::UartExt<3> for UART3 {}
impl sophgo_hal::uart::UartExt<4> for UART4 {}

#[cfg(target_arch = "riscv64")]
use core::arch::naked_asm;

#[cfg(target_arch = "riscv64")]
const LEN_STACK: usize = 1 * 1024;

/// RISC-V program stack.
///
/// In standard RISC-V ABI specification, the stack grows downward and
/// the stack pointer is always kept 16-byte aligned.
#[repr(align(16))]
pub struct Stack<const N: usize>([u8; N]);

#[cfg(target_arch = "riscv64")]
#[unsafe(naked)]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn entry() -> ! {
    #[link_section = ".bss.uninit"]
    static mut STACK: Stack<LEN_STACK> = Stack([0; LEN_STACK]);
    naked_asm!(
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
    )
}

#[cfg(target_arch = "riscv64")]
extern "Rust" {
    // This symbol is generated by `#[entry]` macro
    fn main() -> !;
}
