#![no_std]

pub mod gpio;
pub mod pad;

use core::ops;

use base_address::BaseAddress;

/// Universal Asynchronous Receiver/Transmitter.
pub struct UART<A: BaseAddress> {
    base: A,
}

impl<A: BaseAddress> ops::Deref for UART<A> {
    type Target = uart16550::Uart16550<u32>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

/// General Purpose Input/Output.
pub struct GPIO<A: BaseAddress> {
    base: A,
}

impl<A: BaseAddress> ops::Deref for GPIO<A> {
    type Target = gpio::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

/// Pad function multiplexer peripheral.
pub struct PINMUX<A: BaseAddress> {
    base: A,
}

impl<A: BaseAddress> ops::Deref for PINMUX<A> {
    type Target = pad::PinMux;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

impl<A: BaseAddress> AsRef<pad::FMux> for PINMUX<A> {
    #[inline(always)]
    fn as_ref(&self) -> &pad::FMux {
        &self.fmux
    }
}
