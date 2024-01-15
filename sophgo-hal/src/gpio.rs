//! General Purpose Input/Output.

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

/// GPIO direction.
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
