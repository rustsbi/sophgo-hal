//! Silicon pad multiplexer and configurations.

use base_address::BaseAddress;
use core::marker::PhantomData;
use volatile_register::RW;

/// The `PINMUX` pad multiplexer peripheral.
#[repr(C)]
pub struct PinMux {
    /// Pad function multiplexer registers for all the pads.
    pub fmux: FMux,
    // TODO paddings
    /// Non-RTC domain pad configurations.
    pub config: PadConfigs,
}

impl AsRef<FMux> for PinMux {
    #[inline(always)]
    fn as_ref(&self) -> &FMux {
        &self.fmux
    }
}

/// Pad function multiplexer registers for all the pads.
#[repr(C)]
pub struct FMux {
    _reserved0: [u8; 0x40],
    /// UART-0 TX pad function.
    pub uart0_tx: RW<u32>,
    /// UART-0 RX pad function.
    pub uart0_rx: RW<u32>,
    _reserved1: [u8; 0x28],
    /// I2C-0 Serial Clock (SCL) pad function.
    pub i2c0_scl: RW<u32>,
    /// I2C-0 Serial Data (SDA) pad function.
    pub i2c0_sda: RW<u32>,
    _reserved2: [u8; 0x34],
    /// Power (RTC) domain GPIO-2 pad function.
    pub pwr_gpio2: RW<u32>,
    // TODO other fields and padding
    _reserved3: [u8; 0x1750],
}

impl FMux {
    /// Gets the pad function multiplexer register for the given pad number `N`.
    #[inline]
    pub fn fmux<const N: usize>(&self) -> &RW<u32> {
        match N {
            18 => &self.uart0_tx,
            19 => &self.uart0_rx,
            28 => &self.i2c0_scl,
            29 => &self.i2c0_sda,
            49 => &self.pwr_gpio2,
            _ => todo!(),
        }
    }
}

/// Non-RTC domain pad configurations.
#[repr(C)]
pub struct PadConfigs {
    _reserved0: [u8; 0x10C],
    /// Non-RTC domain UART-0 TX pad configurations.
    pub uart0_tx: RW<PadConfig>,
    /// Non-RTC domain UART-0 RX pad configurations.
    pub uart0_rx: RW<PadConfig>,
    _reserved1: [u8; 0x28],
    /// Non-RTC domain i2c-0 SCL pad configurations.
    pub i2c0_scl: RW<PadConfig>,
    /// Non-RTC domain i2c-0 SDA pad configurations.
    pub i2c0_sda: RW<PadConfig>,
}

impl PadConfigs {
    /// Gets the pad configuration register for the given pad number `N`.
    ///
    /// `N` must be number of a pad in the non-RTC domain.
    #[inline]
    const fn pad_config<const N: usize>(&self) -> &RW<PadConfig> {
        match N {
            18 => &self.uart0_tx,
            19 => &self.uart0_rx,
            28 => &self.i2c0_scl,
            29 => &self.i2c0_sda,
            // if not a non-RTC pad, return unimplemented!()
            _ => todo!(),
        }
    }
}

/// Power (RTC) domain pad configurations.
#[repr(C)]
pub struct PwrPadConfigs {
    _reserved0: [u8; 0x34],
    /// Power (RTC) domain GPIO-2 pad configuration.
    pub pwr_gpio2: RW<PadConfig>,
}

impl PwrPadConfigs {
    /// Gets the pad configuration register for the given pad number `N`.
    ///
    /// `N` must be number of a pad in the power (RTC) domain.
    #[inline]
    const fn pad_config<const N: usize>(&self) -> &RW<PadConfig> {
        match N {
            49 => &self.pwr_gpio2,
            // if not a power pad, return unimplemented!()
            _ => todo!(),
        }
    }
}

/// Pad configuration register for all the pads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PadConfig(u32);

impl PadConfig {
    const PULL: u32 = 0b11 << 2;
    const DRIVE: u32 = 0b11 << 5;
    const SCHMITT: u32 = 0b11 << 8;
    const BUS_HOLDER: u32 = 1 << 10;
    const SLEW_RATE_LIMIT: u32 = 1 << 11;

    /// Get pull direction of current pad.
    #[inline]
    pub const fn pull(self) -> Pull {
        match (self.0 & Self::PULL) >> 4 {
            0 => Pull::None,
            1 => Pull::Up,
            2 => Pull::Down,
            _ => unreachable!(),
        }
    }
    /// Set pull direction of current pad.
    #[inline]
    pub const fn set_pull(self, val: Pull) -> Self {
        Self((self.0 & !Self::PULL) | ((val as u32) << 4))
    }
    #[inline]
    pub fn set_drive(self, drive: u8) -> Self {
        Self((self.0 & !Self::DRIVE) | ((drive as u32) << 5))
    }
    #[inline]
    pub fn set_schmitt(self, schmitt: u8) -> Self {
        Self((self.0 & !Self::SCHMITT) | ((schmitt as u32) << 8))
    }
    #[inline]
    pub fn enable_bus_holder(self) -> Self {
        Self(self.0 | Self::BUS_HOLDER)
    }
    #[inline]
    pub fn disable_bus_holder(self) -> Self {
        Self(self.0 & !Self::BUS_HOLDER)
    }
    #[inline]
    pub fn enable_slew_rate_limit(self) -> Self {
        Self(self.0 | Self::SLEW_RATE_LIMIT)
    }
    #[inline]
    pub fn disable_slew_rate_limit(self) -> Self {
        Self(self.0 & !Self::SLEW_RATE_LIMIT)
    }
}

/// Ownership of a pad with function type state.
pub struct Pad<A: BaseAddress, const N: usize, F> {
    base: A,
    _function: PhantomData<F>,
}

impl<A: BaseAddress, const N: usize, F> Pad<A, N, F> {
    /// Converts the function of this pad.
    #[inline]
    pub fn into_function<F2: Function>(self, fmux: impl AsRef<FMux>) -> Pad<A, N, F2> {
        unsafe { fmux.as_ref().fmux::<N>().write(F2::fmux::<N>()) };
        unsafe { self.pad_config().modify(|w| w.set_pull(F2::PULL)) };
        Pad {
            base: self.base,
            _function: PhantomData,
        }
    }
    #[inline]
    pub fn pad_config(&self) -> &RW<PadConfig> {
        match N {
            // TODO in range of power pads ...
            49 => unsafe { &*(self.base.ptr() as *const PwrPadConfigs) }.pad_config::<N>(),
            // TODO in range of conventional pads ...
            18..=19 | 28..=29 => {
                unsafe { &*(self.base.ptr() as *const PadConfigs) }.pad_config::<N>()
            }
            // .. => { ... }
            _ => todo!(),
        }
    }
}

impl<A: BaseAddress, const N: usize, T> Pad<A, N, GpioFunc<T>> {
    #[inline]
    pub(crate) fn into_gpio_pull_up(self) -> Pad<A, N, GpioFunc<PullUp>> {
        unsafe { self.pad_config().modify(|w| w.set_pull(Pull::Up)) };
        Pad {
            base: self.base,
            _function: PhantomData,
        }
    }
}

/// GPIO function with a pull mode (type state).
pub struct GpioFunc<T> {
    _pull: PhantomData<T>,
}

/// Pulled down as pull mode (type state).
pub struct PullDown;

/// Pulled up as pull mode (type state).
pub struct PullUp;

/// Floating as pull mode (type state).
pub struct Floating;

/// UART function (type state).
pub struct UartFunc<const I: usize>;

/// Trait for all valid pad functions.
pub trait Function {
    /// Pull direction associated with this pad function.
    const PULL: Pull;
    /// Function ID for the `fmux` multiplexer register.
    fn fmux<const N: usize>() -> u32;
}

impl Function for GpioFunc<Floating> {
    const PULL: Pull = Pull::None;
    #[inline]
    fn fmux<const N: usize>() -> u32 {
        gpio_fmux::<N>()
    }
}

impl Function for GpioFunc<PullUp> {
    const PULL: Pull = Pull::Up;
    #[inline]
    fn fmux<const N: usize>() -> u32 {
        gpio_fmux::<N>()
    }
}

impl Function for GpioFunc<PullDown> {
    const PULL: Pull = Pull::Down;
    #[inline]
    fn fmux<const N: usize>() -> u32 {
        gpio_fmux::<N>()
    }
}

const fn gpio_fmux<const N: usize>() -> u32 {
    match N {
        1..=45 | 51..=86 => 3,
        47..=49 => 0,
        _ => unimplemented!(),
    }
}

impl<const I: usize> Function for UartFunc<I> {
    const PULL: Pull = Pull::Up;
    #[inline]
    fn fmux<const N: usize>() -> u32 {
        uart_fmux::<N, I>()
    }
}

const fn uart_fmux<const N: usize, const I: usize>() -> u32 {
    match I {
        0 => match N {
            18..=19 => 0,
            _ => unimplemented!(),
        },
        1 => match N {
            28..=29 => 1,
            _ => unimplemented!(),
        },
        2 => match N {
            28..=29 => 2,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

/// Pad internal pull direction values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pull {
    /// No internal pulls.
    None = 0,
    /// Internally pulled up.
    Up = 1,
    /// Internally pulled down.
    Down = 2,
}

#[cfg(test)]
mod tests {
    use super::{FMux, PadConfigs, PinMux};
    use memoffset::offset_of;

    #[test]
    fn struct_pinmux_offset() {
        assert_eq!(offset_of!(PinMux, fmux), 0x00);
        assert_eq!(offset_of!(PinMux, config), 0x1800);
    }

    #[test]
    fn struct_fmux_offset() {
        assert_eq!(offset_of!(FMux, uart0_tx), 0x40);
        assert_eq!(offset_of!(FMux, uart0_rx), 0x44);
        assert_eq!(offset_of!(FMux, i2c0_scl), 0x70);
        assert_eq!(offset_of!(FMux, i2c0_sda), 0x74);
        assert_eq!(offset_of!(FMux, pwr_gpio2), 0xAC);
    }

    #[test]
    fn struct_pad_configs_offset() {
        assert_eq!(offset_of!(PadConfigs, uart0_tx), 0x190C - 0x1800);
        assert_eq!(offset_of!(PadConfigs, uart0_rx), 0x1910 - 0x1800);
        assert_eq!(offset_of!(PadConfigs, i2c0_scl), 0x193C - 0x1800);
        assert_eq!(offset_of!(PadConfigs, i2c0_sda), 0x1940 - 0x1800);
    }
}
