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

    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const UNUSED            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}
