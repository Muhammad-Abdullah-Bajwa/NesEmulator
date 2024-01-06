use std::vec;

use crate::operation::{AddressingModes, Operation};
use lazy_static::lazy_static;

pub const RESET_STACK_ADDRESS: u8 = 0xFD;
pub const STATUS_REGISTER_INITIAL: u8 = 0b00100100;
pub const RAM_START: u16 = 0x0000;
pub const RAM_MIRRORS_END: u16 = 0x1FFF;
pub const PPU_REGISTERS: u16 = 0x2000;
pub const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
pub const RAM_SIZE: u16 = 2048;

lazy_static! {
    pub static ref OPERATION_INFORMATION: Vec<Option<Operation>> = {
        let mut vector = Vec::with_capacity(256);

        vector[0x00] = Some(Operation::new("BRK", 7, 1, 0, AddressingModes::Implicit));
        vector[0x01] = Some(Operation::new(
            "ORA",
            6,
            2,
            0,
            AddressingModes::IndexedIndirect,
        ));
        vector[0x02] = None;
        vector[0x03] = None;
        vector[0x04] = None;
        vector[0x05] = Some(Operation::new("ORA", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0x06] = Some(Operation::new("ASL", 5, 2, 0, AddressingModes::ZeroPage));
        vector[0x07] = None;
        vector[0x08] = Some(Operation::new("PHP", 3, 1, 0, AddressingModes::Implicit));
        vector[0x09] = Some(Operation::new("ORA", 2, 2, 0, AddressingModes::Immediate));
        vector[0x0A] = Some(Operation::new("ASL", 2, 1, 0, AddressingModes::Accumulator));
        vector[0x0B] = None;
        vector[0x0C] = None;
        vector[0x0D] = Some(Operation::new("ORA", 4, 3, 0, AddressingModes::Absolute));
        vector[0x0E] = Some(Operation::new("ASL", 6, 3, 0, AddressingModes::Absolute));
        vector[0x0F] = None;

        vector[0x10] = Some(Operation::new("BPL", 2, 2, 1, AddressingModes::Relative));
        vector[0x11] = Some(Operation::new(
            "ORA",
            5,
            2,
            1,
            AddressingModes::IndirectIndexed,
        ));
        vector[0x12] = None;
        vector[0x13] = None;
        vector[0x14] = None;
        vector[0x15] = Some(Operation::new("ORA", 4, 2, 0, AddressingModes::ZeroPageX));
        vector[0x16] = Some(Operation::new("ASL", 6, 2, 0, AddressingModes::ZeroPageX));
        vector[0x17] = None;
        vector[0x18] = Some(Operation::new("CLC", 2, 1, 0, AddressingModes::Implicit));
        vector[0x19] = Some(Operation::new("ORA", 4, 3, 1, AddressingModes::AbsoluteY));
        vector[0x1A] = None;
        vector[0x1B] = None;
        vector[0x1C] = None;
        vector[0x1D] = Some(Operation::new("ORA", 4, 3, 1, AddressingModes::AbsoluteX));
        vector[0x1E] = Some(Operation::new("ASL", 7, 3, 0, AddressingModes::AbsoluteX));
        vector[0x1F] = None;

        vector[0x20] = Some(Operation::new("JSR", 6, 3, 0, AddressingModes::Absolute));
        vector[0x21] = Some(Operation::new(
            "AND",
            6,
            2,
            0,
            AddressingModes::IndexedIndirect,
        ));
        vector[0x22] = None;
        vector[0x23] = None;
        vector[0x24] = Some(Operation::new("BIT", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0x25] = Some(Operation::new("AND", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0x26] = Some(Operation::new("ROL", 5, 2, 0, AddressingModes::ZeroPage));
        vector[0x27] = None;
        vector[0x28] = Some(Operation::new("PLP", 4, 1, 0, AddressingModes::Implicit));
        vector[0x29] = Some(Operation::new("AND", 2, 2, 0, AddressingModes::Immediate));
        vector[0x2A] = Some(Operation::new("ROL", 2, 1, 0, AddressingModes::Accumulator));
        vector[0x2B] = None;
        vector[0x2C] = Some(Operation::new("BIT", 3, 4, 0, AddressingModes::Absolute));
        vector[0x2D] = Some(Operation::new("AND", 4, 3, 0, AddressingModes::Absolute));
        vector[0x2E] = Some(Operation::new("ROL", 3, 6, 0, AddressingModes::Absolute));
        vector[0x2F] = None;

        vector[0x30] = Some(Operation::new("BMI", 2, 2, 2, AddressingModes::Relative));
        vector[0x31] = Some(Operation::new(
            "AND",
            5,
            2,
            1,
            AddressingModes::IndirectIndexed,
        ));
        vector[0x32] = None;
        vector[0x33] = None;
        vector[0x34] = None;
        vector[0x35] = Some(Operation::new("AND", 4, 2, 0, AddressingModes::ZeroPageX));
        vector[0x36] = Some(Operation::new("ROL", 2, 6, 0, AddressingModes::ZeroPageX));
        vector[0x37] = None;
        vector[0x38] = Some(Operation::new("SEC", 2, 1, 0, AddressingModes::Implicit));
        vector[0x39] = Some(Operation::new("AND", 4, 3, 1, AddressingModes::AbsoluteY));
        vector[0x3A] = None;
        vector[0x3B] = None;
        vector[0x3C] = None;
        vector[0x3D] = Some(Operation::new("AND", 4, 3, 1, AddressingModes::AbsoluteX));
        vector[0x3E] = Some(Operation::new("ROL", 7, 3, 0, AddressingModes::AbsoluteX));
        vector[0x3F] = None;

        vector[0x40] = Some(Operation::new("RTI", 6, 1, 0, AddressingModes::Implicit));
        vector[0x41] = Some(Operation::new(
            "EOR",
            6,
            2,
            0,
            AddressingModes::IndexedIndirect,
        ));
        vector[0x42] = None;
        vector[0x43] = None;
        vector[0x44] = None;
        vector[0x45] = Some(Operation::new("EOR", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0x46] = Some(Operation::new("LSR", 5, 2, 0, AddressingModes::ZeroPage));
        vector[0x47] = None;
        vector[0x48] = Some(Operation::new("PHA", 3, 1, 0, AddressingModes::Implicit));
        vector[0x49] = Some(Operation::new("EOR", 2, 2, 0, AddressingModes::Immediate));
        vector[0x4A] = Some(Operation::new("LSR", 2, 1, 0, AddressingModes::Accumulator));
        vector[0x4B] = None;
        vector[0x4C] = Some(Operation::new("JMP", 3, 3, 0, AddressingModes::Absolute));
        vector[0x4D] = Some(Operation::new("EOR", 4, 3, 0, AddressingModes::Absolute));
        vector[0x4E] = Some(Operation::new("LSR", 6, 3, 0, AddressingModes::Absolute));
        vector[0x4F] = None;

        vector[0x50] = Some(Operation::new("BVC", 2, 2, 1, AddressingModes::Relative));
        vector[0x51] = Some(Operation::new(
            "EOR",
            5,
            2,
            1,
            AddressingModes::IndirectIndexed,
        ));
        vector[0x52] = None;
        vector[0x53] = None;
        vector[0x54] = None;
        vector[0x55] = Some(Operation::new("EOR", 4, 2, 0, AddressingModes::ZeroPageX));
        vector[0x56] = Some(Operation::new("LSR", 6, 2, 0, AddressingModes::ZeroPageX));
        vector[0x57] = None;
        vector[0x58] = Some(Operation::new("CLI", 2, 1, 0, AddressingModes::Implicit));
        vector[0x59] = Some(Operation::new("EOR", 4, 3, 1, AddressingModes::AbsoluteY));
        vector[0x5A] = None;
        vector[0x5B] = None;
        vector[0x5C] = None;
        vector[0x5D] = Some(Operation::new("EOR", 4, 3, 1, AddressingModes::AbsoluteX));
        vector[0x5E] = Some(Operation::new("LSR", 7, 3, 0, AddressingModes::AbsoluteX));
        vector[0x5F] = None;

        vector[0x60] = Some(Operation::new("RTS", 6, 1, 0, AddressingModes::Implicit));
        vector[0x61] = Some(Operation::new(
            "ADC",
            6,
            2,
            0,
            AddressingModes::IndirectIndexed,
        ));
        vector[0x62] = None;
        vector[0x63] = None;
        vector[0x64] = None;
        vector[0x65] = Some(Operation::new("ADC", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0x66] = Some(Operation::new("ROR", 5, 2, 0, AddressingModes::ZeroPage));
        vector[0x67] = None;
        vector[0x68] = Some(Operation::new("PLA", 4, 1, 0, AddressingModes::Implicit));
        vector[0x69] = Some(Operation::new("ADC", 2, 2, 0, AddressingModes::Immediate));
        vector[0x6A] = Some(Operation::new("ROR", 2, 1, 0, AddressingModes::Accumulator));
        vector[0x6B] = None;
        vector[0x6C] = Some(Operation::new("JMP", 5, 3, 0, AddressingModes::Indirect));
        vector[0x6D] = Some(Operation::new("ADC", 4, 3, 0, AddressingModes::Absolute));
        vector[0x6E] = Some(Operation::new("ROR", 6, 3, 0, AddressingModes::Absolute));
        vector[0x6F] = None;

        vector[0x70] = Some(Operation::new("BVS", 2, 2, 1, AddressingModes::Relative));
        vector[0x71] = Some(Operation::new(
            "ADC",
            5,
            2,
            1,
            AddressingModes::IndirectIndexed,
        ));
        vector[0x72] = None;
        vector[0x73] = None;
        vector[0x74] = None;
        vector[0x75] = Some(Operation::new("ADC", 4, 2, 0, AddressingModes::ZeroPageX));
        vector[0x76] = Some(Operation::new("ROR", 6, 2, 0, AddressingModes::ZeroPageX));
        vector[0x77] = None;
        vector[0x78] = Some(Operation::new("SEI", 2, 1, 0, AddressingModes::Implicit));
        vector[0x79] = Some(Operation::new("ADC", 4, 3, 1, AddressingModes::AbsoluteY));
        vector[0x7A] = None;
        vector[0x7B] = None;
        vector[0x7C] = None;
        vector[0x7D] = Some(Operation::new("ADC", 4, 3, 1, AddressingModes::AbsoluteX));
        vector[0x7E] = Some(Operation::new("ROR", 7, 3, 0, AddressingModes::AbsoluteX));
        vector[0x7F] = None;

        vector[0x80] = None;
        vector[0x81] = Some(Operation::new(
            "STA",
            6,
            2,
            0,
            AddressingModes::IndirectIndexed,
        ));
        vector[0x82] = None;
        vector[0x83] = None;
        vector[0x84] = Some(Operation::new("STY", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0x85] = Some(Operation::new("STA", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0x86] = Some(Operation::new("STX", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0x87] = None;
        vector[0x88] = Some(Operation::new("DEY", 2, 1, 0, AddressingModes::Implicit));
        vector[0x89] = None;
        vector[0x8A] = Some(Operation::new("TXA", 2, 1, 0, AddressingModes::Implicit));
        vector[0x8B] = None;
        vector[0x8C] = Some(Operation::new("STY", 4, 3, 0, AddressingModes::Absolute));
        vector[0x8D] = Some(Operation::new("STA", 4, 3, 0, AddressingModes::Absolute));
        vector[0x8E] = Some(Operation::new("STX", 4, 3, 0, AddressingModes::Absolute));
        vector[0x8F] = None;

        vector[0x90] = Some(Operation::new("BCC", 2, 2, 1, AddressingModes::Relative));
        vector[0x91] = Some(Operation::new(
            "STA",
            6,
            2,
            0,
            AddressingModes::IndirectIndexed,
        ));
        vector[0x92] = None;
        vector[0x93] = None;
        vector[0x94] = Some(Operation::new("STY", 4, 2, 0, AddressingModes::ZeroPageX));
        vector[0x95] = Some(Operation::new("STA", 4, 2, 0, AddressingModes::ZeroPageX));
        vector[0x96] = Some(Operation::new("STX", 4, 2, 0, AddressingModes::ZeroPageY));
        vector[0x97] = None;
        vector[0x98] = Some(Operation::new("TYA", 2, 1, 0, AddressingModes::Implicit));
        vector[0x99] = Some(Operation::new("STA", 5, 3, 0, AddressingModes::AbsoluteY));
        vector[0x9A] = Some(Operation::new("TXS", 2, 1, 0, AddressingModes::Implicit));
        vector[0x9B] = None;
        vector[0x9C] = None;
        vector[0x9D] = Some(Operation::new("STA", 5, 3, 0, AddressingModes::AbsoluteX));
        vector[0x9E] = None;
        vector[0x9F] = None;

        vector[0xA0] = Some(Operation::new("LDY", 2, 2, 0, AddressingModes::Immediate));
        vector[0xA1] = Some(Operation::new(
            "LDA",
            6,
            2,
            0,
            AddressingModes::IndexedIndirect,
        ));
        vector[0xA2] = Some(Operation::new("LDX", 2, 2, 0, AddressingModes::Immediate));
        vector[0xA3] = None;
        vector[0xA4] = Some(Operation::new("LDY", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0xA5] = Some(Operation::new("LDA", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0xA6] = Some(Operation::new("LDX", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0xA7] = None;
        vector[0xA8] = Some(Operation::new("TAY", 2, 1, 0, AddressingModes::Implicit));
        vector[0xA9] = Some(Operation::new("LDA", 2, 2, 0, AddressingModes::Immediate));
        vector[0xAA] = Some(Operation::new("TAX", 2, 1, 0, AddressingModes::Implicit));
        vector[0xAB] = None;
        vector[0xAC] = Some(Operation::new("LDY", 4, 3, 0, AddressingModes::Absolute));
        vector[0xAD] = Some(Operation::new("LDA", 4, 3, 0, AddressingModes::Absolute));
        vector[0xAE] = Some(Operation::new("LDX", 4, 3, 0, AddressingModes::Absolute));
        vector[0xAF] = None;

        vector[0xB0] = Some(Operation::new("BCS", 2, 2, 1, AddressingModes::Relative));
        vector[0xB1] = Some(Operation::new(
            "LDA",
            5,
            2,
            1,
            AddressingModes::IndirectIndexed,
        ));
        vector[0xB2] = None;
        vector[0xB3] = None;
        vector[0xB4] = Some(Operation::new("LDY", 4, 2, 0, AddressingModes::ZeroPageX));
        vector[0xB5] = Some(Operation::new("LDA", 4, 2, 0, AddressingModes::ZeroPageX));
        vector[0xB6] = Some(Operation::new("LDX", 4, 2, 0, AddressingModes::ZeroPageY));
        vector[0xB7] = None;
        vector[0xB8] = Some(Operation::new("CLV", 2, 1, 0, AddressingModes::Implicit));
        vector[0xB9] = Some(Operation::new("LDA", 4, 3, 1, AddressingModes::AbsoluteY));
        vector[0xBA] = Some(Operation::new("TSX", 2, 1, 0, AddressingModes::Implicit));
        vector[0xBB] = None;
        vector[0xBC] = Some(Operation::new("LDY", 4, 3, 1, AddressingModes::AbsoluteX));
        vector[0xBD] = Some(Operation::new("LDA", 4, 3, 1, AddressingModes::AbsoluteX));
        vector[0xBE] = Some(Operation::new("LDX", 4, 3, 1, AddressingModes::AbsoluteY));
        vector[0xBF] = None;

        vector[0xC0] = Some(Operation::new("CPY", 2, 2, 0, AddressingModes::Immediate));
        vector[0xC1] = Some(Operation::new(
            "CMP",
            6,
            2,
            0,
            AddressingModes::IndexedIndirect,
        ));
        vector[0xC2] = None;
        vector[0xC3] = None;
        vector[0xC4] = Some(Operation::new("CPY", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0xC5] = Some(Operation::new("CMP", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0xC6] = Some(Operation::new("DEC", 5, 2, 0, AddressingModes::ZeroPage));
        vector[0xC7] = None;
        vector[0xC8] = Some(Operation::new("INY", 2, 1, 0, AddressingModes::Implicit));
        vector[0xC9] = Some(Operation::new("CMP", 2, 2, 0, AddressingModes::Immediate));
        vector[0xCA] = Some(Operation::new("DEX", 2, 1, 0, AddressingModes::Implicit));
        vector[0xCB] = None;
        vector[0xCC] = Some(Operation::new("CPY", 4, 3, 0, AddressingModes::Absolute));
        vector[0xCD] = Some(Operation::new("CMP", 4, 3, 0, AddressingModes::Absolute));
        vector[0xCE] = Some(Operation::new("DEC", 6, 3, 0, AddressingModes::Absolute));
        vector[0xCF] = None;

        vector[0xD0] = Some(Operation::new("BNE", 2, 2, 1, AddressingModes::Relative));
        vector[0xD1] = Some(Operation::new(
            "CMP",
            5,
            2,
            0,
            AddressingModes::IndirectIndexed,
        ));
        vector[0xD2] = None;
        vector[0xD3] = None;
        vector[0xD4] = None;
        vector[0xD5] = Some(Operation::new("CMP", 4, 2, 0, AddressingModes::ZeroPageX));
        vector[0xD6] = Some(Operation::new("DEC", 6, 2, 0, AddressingModes::ZeroPageX));
        vector[0xD7] = None;
        vector[0xD8] = Some(Operation::new("CLD", 2, 1, 0, AddressingModes::Implicit));
        vector[0xD9] = Some(Operation::new("CMP", 4, 3, 1, AddressingModes::AbsoluteY));
        vector[0xDA] = None;
        vector[0xDB] = None;
        vector[0xDC] = None;
        vector[0xDD] = Some(Operation::new("CMP", 4, 3, 1, AddressingModes::AbsoluteX));
        vector[0xDE] = Some(Operation::new("DEC", 7, 3, 0, AddressingModes::AbsoluteX));
        vector[0xDF] = None;

        vector[0xE0] = Some(Operation::new("CPX", 2, 2, 0, AddressingModes::Immediate));
        vector[0xE1] = Some(Operation::new(
            "SBC",
            6,
            2,
            0,
            AddressingModes::IndexedIndirect,
        ));
        vector[0xE2] = None;
        vector[0xE3] = None;
        vector[0xE4] = Some(Operation::new("CPX", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0xE5] = Some(Operation::new("SBC", 3, 2, 0, AddressingModes::ZeroPage));
        vector[0xE6] = Some(Operation::new("INC", 5, 2, 0, AddressingModes::ZeroPage));
        vector[0xE7] = None;
        vector[0xE8] = Some(Operation::new("INX", 2, 1, 0, AddressingModes::Implicit));
        vector[0xE9] = Some(Operation::new("SBC", 2, 2, 0, AddressingModes::Immediate));
        vector[0xEA] = Some(Operation::new("NOP", 2, 1, 0, AddressingModes::Implicit));
        vector[0xEB] = None;
        vector[0xEC] = Some(Operation::new("CPX", 4, 3, 0, AddressingModes::Absolute));
        vector[0xED] = Some(Operation::new("SBC", 4, 3, 0, AddressingModes::Absolute));
        vector[0xEE] = Some(Operation::new("INC", 6, 3, 0, AddressingModes::Absolute));
        vector[0xEF] = None;

        vector[0xF0] = Some(Operation::new("BEQ", 2, 2, 1, AddressingModes::Relative));
        vector[0xF1] = Some(Operation::new(
            "SBC",
            5,
            2,
            1,
            AddressingModes::IndirectIndexed,
        ));
        vector[0xF2] = None;
        vector[0xF3] = None;
        vector[0xF4] = None;
        vector[0xF5] = Some(Operation::new("SBC", 4, 2, 0, AddressingModes::ZeroPage));
        vector[0xF6] = Some(Operation::new("INC", 6, 2, 0, AddressingModes::ZeroPageX));
        vector[0xF7] = None;
        vector[0xF8] = Some(Operation::new("SED", 2, 1, 0, AddressingModes::Implicit));
        vector[0xF9] = Some(Operation::new("SBC", 4, 3, 1, AddressingModes::AbsoluteY));
        vector[0xFA] = None;
        vector[0xFB] = None;
        vector[0xFC] = None;
        vector[0xFD] = Some(Operation::new("SBC", 4, 3, 1, AddressingModes::AbsoluteX));
        vector[0xFE] = Some(Operation::new("INC", 7, 3, 0, AddressingModes::AbsoluteX));
        vector[0xFF] = None;

        vector
    };
}
