use crate::constants::{
    PPU_REGISTERS, PPU_REGISTERS_MIRRORS_END, RAM_MIRRORS_END, RAM_SIZE, RAM_START,
};

pub struct Bus {
    cpu_ram: [u8; RAM_SIZE as usize],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_ram: [0; RAM_SIZE as usize],
        }
    }

    pub fn mem_read(&self, address: u16) -> u8 {
        match address {
            RAM_START..=RAM_MIRRORS_END => {
                let mirror_down_addr = address & (RAM_SIZE - 1);
                let mirror_down_addr = mirror_down_addr as usize;
                self.cpu_ram[mirror_down_addr]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                todo!("Handle PPU Memory");
            }
            _ => {
                println!("Invalid Memory access at {}", address);
                0
            }
        }
    }

    pub fn mem_write(&mut self, address: u16, value: u8) {
        match address {
            RAM_START..=RAM_MIRRORS_END => {
                let mirror_down_addr = address & (RAM_SIZE - 1);
                let mirror_down_addr = mirror_down_addr as usize;
                self.cpu_ram[mirror_down_addr] = value;
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                todo!("Handle PPU Memory");
            }
            _ => {
                println!("Invalid Memory access at {}", address);
            }
        }
    }
}
