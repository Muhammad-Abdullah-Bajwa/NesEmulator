use crate::bus::Bus;
use crate::constants::{
    BitMasks, IRQ_INTERRUPT_VECTOR_ADDRESS, RESET_PROGRAM_COUNTER_ADDRESS, RESET_STACK_ADDRESS,
    STACK_START, STATUS_REGISTER_INITIAL, U16_HIGH_BYTE_MASK, U16_LOW_BYTE_MASK,
};
use crate::cpu_flags::CpuFlags;

pub struct Cpu {
    program_counter: u16,
    stack_pointer: u8,
    register_accumulator: u8,
    register_x: u8,
    register_y: u8,
    register_status: CpuFlags,
    bus: Bus,
}

// Implement Basic Functions for CPU
impl Cpu {
    pub fn new(bus: Bus) -> Self {
        let register = CpuFlags::from_bits(STATUS_REGISTER_INITIAL);

        match register {
            Some(status) => {
                return Cpu {
                    program_counter: 0,
                    stack_pointer: RESET_STACK_ADDRESS,
                    register_accumulator: 0,
                    register_x: 0,
                    register_y: 0,
                    register_status: status,
                    bus: bus,
                };
            }
            None => panic!("Could not create CPU flags!"),
        }
    }

    pub fn reset(&mut self) {
        self.register_accumulator = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = RESET_STACK_ADDRESS;

        let flags = CpuFlags::from_bits(STATUS_REGISTER_INITIAL);

        match flags {
            Some(status) => {
                self.register_status = status;
            }
            None => {
                panic!("Could not create CPU flags!");
            }
        }

        let reset_pc = self.mem_read_u16(RESET_PROGRAM_COUNTER_ADDRESS);
        self.program_counter = reset_pc;
    }
}

// CMP related operations
impl Cpu {
    fn compare(&mut self, address: u16, register_value: u8) {
        let value = self.mem_read(address);

        if register_value >= value {
            self.register_status.insert(CpuFlags::CARRY);
        } else {
            self.register_status.remove(CpuFlags::CARRY);
        }

        self.update_zero_and_negative_flags(register_value.wrapping_sub(value));
    }

    fn cmp(&mut self, address: u16) {
        self.compare(address, self.register_accumulator);
    }

    fn cpx(&mut self, address: u16) {
        self.compare(address, self.register_x);
    }

    fn cpy(&mut self, address: u16) {
        self.compare(address, self.register_y);
    }
}

// Branching related operations
impl Cpu {
    fn branch_helper(&mut self, condition: bool) {
        if condition {
            let jump = self.mem_read(self.program_counter) as i8;
            let jump_address = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(jump as u16);
            self.program_counter = jump_address;
        }
    }

    fn blp(&mut self) {
        self.branch_helper(!self.register_status.contains(CpuFlags::NEGATIVE));
    }

    fn bcc(&mut self) {
        self.branch_helper(!self.register_status.contains(CpuFlags::CARRY));
    }

    fn bcs(&mut self) {
        self.branch_helper(self.register_status.contains(CpuFlags::CARRY));
    }

    fn beq(&mut self) {
        self.branch_helper(self.register_status.contains(CpuFlags::ZERO));
    }

    fn bmi(&mut self) {
        self.branch_helper(self.register_status.contains(CpuFlags::NEGATIVE));
    }

    fn bne(&mut self) {
        self.branch_helper(!self.register_status.contains(CpuFlags::ZERO));
    }

    fn bvc(&mut self) {
        self.branch_helper(!self.register_status.contains(CpuFlags::OVERFLOW));
    }

    fn bvs(&mut self) {
        self.branch_helper(self.register_status.contains(CpuFlags::OVERFLOW));
    }
}

// Implement Memory functions
impl Cpu {
    fn mem_read(&self, address: u16) -> u8 {
        self.bus.mem_read(address)
    }

    fn mem_write(&mut self, address: u16, value: u8) {
        self.bus.mem_write(address, value)
    }

    fn mem_read_u16(&self, address: u16) -> u16 {
        let lo = self.mem_read(address) as u16;
        let hi = self.mem_read(address + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn get_address_for_indirect(&self) -> u16 {
        let addr = self.mem_read_u16(self.program_counter);

        let mut address = self.mem_read_u16(addr);

        if addr & U16_LOW_BYTE_MASK == U16_LOW_BYTE_MASK {
            let lo = self.mem_read(address);
            let hi = self.mem_read(address & U16_HIGH_BYTE_MASK);
            address = (hi as u16) << 8 | (lo as u16);
        }

        address
    }

    fn mem_write_u16(&mut self, address: u16, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;
        self.mem_write(address, lo);
        self.mem_write(address + 1, hi)
    }
}

// Implement Stack functions for Cpu
impl Cpu {
    fn stack_push(&mut self, val: u8) {
        let address = STACK_START + self.stack_pointer as u16;
        self.mem_write(address, val);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, val: u16) {
        let hi = (val >> 8) as u8;
        let lo = (val & 0xff) as u8;
        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let address = STACK_START + self.stack_pointer as u16;
        self.mem_read(address)
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;

        hi << 8 | lo
    }
}

// Flag related functions
impl Cpu {
    fn update_negative_flag(&mut self, value: u8) {
        if value >> 7 == 1 {
            self.register_status.insert(CpuFlags::NEGATIVE);
        } else {
            self.register_status.remove(CpuFlags::NEGATIVE);
        }
    }

    fn update_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.register_status.insert(CpuFlags::ZERO);
        } else {
            self.register_status.remove(CpuFlags::ZERO);
        }
    }

    fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn set_carry_flag(&mut self) {
        self.register_status.insert(CpuFlags::CARRY);
    }

    fn clear_carry_flag(&mut self) {
        self.register_status.remove(CpuFlags::CARRY);
    }

    fn set_decimal_mode_flag(&mut self) {
        self.register_status.insert(CpuFlags::DECIMAL_MODE);
    }

    fn clear_decimal_mode_flag(&mut self) {
        self.register_status.remove(CpuFlags::DECIMAL_MODE);
    }

    fn set_interrupt_disable_flag(&mut self) {
        self.register_status.insert(CpuFlags::INTERRUPT_DISABLE);
    }

    fn clear_interrupt_disable_flag(&mut self) {
        self.register_status.remove(CpuFlags::INTERRUPT_DISABLE);
    }

    fn clear_overflow_flag(&mut self) {
        self.register_status.remove(CpuFlags::OVERFLOW);
    }
}

// Register related functions
impl Cpu {
    fn set_accumulator(&mut self, value: u8) {
        self.register_accumulator = value;
        self.update_zero_and_negative_flags(self.register_accumulator);
    }

    fn add_to_accumulator(&mut self, value: u8) {
        let carry_val = if (self.register_status.contains(CpuFlags::CARRY)) {
            1
        } else {
            0
        };

        let sum = self.register_accumulator as u16 + value as u16 + carry_val as u16;

        // Check for carry and set carry flag
        let carry_exists = sum > 0xff;

        if carry_exists {
            self.register_status.insert(CpuFlags::CARRY);
        } else {
            self.register_status.remove(CpuFlags::CARRY);
        }

        // Check and set overflow flag
        let res = sum as u8;

        let overflow_exists = (value ^ res) & (res ^ self.register_accumulator) & 0x80 != 0;

        if overflow_exists {
            self.register_status.insert(CpuFlags::OVERFLOW);
        } else {
            self.register_status.remove(CpuFlags::OVERFLOW);
        }

        // Sets value of accumulator and updates zero and negative flags
        self.set_accumulator(res);
    }
}

// Operation functions
impl Cpu {
    fn asl_accumulator(&mut self) {
        let mut val = self.register_accumulator;

        if val >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        val = val << 1;
        self.set_accumulator(val);
    }

    fn asl(&mut self, address: u16) {
        let mut val = self.mem_read(address);

        if val >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        val = val << 1;
        self.mem_write(address, val);
        self.update_zero_and_negative_flags(val);
    }

    fn and(&mut self, address: u16) {
        let val = self.mem_read(address);
        let val = val & self.register_accumulator;
        self.set_accumulator(val);
    }

    fn adc(&mut self, address: u16) {
        let val = self.mem_read(address);
        self.add_to_accumulator(val);
    }

    fn bit(&mut self, address: u16) {
        let val = self.mem_read(address);
        let and_res = self.register_accumulator & val;

        if and_res == 0 {
            self.register_status.insert(CpuFlags::ZERO);
        } else {
            self.register_status.remove(CpuFlags::ZERO);
        }

        let bit_six = val & 0b01000000;
        self.register_status.set(CpuFlags::OVERFLOW, bit_six > 0);

        let bit_seven = val & 0b10000000;
        self.register_status.set(CpuFlags::NEGATIVE, bit_seven > 0);
    }

    fn brk(&mut self) {
        self.stack_push_u16(self.program_counter);
        self.stack_push(self.register_status.bits());

        let new_pc = self.mem_read_u16(IRQ_INTERRUPT_VECTOR_ADDRESS);
        self.program_counter = new_pc;

        self.register_status.insert(CpuFlags::BREAK);
    }

    fn dec(&mut self, address: u16) {
        let value = self.mem_read(address);
        let value = value.wrapping_sub(1);
        self.mem_write(address, value);
        self.update_zero_and_negative_flags(value);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn eor(&mut self, address: u16) {
        let value = self.mem_read(address);
        let accumulator = self.register_accumulator ^ value;
        self.set_accumulator(accumulator);
    }

    fn inc(&mut self, address: u16) {
        let value = self.mem_read(address);
        let value = value.wrapping_add(1);
        self.mem_write(address, value);
        self.update_zero_and_negative_flags(value);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn jmp_absolute(&mut self) {
        let pc = self.mem_read_u16(self.program_counter);
        self.program_counter = pc;
    }

    fn jmp_indirect(&mut self) {
        let address = self.get_address_for_indirect();
        self.program_counter = address;
    }

    fn jsr(&mut self, address: u16) {
        let temp_pc = self.program_counter - 1;
        self.stack_push_u16(temp_pc);
        self.program_counter = address;
    }

    fn lda(&mut self, address: u16) {
        let accumulator = self.mem_read(address);
        self.set_accumulator(accumulator);
    }

    fn ldx(&mut self, address: u16) {
        let reg_x = self.mem_read(address);
        self.register_x = reg_x;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn ldy(&mut self, address: u16) {
        let reg_y = self.mem_read(address);
        self.register_y = reg_y;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn lsr_accumulator(&mut self) {
        let accumulator = self.register_accumulator;

        if accumulator & 0b00000001 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        let accumulator = accumulator >> 1;
        self.set_accumulator(accumulator);
    }

    fn lsr(&mut self, address: u16) {
        let value = self.mem_read(address);

        if value & 0b00000001 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        let value = value >> 1;
        self.mem_write(address, value);
        self.update_zero_and_negative_flags(value);
    }

    fn ora(&mut self, address: u16) {
        let val = self.mem_read(address);
        self.set_accumulator(val | self.register_accumulator);
    }

    fn pha(&mut self) {
        self.stack_push(self.register_accumulator);
    }

    // https://www.masswerk.at/6502/6502_instruction_set.html#PHP
    fn php(&mut self) {
        let mut flags = self.register_status.clone();
        flags.insert(CpuFlags::BREAK);
        flags.insert(CpuFlags::UNUSED);
        self.stack_push(flags.bits());
    }

    fn pla(&mut self) {
        let value = self.stack_pop();
        self.set_accumulator(value);
    }

    fn plp(&mut self) {
        let flags = self.stack_pop();
        let flags = CpuFlags::from_bits(flags);

        match flags {
            Some(status) => {
                self.register_status = status;
            }
            None => {
                panic!("Not able to fetch status flags from the stack");
            }
        }

        self.register_status.remove(CpuFlags::BREAK);
        self.register_status.insert(CpuFlags::UNUSED);
    }

    fn rol_accumulator(&mut self) {
        let mut accumulator = self.register_accumulator;
        let current_carry = self.register_status.contains(CpuFlags::CARRY);

        if accumulator & BitMasks::SEVENTH == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        accumulator = accumulator << 1;

        if current_carry {
            accumulator = accumulator | BitMasks::ZERO;
        }

        self.set_accumulator(accumulator);
    }

    fn rol(&mut self, address: u16) {
        let mut value = self.mem_read(address);
        let current_carry = self.register_status.contains(CpuFlags::CARRY);

        if value & BitMasks::SEVENTH == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        value = value << 1;

        if current_carry {
            value = value | BitMasks::ZERO;
        }

        self.mem_write(address, value);
        self.update_negative_flag(value);
    }

    fn ror_accumulator(&mut self) {
        let mut accumulator = self.register_accumulator;
        let current_carry = self.register_status.contains(CpuFlags::CARRY);

        if accumulator & BitMasks::ZERO == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        accumulator = accumulator >> 1;

        if current_carry {
            accumulator = accumulator | BitMasks::SEVENTH;
        }

        self.set_accumulator(accumulator);
    }

    fn ror(&mut self, address: u16) {
        let mut value = self.mem_read(address);
        let current_carry = self.register_status.contains(CpuFlags::CARRY);

        if value & BitMasks::ZERO == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        value = value >> 1;

        if current_carry {
            value = value | BitMasks::SEVENTH;
        }

        self.mem_write(address, value);
        self.update_negative_flag(value);
    }

    fn rti(&mut self) {
        self.plp();

        let pc = self.stack_pop_u16();
        self.program_counter = pc;
    }

    fn rts(&mut self) {
        let pc = self.stack_pop_u16();
        self.program_counter = pc.wrapping_add(1);
    }

    fn sbc(&mut self, address: u16) {
        let value = self.mem_read(address) as i8;

        let value_twos_complement = value.wrapping_neg();

        // We sub here and then if carry flag is set we can add it back or
        // not.
        let value_twos_complement = value_twos_complement.wrapping_sub(1);

        let value_twos_complement = value_twos_complement as u8;

        self.add_to_accumulator(value_twos_complement);
    }

    fn sta(&mut self, address: u16) {
        let value = self.register_accumulator;
        self.mem_write(address, value)
    }

    fn stx(&mut self, address: u16) {
        let value = self.register_x;
        self.mem_write(address, value)
    }

    fn sty(&mut self, address: u16) {
        let value = self.register_y;
        self.mem_write(address, value)
    }

    fn tax(&mut self) {
        let value = self.register_accumulator;
        self.register_x = value;
        self.update_zero_and_negative_flags(value);
    }

    fn tay(&mut self) {
        let value = self.register_accumulator;
        self.register_y = value;
        self.update_zero_and_negative_flags(value);
    }

    fn tsx(&mut self) {
        let value = self.stack_pointer;
        self.register_x = value;
        self.update_zero_and_negative_flags(value);
    }

    fn txa(&mut self) {
        let value = self.register_x;
        self.set_accumulator(value);
    }

    fn txs(&mut self) {
        let value = self.register_x;
        self.stack_pointer = value;
    }

    fn tya(&mut self) {
        let value = self.register_y;
        self.set_accumulator(value);
    }
}
