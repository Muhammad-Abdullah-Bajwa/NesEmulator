use crate::constants::BitMasks;
use bitflags::bitflags;

bitflags! {
    /*
            7  bit  0
            ---- ----
            NV1B DIZC
            |||| ||||
            |||| |||+- Carry
            |||| ||+-- Zero
            |||| |+--- Interrupt Disable
            |||| +---- Decimal
            |||+------ (No CPU effect; see: the B flag)
            ||+------- (No CPU effect; always pushed as 1)
            |+-------- Overflow
            +--------- Negative
     */
    #[derive(PartialEq, Clone, Copy)]
    pub struct CpuFlags: u8 {
        const CARRY             = BitMasks::ZERO;
        const ZERO              = BitMasks::FIRST;
        const INTERRUPT_DISABLE = BitMasks::SECOND;
        const DECIMAL_MODE      = BitMasks::THIRD;
        const BREAK             = BitMasks::FOURTH;
        const UNUSED            = BitMasks::FIFTH;
        const OVERFLOW          = BitMasks::SIXTH;
        const NEGATIVE          = BitMasks::SEVENTH;
    }
}
