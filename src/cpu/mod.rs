pub mod register;
pub mod instruction;

use self::register::Registers;
use self::instruction::Instruction;

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: MemoryBus,
}

impl CPU {
    pub fn step(&mut self) {
        let instruction_byte = self.bus.read_byte(self.pc);

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.execute(instruction)
        } else {
            panic!("Unknown instruction found for: 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
    }
}

pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}