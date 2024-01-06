use crate::bus::Bus;
use crate::constants::{RESET_STACK_ADDRESS, STATUS_REGISTER_INITIAL};
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
}

// Implement Memory functions
impl Cpu {
    fn mem_read(&self, address: u16) -> u8 {
        self.bus.mem_read(address)
    }

    fn mem_write(&mut self, address: u16, value: u8) {
        self.bus.mem_write(address, value)
    }

    fn mem_read_u16(&self, address: u16) -> u16
    {
        let lo = self.mem_read(address) as u16;
        let hi = self.mem_read(address + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, address: u16, value: u16)
    {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;
        self.mem_write(address, lo);
        self.mem_write(address + 1, hi)
    }
}

// Operation functions 
impl Cpu {

}
