#[derive(PartialEq, Clone, Copy)]
pub enum AddressingModes {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Operation {
    pub name: &'static str,
    pub instruction_cycles: u8,
    pub instruction_size: u8,
    pub instruction_page_cycles: u8,
    pub instruction_addressing_mode: AddressingModes,
}

impl Operation {
    pub fn new(
        name: &'static str,
        instruction_cycles: u8,
        instruction_size: u8,
        instruction_page_cycles: u8,
        instruction_addressing_mode: AddressingModes,
    ) -> Self {
        Operation {
            name,
            instruction_cycles,
            instruction_size,
            instruction_page_cycles,
            instruction_addressing_mode,
        }
    }
}
