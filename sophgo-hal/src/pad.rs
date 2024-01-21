//! Silicon pad multiplexer and configurations.

use base_address::BaseAddress;
use core::marker::PhantomData;
use volatile_register::RW;

/// The `PINMUX` pad multiplexer peripheral.
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
pub struct FMux {
    _reserved0: [u8; 0xAC],
    /// Power (RTC) domain GPIO-2 pad function.
    pub pwr_gpio2: RW<u32>,
    // TODO other fields and padding
}

impl FMux {
    /// Gets the pad function multiplexer register for the given pad number `N`.
    #[inline]
    pub fn fmux<const N: usize>(&self) -> &RW<u32> {
        match N {
            49 => &self.pwr_gpio2,
            _ => todo!(),
        }
    }
}

/// Non-RTC domain pad configurations.
pub struct PadConfigs {
    // TODO
}

// TODO fn pad_config in impl PadConfigs

/// Power (RTC) domain pad configurations.
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
    fn pad_config(&self) -> &RW<PadConfig> {
        match N {
            // TODO in range of power pads ...
            49 => unsafe { &*(self.base.ptr() as *const PwrPadConfigs) }.pad_config::<N>(),
            // TODO in range of conventional pads ...
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
