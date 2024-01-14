#![no_std]

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
