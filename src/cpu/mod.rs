pub mod register;
pub mod instruction;

use self::register::Registers;

pub struct CPU {
    pub regs: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: MemoryBus,
}

impl CPU {
    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let instruction_size = self.execute(instruction_byte, prefixed);
        self.pc = self.pc.wrapping_add(instruction_size);
    }
}

pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}