use super::CPU;

pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ADDHLTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(),
    OR(),
    CP(),
    INC(),
    DEC(),
    CCF(),
    SCF(),
    RRA(),
    RLA(),
    RRCA(),
    RRLA(),
    CPL(),
    BIT(),
    RESET(),
    SET(),
    SRL(),
    RR(),
    RL(),
    RRC(),
    RLC(),
    SRA(),
    SLA(),
    SWAP(),
}

impl Instruction {
    fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            _ => todo!(ADD MAPPING FOR REST OF INSTRUCTIONS)
        }
    }
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L, HL,
}

pub enum ADDHLTarget {
    BC,
    DE,
    HL,
    SP,
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                let value = self.get_value(target);
                let new_value = self.add(value, false);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::ADDHL(target) => {
                match target {
                    ADDHLTarget::BC => {
                        let value = self.registers.get_bc();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    ADDHLTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    ADDHLTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    ADDHLTarget::SP => {
                        let value = self.sp();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::ADC(target) => {
                let value = self.get_value(target);
                let new_value = self.add(value, true);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
        }
    }

    fn get_value(&self, target: ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => self.bus.read_byte(self.registers.get_hl()),
        }
    }

    fn add(&mut self, value: u8, with_carry: bool) -> u8 {
        let add_carry = if with_carry && self.registers.f.carry {
            1
        } else {
            0
        };
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        let (new_value2, did_overflow2) = new_value.overflowing_add(add_carry);
        self.registers.f.zero = new_value2 == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow || did_overflow2;
        self.registers.f.half_carry =
            (self.registers.a & 0xF) + (value & 0xF) + add_carry > 0xF;
        new_value2
    }

    fn add_hl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.get_hl() & 0b111_1111_1111) +
            (value & 0b111_1111_1111) > 0b111_1111_1111;
        new_value
    }
}
