#![no_std]

pub mod gpio;
pub mod pad;
pub mod uart;

pub mod prelude {
    pub use crate::uart::UartExt as __sophgo_hal__uart__UartExt;
}
