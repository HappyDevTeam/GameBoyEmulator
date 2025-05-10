use super::CPU;

pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ADDHLTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(IncDecTarget),
    DEC(IncDecTarget),
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RLCA,
    CPL,
    BIT(PrefixTarget, BitPosition),
    RESET(PrefixTarget, BitPosition),
    SET(PrefixTarget, BitPosition),
    SRL(PrefixTarget),
    RR(PrefixTarget),
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RLC(PrefixTarget),
    SRA(PrefixTarget),
    SLA(PrefixTarget),
    SWAP(PrefixTarget),
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            _ => todo!("ADD MAPPING FOR REST OF INSTRUCTIONS"),
        }
    }
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

pub enum ADDHLTarget {
    BC,
    DE,
    HL,
    SP,
}

pub enum IncDecTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    BC,
    DE,
    HL,
    SP,
}

pub enum PrefixTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

pub enum BitPosition {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}

impl From<BitPosition> for u8 {
    fn from(position: BitPosition) -> u8 {
        match position {
            BitPosition::B0 => 0,
            BitPosition::B1 => 1,
            BitPosition::B2 => 2,
            BitPosition::B3 => 3,
            BitPosition::B4 => 4,
            BitPosition::B5 => 5,
            BitPosition::B6 => 6,
            BitPosition::B7 => 7,
        }
    }
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(register) => {
                let value = self.get_value(register);
                let new_value = self.add(value, false);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }

            Instruction::ADDHL(register) => {
                let value = match register {
                    ADDHLTarget::BC => self.registers.get_bc(),
                    ADDHLTarget::DE => self.registers.get_de(),
                    ADDHLTarget::HL => self.registers.get_hl(),
                    ADDHLTarget::SP => self.sp,
                };

                let new_value = self.add_hl(value);
                self.registers.set_hl(new_value);
                self.pc.wrapping_add(1)
            }

            Instruction::ADC(register) => {
                let value = self.get_value(register);
                let new_value = self.add(value, true);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }

            Instruction::AND(register) => {
                let value = self.get_value(register);
                let new_value = self.and(value);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }

            Instruction::CP(register) => {
                let value = self.get_value(register);
                self.cp(value);
                self.pc.wrapping_add(1)
            }

            Instruction::INC(register) => {
                match register {
                    IncDecTarget::A => self.registers.a = self.inc_8bit(self.registers.a),
                    IncDecTarget::B => self.registers.b = self.inc_8bit(self.registers.b),
                    IncDecTarget::C => self.registers.c = self.inc_8bit(self.registers.c),
                    IncDecTarget::D => self.registers.d = self.inc_8bit(self.registers.d),
                    IncDecTarget::E => self.registers.e = self.inc_8bit(self.registers.e),
                    IncDecTarget::H => self.registers.h = self.inc_8bit(self.registers.h),
                    IncDecTarget::L => self.registers.l = self.inc_8bit(self.registers.l),
                    IncDecTarget::HLI => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.inc_8bit(value);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                    }
                    IncDecTarget::BC => {
                        self.registers.set_bc(self.inc_16bit(self.registers.get_bc()))
                    }
                    IncDecTarget::DE => {
                        self.registers.set_de(self.inc_16bit(self.registers.get_de()))
                    }
                    IncDecTarget::HL => {
                        self.registers.set_hl(self.inc_16bit(self.registers.get_hl()))
                    }
                    IncDecTarget::SP => {
                        self.sp = self.inc_16bit(self.sp)
                    }
                };
                self.pc.wrapping_add(1)
            }

            Instruction::CCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = !self.registers.f.carry;
                self.pc.wrapping_add(1)
            }

            Instruction::RRA => {
                self.registers.a = self.rotate_right_through_carry_retain_zero(self.registers.a);
                self.pc.wrapping_add(1)
            }

            Instruction::RRCA => {
                self.registers.a = self.rotate_right(self.registers.a, true);
                self.pc.wrapping_add(1)
            }

            Instruction::CPL => {
                self.registers.a = !self.registers.a;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = true;
                self.pc.wrapping_add(1)
            }

            Instruction::RESET(register, position) => {
                match register {
                    PrefixTarget::A =>
                        self.registers.a = self.reset_bit(self.registers.a, position),
                    PrefixTarget::B =>
                        self.registers.b = self.reset_bit(self.registers.b, position),
                    PrefixTarget::C =>
                        self.registers.c = self.reset_bit(self.registers.c, position),
                    PrefixTarget::D =>
                        self.registers.d = self.reset_bit(self.registers.d, position),
                    PrefixTarget::E =>
                        self.registers.e = self.reset_bit(self.registers.e, position),
                    PrefixTarget::H =>
                        self.registers.h = self.reset_bit(self.registers.h, position),
                    PrefixTarget::L =>
                        self.registers.l = self.reset_bit(self.registers.l, position),
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.reset_bit(value, position);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                    }
                };
                self.pc.wrapping_add(1)
            }

            Instruction::SRL(register) => {
                match register {
                    PrefixTarget::A =>
                        self.registers.a = self.shift_right_logical(self.registers.a),
                    PrefixTarget::B =>
                        self.registers.b = self.shift_right_logical(self.registers.b),
                    PrefixTarget::C =>
                        self.registers.c = self.shift_right_logical(self.registers.c),
                    PrefixTarget::D =>
                        self.registers.d = self.shift_right_logical(self.registers.d),
                    PrefixTarget::E =>
                        self.registers.e = self.shift_right_logical(self.registers.e),
                    PrefixTarget::H =>
                        self.registers.h = self.shift_right_logical(self.registers.h),
                    PrefixTarget::L =>
                        self.registers.l = self.shift_right_logical(self.registers.l),
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.shift_right_logical(value);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                    }
                };
                self.pc.wrapping_add(1)
            }

            Instruction::RR(register) => {
                match register {
                    PrefixTarget::A =>
                        self.registers.a =
                            self.rotate_right_through_carry_set_zero(self.registers.a),
                    PrefixTarget::B =>
                        self.registers.b =
                            self.rotate_right_through_carry_set_zero(self.registers.b),
                    PrefixTarget::C =>
                        self.registers.c =
                            self.rotate_right_through_carry_set_zero(self.registers.c),
                    PrefixTarget::D =>
                        self.registers.d =
                            self.rotate_right_through_carry_set_zero(self.registers.d),
                    PrefixTarget::E =>
                        self.registers.e =
                            self.rotate_right_through_carry_set_zero(self.registers.e),
                    PrefixTarget::H =>
                        self.registers.h =
                            self.rotate_right_through_carry_set_zero(self.registers.h),
                    PrefixTarget::L =>
                        self.registers.l =
                            self.rotate_right_through_carry_set_zero(self.registers.l),
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.rotate_right_through_carry_set_zero(value);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                    }
                };
                self.pc.wrapping_add(1)
            }

            Instruction::RRC(register) => {
                match register {
                    PrefixTarget::A => self.registers.a = self.rotate_right(self.registers.a, true),
                    PrefixTarget::B => self.registers.b = self.rotate_right(self.registers.b, true),
                    PrefixTarget::C => self.registers.c = self.rotate_right(self.registers.c, true),
                    PrefixTarget::D => self.registers.d = self.rotate_right(self.registers.d, true),
                    PrefixTarget::E => self.registers.e = self.rotate_right(self.registers.e, true),
                    PrefixTarget::H => self.registers.h = self.rotate_right(self.registers.h, true),
                    PrefixTarget::L => self.registers.l = self.rotate_right(self.registers.l, true),
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.rotate_right(value, true);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                    }
                };
                self.pc.wrapping_add(1)
            }
            
            Instruction::SUB(register) => {
                let value = self.get_value(register);
                let new_value = self.sub(value);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }

            Instruction::SBC(register) => {
                let value = self.get_value(register);
                let new_value = self.sbc(value);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            
            Instruction::OR(register) => {
                let value = self.get_value(register);
                let new_value = self.or(value);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }

            Instruction::XOR(register) => {
                let value = self.get_value(register);
                let new_value = self.xor(value);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            
            Instruction::DEC(register) => {
                match register {
                    IncDecTarget::A => self.registers.a = self.dec_8bit(self.registers.a),
                    IncDecTarget::B => self.registers.b = self.dec_8bit(self.registers.b),
                    IncDecTarget::C => self.registers.c = self.dec_8bit(self.registers.c),
                    IncDecTarget::D => self.registers.d = self.dec_8bit(self.registers.d),
                    IncDecTarget::E => self.registers.e = self.dec_8bit(self.registers.e),
                    IncDecTarget::H => self.registers.h = self.dec_8bit(self.registers.h),
                    IncDecTarget::L => self.registers.l = self.dec_8bit(self.registers.l),
                    IncDecTarget::HLI => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.dec_8bit(value);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                    }
                    IncDecTarget::BC => {
                        self.registers.set_bc(self.dec_16bit(self.registers.get_bc()))
                    }
                    IncDecTarget::DE => {
                        self.registers.set_de(self.dec_16bit(self.registers.get_de()))
                    }
                    IncDecTarget::HL => {
                        self.registers.set_hl(self.dec_16bit(self.registers.get_hl()))
                    }
                    IncDecTarget::SP => {
                        self.sp = self.dec_16bit(self.sp)
                    }
                };
                self.pc.wrapping_add(1)
            }
            
            Instruction::SCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
                self.pc.wrapping_add(1)
            }

            Instruction::RLA => {
                self.registers.a = self.rotate_left_through_carry_retain_zero(self.registers.a);
                self.pc.wrapping_add(1)
            }

            Instruction::RLCA => {
                self.registers.a = self.rotate_left(self.registers.a, false);
                self.pc.wrapping_add(1)
            }
            
            Instruction::BIT(prefix_target, bit_pos) => {
                let value = match prefix_target {
                    PrefixTarget::A  => self.registers.a,
                    PrefixTarget::B  => self.registers.b,
                    PrefixTarget::C  => self.registers.c,
                    PrefixTarget::D  => self.registers.d,
                    PrefixTarget::E  => self.registers.e,
                    PrefixTarget::H  => self.registers.h,
                    PrefixTarget::L  => self.registers.l,
                    PrefixTarget::HL => self.bus.read_byte(self.registers.get_hl()),
                };
                self.test_bit(value, bit_pos);
                self.pc.wrapping_add(2)
            }
            
            Instruction::SET(target, bit_pos) => {
                match target {
                    PrefixTarget::A  => self.registers.a = self.set_bit(self.registers.a, bit_pos),
                    PrefixTarget::B  => self.registers.b = self.set_bit(self.registers.b, bit_pos),
                    PrefixTarget::C  => self.registers.c = self.set_bit(self.registers.c, bit_pos),
                    PrefixTarget::D  => self.registers.d = self.set_bit(self.registers.d, bit_pos),
                    PrefixTarget::E  => self.registers.e = self.set_bit(self.registers.e, bit_pos),
                    PrefixTarget::H  => self.registers.h = self.set_bit(self.registers.h, bit_pos),
                    PrefixTarget::L  => self.registers.l = self.set_bit(self.registers.l, bit_pos),
                    PrefixTarget::HL => {
                        let address = self.registers.get_hl();
                        let value = self.bus.read_byte(address);
                        let new_value = self.set_bit(value, bit_pos);
                        self.bus.write_byte(address, new_value);
                    }
                }
                self.pc.wrapping_add(2)
            }
            
            Instruction::RL(target) => {
                match target {
                    PrefixTarget::A  => self.registers.a = self.rotate_left(self.registers.a, true),
                    PrefixTarget::B  => self.registers.b = self.rotate_left(self.registers.b, true),
                    PrefixTarget::C  => self.registers.c = self.rotate_left(self.registers.c, true),
                    PrefixTarget::D  => self.registers.d = self.rotate_left(self.registers.d, true),
                    PrefixTarget::E  => self.registers.e = self.rotate_left(self.registers.e, true),
                    PrefixTarget::H  => self.registers.h = self.rotate_left(self.registers.h, true),
                    PrefixTarget::L  => self.registers.l = self.rotate_left(self.registers.l, true),
                    PrefixTarget::HL => {
                        let address = self.registers.get_hl();
                        let value = self.bus.read_byte(address);
                        let new_value = self.rotate_left(value, true);
                        self.bus.write_byte(address, new_value);
                    }
                }
                self.pc.wrapping_add(2)
            }
            
            _ => todo!("ADD MAPPING FOR REST OF INSTRUCTIONS"),
        }
    }

    fn get_value(&self, register: ArithmeticTarget) -> u8 {
        match register {
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
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + add_carry > 0xF;
        self.registers.f.carry = did_overflow || did_overflow2;
        new_value2
    }

    fn add_hl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.get_hl() & 0b111_1111_1111)
            + (value & 0b111_1111_1111)
            > 0b111_1111_1111;
        self.registers.f.carry = did_overflow;
        new_value
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
        new_value
    }

    fn cp(&mut self, value: u8) {
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = self.registers.a < value;
    }

    fn inc_8bit(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = value == new_value;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = value & 0xF == 0xF;
        new_value
    }

    fn inc_16bit(&mut self, value: u16) -> u16 {
        value.wrapping_add(1)
    }

    fn rotate_right_through_carry_retain_zero(&mut self, value: u8) -> u8 {
        self.rotate_right_through_carry(value, false)
    }

    fn rotate_right_through_carry_set_zero(&mut self, value: u8) -> u8 {
        self.rotate_right_through_carry(value, true)
    }

    fn rotate_right_through_carry(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = (self.registers.f.carry as u8) << 7;
        let new_value = carry_bit | (value >> 1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    fn rotate_right(&mut self, value: u8, set_zero: bool) -> u8 {
        let new_value = value.rotate_right(1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    fn reset_bit(&mut self, value: u8, bit_position: BitPosition) -> u8 {
        let bit_position: u8 = bit_position.into();
        value & !(1 << bit_position)
    }

    fn shift_right_logical(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    fn shift_right_arithmetic(&mut self, value: u8) -> u8 {
        let new_value = (value >> 1) | (value & 0x80);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }
    
    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
    
    fn sbc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        let (new_value2, did_overflow2) = if self.registers.f.carry {
            new_value.overflowing_sub(1)
        } else {
            (new_value, did_overflow)
        };
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow || did_overflow2;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + 1 > 0xF;
        new_value2
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }
    
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }
    
    fn dec_8bit(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = value == new_value;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = value & 0xF == 0xF;
        new_value
    }

    fn dec_16bit(&mut self, value: u16) -> u16 {
        value.wrapping_sub(1)
    }

    fn rotate_left_through_carry_retain_zero(&mut self, value: u8) -> u8 {
        self.rotate_left_through_carry(value, false)
    }

    fn rotate_left_through_carry_set_zero(&mut self, value: u8) -> u8 {
        self.rotate_left_through_carry(value, true)
    }

    fn rotate_left_through_carry(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = if self.registers.f.carry { 1 } else { 0 };
        let new_value = carry_bit | (value << 1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0x80 == 0x80;
        new_value
    }
    fn rotate_left(&mut self, value: u8, set_zero: bool) -> u8 {
        let new_value = value.rotate_left(1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0x80 == 0x80;
        new_value
    }
    
    fn test_bit(&mut self, value: u8, bit_pos: BitPosition) {
        let bit_pos: u8 = bit_pos.into();
        let new_value = (value >> bit_pos) & 0b1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }
    
    fn set_bit(&mut self, value: u8, bit_pos: BitPosition) -> u8 {
        let bit_pos: u8 = bit_pos.into();
        value | (1 << bit_pos)
    }
}
