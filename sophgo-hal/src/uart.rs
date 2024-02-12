//! Universal Asynchronous Receiver/Transmitter.

use crate::pad::{Pad, UartFunc};
use volatile_register::{RO, RW, WO};

/// Universal Asynchoronous Receiver/Transmitter registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Receive Buffer,Transmit Holding or Divisor Latch Low byte Register.
    pub rbr_thr_dll: RW<u32>,
    /// Interrupt Enable or Divisor Latch high byte Register.
    pub ier_dlh: RW<u32>,
    /// FIFO Control or Interrupt Identification Register.
    pub fcr_iir: RW<u32>,
    /// Line Control Register.
    pub lcr: RW<LCR>,
    /// Modem Control Register.
    pub mcr: RW<MCR>,
    /// Line Status Register.
    pub lsr: RO<LSR>,
    /// Modem Status Register.
    pub msr: RO<u32>,
    _reserved0: [u8; 0x4],
    /// Low Power Divisor Latch (Low) Register.
    pub lpdll: RW<u32>,
    /// Low Power Divisor Latch (High) Register.
    pub lpdlh: RW<u32>,
    _reserved1: [u8; 0x8],
    /// Shadow Receive/Trasnmit Buffer Register.
    pub srbr_sthr: RW<u32>,
    _reserved2: [u8; 0x3C],
    /// FIFO Access Register.
    pub far: RW<u32>,
    /// Transmit FIFO Read.
    pub tfr: RW<u32>,
    /// Receive FIFO Write.
    pub rfw: RW<u32>,
    /// UART Status Register.
    pub usr: RO<u32>,
    /// Transmit FIFO Level.
    pub tfl: RO<u32>,
    /// Receive FIFO Level.
    pub rfl: RO<u32>,
    /// Software Reset Register.
    pub srr: WO<u32>,
    /// Shadow Request to Send.
    pub srts: RW<u32>,
    /// Shadow Break Control Register.
    pub sbcr: RW<u32>,
    /// Shadow DMA Mode.
    pub sdmam: RW<u32>,
    /// Shadow FIFO Enable.
    pub sfe: RW<u32>,
    /// Shadow RCVR Trigger.
    pub srt: RW<u32>,
    /// Shadow TX Empty Trigger.
    pub stet: RW<u32>,
    /// Halt TX.
    pub htx: RW<u32>,
    /// DMA Software Acknowledg.
    pub dmasa: RW<u32>,
}

/// Line Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct LCR(u32);

impl LCR {
    const WROD_LENGTH: u32 = 0b11;
    const STOP_BIT: u32 = 1 << 2;
    const PARITY: u32 = 0b11 << 3;
    const DIVISOR_LATCH_ACCESS: u32 = 1 << 7;

    /// Set word length.
    #[inline]
    pub fn set_word_length(self, len: WordLength) -> Self {
        Self((self.0 & !(Self::WROD_LENGTH)) | (len as u32))
    }
    /// Get word length.
    #[inline]
    pub fn word_length(self) -> WordLength {
        match self.0 & (Self::WROD_LENGTH) {
            0b00 => WordLength::Five,
            0b01 => WordLength::Six,
            0b10 => WordLength::Seven,
            0b11 => WordLength::Eight,
            _ => unreachable!(),
        }
    }
    /// Set stop bit.
    #[inline]
    pub fn set_stop_bit(self, stop: StopBits) -> Self {
        Self((self.0 & !(Self::STOP_BIT)) | ((stop as u32) << 2))
    }
    /// Get stop bit.
    #[inline]
    pub fn stop_bit(self) -> StopBits {
        match (self.0 & (Self::STOP_BIT)) >> 2 {
            0 => StopBits::One,
            1 => StopBits::OnePointFiveOrTwo,
            _ => unreachable!(),
        }
    }
    /// Set parity check.
    #[inline]
    pub fn set_parity(self, parity: Parity) -> Self {
        Self((self.0 & !(Self::PARITY)) | ((parity as u32) << 3))
    }
    /// Get parity check.
    #[inline]
    pub fn parity(self) -> Parity {
        match (self.0 & (Self::PARITY)) >> 3 {
            0b00 => Parity::None,
            0b01 => Parity::Odd,
            0b11 => Parity::Even,
            _ => unreachable!(),
        }
    }
    /// Enable divisor latch access.
    #[inline]
    pub fn enable_divisor_latch_access(self) -> Self {
        Self(self.0 | Self::DIVISOR_LATCH_ACCESS)
    }
    /// Disable divisor latch access.
    #[inline]
    pub fn disable_divisor_latch_access(self) -> Self {
        Self(self.0 & !(Self::DIVISOR_LATCH_ACCESS))
    }
    /// Check if divisor latch access is enabled.
    #[inline]
    pub fn is_divisor_latch_access_enabled(self) -> bool {
        self.0 & (Self::DIVISOR_LATCH_ACCESS) != 0
    }
}

/// Modem Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MCR(u32);

impl MCR {
    const REQUEST_TO_SEND: u32 = 1 << 1;
    const AUTO_FLOW_CONTROL: u32 = 1 << 5;

    /// Enable request to send.
    #[inline]
    pub fn enable_request_to_send(self) -> Self {
        Self(self.0 | Self::REQUEST_TO_SEND)
    }
    /// Disable request to send.
    #[inline]
    pub fn disable_request_to_send(self) -> Self {
        Self(self.0 & !(Self::REQUEST_TO_SEND))
    }
    /// Check if request to send is enabled.
    #[inline]
    pub fn is_request_to_send_enabled(self) -> bool {
        self.0 & (Self::REQUEST_TO_SEND) != 0
    }
    /// Enable auto flow control.
    #[inline]
    pub fn enable_auto_flow_control(self) -> Self {
        Self(self.0 | Self::AUTO_FLOW_CONTROL)
    }
    /// Disable auto flow control.
    #[inline]
    pub fn disable_auto_flow_control(self) -> Self {
        Self(self.0 & !(Self::AUTO_FLOW_CONTROL))
    }
    /// Check if auto flow control is enabled.
    #[inline]
    pub fn is_auto_flow_control_enabled(self) -> bool {
        self.0 & (Self::AUTO_FLOW_CONTROL) != 0
    }
}

/// Line Status Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct LSR(u32);

impl LSR {
    const TRANSMIT_HOLDING_EMPTY: u32 = 1 << 5;
    const TRANSMIT_EMPTY: u32 = 1 << 6;

    /// Check if transmit holding register is empty.
    #[inline]
    pub fn is_transmit_holding_empty(self) -> bool {
        self.0 & (Self::TRANSMIT_HOLDING_EMPTY) != 0
    }
    /// Check if transmit FIFO is empty.
    #[inline]
    pub fn is_transmit_empty(self) -> bool {
        self.0 & (Self::TRANSMIT_EMPTY) != 0
    }
}

/// Managed serial peripheral.
pub struct Serial<T, PADS> {
    uart: T,
    pads: PADS,
}

impl<T, PADS> Serial<T, PADS> {
    /// Release serial instance and return its peripheral and pads.
    #[inline]
    pub fn free(self) -> (T, PADS) {
        (self.uart, self.pads)
    }
}

pub trait UartExt<const I: usize>: AsRef<RegisterBlock> + Sized {
    #[inline]
    fn serial<PADS>(self, config: Config, pads: PADS) -> Serial<Self, PADS>
    where
        PADS: Pads<I>,
    {
        let uart = self.as_ref();
        // TODO clock source and baudrate
        let interval = 14;
        unsafe {
            uart.lcr.modify(|w| w.enable_divisor_latch_access());
            uart.lpdll.write(interval & 0xff);
            uart.lpdlh.write((interval >> 8) & 0xff);
            uart.lcr.modify(|w| w.disable_divisor_latch_access());
        }

        unsafe {
            uart.lcr.modify(|w| {
                w.set_stop_bit(config.stop_bits)
                    .set_word_length(config.word_length)
                    .set_parity(config.parity)
            });
        }

        Serial { uart: self, pads }
    }
}

impl embedded_io::Error for Error {
    #[inline(always)]
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl<T: AsRef<RegisterBlock>, PADS> embedded_io::ErrorType for Serial<T, PADS> {
    type Error = Error;
}

impl<T: AsRef<RegisterBlock>, PADS> embedded_io::Write for Serial<T, PADS> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let uart = self.uart.as_ref();
        let mut len = 0;
        for c in buf {
            while !uart.lsr.read().is_transmit_holding_empty() {
                core::hint::spin_loop();
            }
            unsafe { uart.rbr_thr_dll.write(*c as u32) };
            len += 1;
        }
        Ok(len)
    }
    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        let uart = self.uart.as_ref();
        while !uart.lsr.read().is_transmit_empty() {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

impl<T: AsRef<RegisterBlock>, PADS> embedded_io::Read for Serial<T, PADS> {
    #[inline]
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize, Self::Error> {
        todo!()
    }
}

/// Serial configuration.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Config {
    /// Parity settings.
    pub parity: Parity,
    /// Serial stop bits.
    pub stop_bits: StopBits,
    /// Data word length.
    pub word_length: WordLength,
}

impl Default for Config {
    /// Serial configuration defaults to 8-bit word, no parity check, 1 stop bit, LSB first.
    #[inline]
    fn default() -> Self {
        Config {
            parity: Parity::None,
            stop_bits: StopBits::One,
            word_length: WordLength::Eight,
        }
    }
}

/// Parity check.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Parity {
    /// No parity check.
    None,
    /// Odd parity bit.
    Odd = 0b01,
    /// Even parity bit.
    Even = 0b11,
}

/// Stop bits.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StopBits {
    /// 1 stop bit.
    One,
    /// 1.5 stop bits when word length is 5 bits else 2 stop bits.
    OnePointFiveOrTwo,
}

/// Word length.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WordLength {
    /// Five bits per word.
    Five,
    /// Six bits per word.
    Six,
    /// Seven bits per word.
    Seven,
    /// Eight bits per word.
    Eight,
}

/// Serial error.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Framing error.
    Framing,
    /// Noise error.
    Noise,
    /// RX buffer overrun.
    Overrun,
    /// Parity check error.
    Parity,
}

/// Valid UART pads.
pub trait Pads<const U: usize> {
    /// Checks if this pin configuration includes Request-to-Send feature.
    const RTS: bool;
    /// Checks if this pin configuration includes Clear-to-Send feature.
    const CTS: bool;
    /// Checks if this pin configuration includes Transmit feature.
    const TXD: bool;
    /// Checks if this pin configuration includes Receive feature.
    const RXD: bool;
}

impl<T1, T2> Pads<0> for (Pad<T1, 18, UartFunc<0>>, Pad<T2, 19, UartFunc<0>>) {
    const RTS: bool = false;
    const CTS: bool = false;
    const TXD: bool = true;
    const RXD: bool = true;
}

impl<T1, T2> Pads<1> for (Pad<T1, 28, UartFunc<1>>, Pad<T2, 29, UartFunc<1>>) {
    const RTS: bool = false;
    const CTS: bool = false;
    const TXD: bool = true;
    const RXD: bool = true;
}
