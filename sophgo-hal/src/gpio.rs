//! General Purpose Input/Output.

use crate::{
    pad::{GpioFunc, Pad, PullUp},
    GPIO,
};
use base_address::BaseAddress;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, OutputPin};
use volatile_register::{RO, RW, WO};

/// GPIO registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Port A data register.
    pub data: RW<u32>,
    /// Port A data direction register.
    pub direction: RW<Direction>,
    _reserved0: [u8; 0x28],
    /// Interrupt enable register.
    pub interrupt_enable: RW<u32>,
    /// Interrupt mask register.
    pub interrupt_mask: RW<u32>,
    /// Interrupt level register.
    pub interrupt_level: RW<u32>,
    /// Interrupt polarity register.
    pub interrupt_polarity: RW<u32>,
    /// Interrupt status register.
    pub interrupt_status: RO<u32>,
    /// Raw interrupt status register.
    pub raw_interrupt_status: RO<u32>,
    /// Debounce enable register.
    pub debounce: RW<u32>,
    /// Port A clear interrupt register.
    pub interrupt_clear: WO<u32>,
    /// Port A external port register.
    pub external_port: RW<u32>,
    _reserved1: [u8; 0xC],
    /// Level-sensitive synchronization enable register.
    pub sync_level: RW<u32>,
}

/// GPIO direction register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Direction(u32);

impl Direction {
    /// Set GPIO direction to input.
    #[inline]
    pub fn set_input(self, n: u8) -> Self {
        Self(self.0 & !(1 << n))
    }
    /// Set GPIO direction to output.
    #[inline]
    pub fn set_output(self, n: u8) -> Self {
        Self(self.0 | (1 << n))
    }
    /// Check if GPIO direction is input.
    #[inline]
    pub fn is_input(self, n: u8) -> bool {
        self.0 & (1 << n) == 0
    }
    /// Check if GPIO direction is output.
    #[inline]
    pub fn is_output(self, n: u8) -> bool {
        self.0 & (1 << n) != 0
    }
}

/// Owned GPIO peripheral signal with mode type state.
pub struct Gpio<A: BaseAddress, const I: u8, M> {
    base: GPIO<A>,
    _mode: PhantomData<M>,
}

/// Input mode (type state).
pub struct Input;

/// Output mode (type state).
pub struct Output;

impl<A: BaseAddress, const I: u8, M> Gpio<A, I, M> {
    /// Configures the GPIO signal as a `GpioPad` operating as a pull up output.
    ///
    /// # Examples
    ///
    /// Gets ownership of pad from `PwrPads`, configures it as a pull up output
    /// to drive an LED.
    ///
    /// ```ignore
    /// let pad_led = p.pwr_pads.gpio2.into_function(&p.pinmux);
    /// let mut led = p.pwr_gpio.a2.into_pull_up_output(pad_led);
    /// ```
    #[inline]
    pub fn into_pull_up_output<A2: BaseAddress, const N: usize>(
        self,
        pad: Pad<A2, N, GpioFunc<PullUp>>,
    ) -> GpioPad<Gpio<A, I, Output>, Pad<A2, N, GpioFunc<PullUp>>> {
        unsafe {
            self.base.direction.modify(|w| w.set_output(I));
        }
        GpioPad {
            gpio: Gpio {
                base: self.base,
                _mode: PhantomData,
            },
            pad,
        }
    }
}

/// Ownership wrapper of a GPIO signal and a pad.
pub struct GpioPad<T, U> {
    gpio: T,
    pad: U,
}

impl<A: BaseAddress, A2: BaseAddress, const I: u8, const N: usize, M, T>
    GpioPad<Gpio<A, I, M>, Pad<A2, N, GpioFunc<T>>>
{
    /// Reconfigures the `GpioPad` to operate as a pull up output.
    #[inline]
    pub fn into_pull_up_output(self) -> GpioPad<Gpio<A, I, Output>, Pad<A2, N, GpioFunc<PullUp>>> {
        let (gpio, pad) = self.into_inner();
        gpio.into_pull_up_output(pad.into_gpio_pull_up())
    }
}

impl<T, U> GpioPad<T, U> {
    /// Unwraps the ownership structure, returning the GPIO signal and the pad.
    #[inline]
    pub fn into_inner(self) -> (T, U) {
        (self.gpio, self.pad)
    }
}

impl<T, U> ErrorType for GpioPad<T, U> {
    type Error = core::convert::Infallible;
}

impl<A: BaseAddress, const I: u8, U> OutputPin for GpioPad<Gpio<A, I, Output>, U> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.gpio.base.data.modify(|w| w & !(1 << I));
        }
        Ok(())
    }

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.gpio.base.data.modify(|w| w | (1 << I));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, data), 0x00);
        assert_eq!(offset_of!(RegisterBlock, direction), 0x04);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x30);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mask), 0x34);
        assert_eq!(offset_of!(RegisterBlock, interrupt_level), 0x38);
        assert_eq!(offset_of!(RegisterBlock, interrupt_polarity), 0x3C);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0x40);
        assert_eq!(offset_of!(RegisterBlock, raw_interrupt_status), 0x44);
        assert_eq!(offset_of!(RegisterBlock, debounce), 0x48);
        assert_eq!(offset_of!(RegisterBlock, interrupt_clear), 0x4C);
        assert_eq!(offset_of!(RegisterBlock, external_port), 0x50);
        assert_eq!(offset_of!(RegisterBlock, sync_level), 0x60);
    }
}
