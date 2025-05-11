use super::CPU;

impl CPU {
    pub fn execute(&mut self, byte: u8, prefixed: bool) -> u16 {
        if prefixed {
            return self.execute_prefixed(byte);
        }
        self.execute_not_prefixed(byte)
    }

    fn execute_not_prefixed(&mut self, byte: u8) -> u16 {
        match byte {
            // ADD
            0x80 => { self.add(self.registers.b, false); 1 }
            0x81 => { self.add(self.registers.c, false); 1 }
            0x82 => { self.add(self.registers.d, false); 1 }
            0x83 => { self.add(self.registers.e, false); 1 }
            0x84 => { self.add(self.registers.h, false); 1 }
            0x85 => { self.add(self.registers.l, false); 1 }
            0x86 => { self.add(self.bus.read_byte(self.registers.get_hl()), false); 1 }
            0x87 => { self.add(self.registers.a, false); 1 }

            // ADC
            0x88 => { self.add(self.registers.b, true); 1 }
            0x89 => { self.add(self.registers.c, true); 1 }
            0x8A => { self.add(self.registers.d, true); 1 }
            0x8B => { self.add(self.registers.e, true); 1 }
            0x8C => { self.add(self.registers.h, true); 1 }
            0x8D => { self.add(self.registers.l, true); 1 }
            0x8E => { self.add(self.bus.read_byte(self.registers.get_hl()), true); 1 }
            0x8F => { self.add(self.registers.a, true); 1 }

            // AND
            0xA0 => { self.and(self.registers.b); 1 }
            0xA1 => { self.and(self.registers.c); 1 }
            0xA2 => { self.and(self.registers.d); 1 }
            0xA3 => { self.and(self.registers.e); 1 }
            0xA4 => { self.and(self.registers.h); 1 }
            0xA5 => { self.and(self.registers.l); 1 }
            0xA6 => { self.and(self.bus.read_byte(self.registers.get_hl())); 1 }
            0xA7 => { self.and(self.registers.b); 1 }

            // CP
            0xB8 => { self.cp(self.registers.b); 1 }
            0xB9 => { self.cp(self.registers.c); 1 }
            0xBA => { self.cp(self.registers.d); 1 }
            0xBB => { self.cp(self.registers.e); 1 }
            0xBC => { self.cp(self.registers.h); 1 }
            0xBD => { self.cp(self.registers.l); 1 }
            0xBE => { self.cp(self.bus.read_byte(self.registers.get_hl())); 1 }
            0xBF => { self.cp(self.registers.a); 1 }

            // INC
            0x04 => { self.registers.b = self.inc_8bit(self.registers.b); 1 }
            0x0C => { self.registers.c = self.inc_8bit(self.registers.c); 1 }
            0x14 => { self.registers.d = self.inc_8bit(self.registers.d); 1 }
            0x1C => { self.registers.e = self.inc_8bit(self.registers.e); 1 }
            0x24 => { self.registers.h = self.inc_8bit(self.registers.h); 1 }
            0x2C => { self.registers.l = self.inc_8bit(self.registers.l); 1 }
            0x34 => {
                let address = self.registers.get_hl();
                let value = self.inc_8bit(self.bus.read_byte(address));
                self.bus.write_byte(address, value);
                1
            }
            0x3C => {self.registers.a = self.inc_8bit(self.registers.a); 1 }
            0x03 => {
                let value = self.registers.get_bc();
                self.registers.set_bc(self.inc_16bit(value));
                1
            }
            0x13 => {
                let value = self.registers.get_de();
                self.registers.set_de(self.inc_16bit(value));
                1
            }
            0x23 => {
                let value = self.registers.get_hl();
                self.registers.set_hl(self.inc_16bit(value));
                1
            }
            0x33 => { self.sp = self.inc_16bit(self.sp); 1 }

            // CCF
            0x3F => { self.registers.f.carry = !self.registers.f.carry; 1 }

            // RRA
            0x1F => {
                self.registers.a = self.rotate_right_through_carry(self.registers.a, true);
                1
            }

            // RRCA
            0x0F => {
                self.registers.a = self.rotate_right(self.registers.a, true);
                1
            }

            // CPL
            0x2F => {
                self.registers.a = !self.registers.a;
                1
            }

            0x3D => {self.registers.a = self.dec_8bit(self.registers.a); 1 }
            0x05 => {self.registers.b = self.dec_8bit(self.registers.b); 1 }
            0x0D => {self.registers.c = self.dec_8bit(self.registers.c); 1 }
            0x15 => {self.registers.d = self.dec_8bit(self.registers.d); 1 }
            0x1D => {self.registers.e = self.dec_8bit(self.registers.e); 1 }
            0x25 => {self.registers.h = self.dec_8bit(self.registers.h); 1 }
            0x2D => {self.registers.l = self.dec_8bit(self.registers.l); 1 }
            0x35 => {
                let address = self.registers.get_hl();
                let value = self.dec_8bit(self.bus.read_byte(address));
                self.bus.write_byte(address, value);
                1
            }
            0x0B => {self.dec_16bit(self.registers.get_bc()); 1}
            0x1B => {self.dec_16bit(self.registers.get_de()); 1}
            0x2B => {self.dec_16bit(self.registers.get_hl()); 1}
            0x3B => {self.dec_16bit(self.sp); 1}

            0x97 => {self.sub(self.registers.a); 1 }
            0x90 => {self.sub(self.registers.b); 1 }
            0x91 => {self.sub(self.registers.c); 1 }
            0x92 => {self.sub(self.registers.d); 1 }
            0x93 => {self.sub(self.registers.e); 1 }
            0x94 => {self.sub(self.registers.h); 1 }
            0x95 => {self.sub(self.registers.l); 1 }
            0x96 => {self.sub(self.bus.read_byte(self.registers.get_hl())); 1}
            
            0x9F => {self.sbc(self.registers.a); 1 }
            0x98 => {self.sbc(self.registers.b); 1 }
            0x99 => {self.sbc(self.registers.c); 1 }
            0x9A => {self.sbc(self.registers.d); 1 }
            0x9B => {self.sbc(self.registers.e); 1 }
            0x9C => {self.sbc(self.registers.h); 1 }
            0x9D => {self.sbc(self.registers.l); 1 }
            0x9E => {self.sbc(self.bus.read_byte(self.registers.get_hl())); 1}

            0xAF => {self.xor(self.registers.a); 1 }
            0xA8 => {self.xor(self.registers.b); 1 }
            0xA9 => {self.xor(self.registers.c); 1 }
            0xAA => {self.xor(self.registers.d); 1 }
            0xAB => {self.xor(self.registers.e); 1 }
            0xAC => {self.xor(self.registers.h); 1 }
            0xAD => {self.xor(self.registers.l); 1 }
            0xAE => {self.xor(self.bus.read_byte(self.registers.get_hl())); 1}
            
            0xB7 => {self.or(self.registers.a); 1 }
            0xB0 => {self.or(self.registers.b); 1 }
            0xB1 => {self.or(self.registers.c); 1 }
            0xB2 => {self.or(self.registers.d); 1 }
            0xB3 => {self.or(self.registers.e); 1 }
            0xB4 => {self.or(self.registers.h); 1 }
            0xB5 => {self.or(self.registers.l); 1 }
            0xB6 => {self.or(self.bus.read_byte(self.registers.get_hl())); 1}
            
            _ => panic!("Unknown instruction found for: 0x{:x}", byte),
        }
    }

    fn execute_prefixed(&mut self, byte: u8) -> u16 {
        match byte {
            // RES
            0x80 => { self.registers.b = self.reset_bit(self.registers.b, 0); 2 }
            0x81 => { self.registers.c = self.reset_bit(self.registers.c, 0); 2 }
            0x82 => { self.registers.d = self.reset_bit(self.registers.d, 0); 2 }
            0x83 => { self.registers.e = self.reset_bit(self.registers.e, 0); 2 }
            0x84 => { self.registers.h = self.reset_bit(self.registers.h, 0); 2 }
            0x85 => { self.registers.l = self.reset_bit(self.registers.l, 0); 2 }
            0x86 => {
                let address = self.registers.get_hl();
                let value = self.reset_bit(self.bus.read_byte(address), 0);
                self.bus.write_byte(address, value);
                2
            }
            0x87 => { self.registers.a = self.reset_bit(self.registers.a, 0); 2 }

            0x88 => { self.registers.b = self.reset_bit(self.registers.b, 1); 2 }
            0x89 => { self.registers.c = self.reset_bit(self.registers.c, 1); 2 }
            0x8a => { self.registers.d = self.reset_bit(self.registers.d, 1); 2 }
            0x8b => { self.registers.e = self.reset_bit(self.registers.e, 1); 2 }
            0x8c => { self.registers.h = self.reset_bit(self.registers.h, 1); 2 }
            0x8d => { self.registers.l = self.reset_bit(self.registers.l, 1); 2 }
            0x8e => {
                let address = self.registers.get_hl();
                let value = self.reset_bit(self.bus.read_byte(address), 1);
                self.bus.write_byte(address, value);
                2
            }
            0x8f => { self.registers.a = self.reset_bit(self.registers.a, 1); 2 }

            0x90 => { self.registers.b = self.reset_bit(self.registers.b, 2); 2 }
            0x91 => { self.registers.c = self.reset_bit(self.registers.c, 2); 2 }
            0x92 => { self.registers.d = self.reset_bit(self.registers.d, 2); 2 }
            0x93 => { self.registers.e = self.reset_bit(self.registers.e, 2); 2 }
            0x94 => { self.registers.h = self.reset_bit(self.registers.h, 2); 2 }
            0x95 => { self.registers.l = self.reset_bit(self.registers.l, 2); 2 }
            0x96 => {
                let address = self.registers.get_hl();
                let value = self.reset_bit(self.bus.read_byte(address), 2);
                self.bus.write_byte(address, value);
                2
            }
            0x97 => { self.registers.a = self.reset_bit(self.registers.a, 2); 2 }

            0x98 => { self.registers.b = self.reset_bit(self.registers.b, 3); 2 }
            0x99 => { self.registers.c = self.reset_bit(self.registers.c, 3); 2 }
            0x9a => { self.registers.d = self.reset_bit(self.registers.d, 3); 2 }
            0x9b => { self.registers.e = self.reset_bit(self.registers.e, 3); 2 }
            0x9c => { self.registers.h = self.reset_bit(self.registers.h, 3); 2 }
            0x9d => { self.registers.l = self.reset_bit(self.registers.l, 3); 2 }
            0x9e => {
                let address = self.registers.get_hl();
                let value = self.reset_bit(self.bus.read_byte(address), 3);
                self.bus.write_byte(address, value);
                2
            }
            0x9f => { self.registers.a = self.reset_bit(self.registers.a, 3); 2 }

            0xa0 => { self.registers.b = self.reset_bit(self.registers.b, 4); 2 }
            0xa1 => { self.registers.c = self.reset_bit(self.registers.c, 4); 2 }
            0xa2 => { self.registers.d = self.reset_bit(self.registers.d, 4); 2 }
            0xa3 => { self.registers.e = self.reset_bit(self.registers.e, 4); 2 }
            0xa4 => { self.registers.h = self.reset_bit(self.registers.h, 4); 2 }
            0xa5 => { self.registers.l = self.reset_bit(self.registers.l, 4); 2 }
            0xa6 => {
                let address = self.registers.get_hl();
                let value = self.reset_bit(self.bus.read_byte(address), 4);
                self.bus.write_byte(address, value);
                2
            }
            0xa7 => { self.registers.a = self.reset_bit(self.registers.a, 4); 2 }

            0xa8 => { self.registers.b = self.reset_bit(self.registers.b, 5); 2 }
            0xa9 => { self.registers.c = self.reset_bit(self.registers.c, 5); 2 }
            0xaa => { self.registers.d = self.reset_bit(self.registers.d, 5); 2 }
            0xab => { self.registers.e = self.reset_bit(self.registers.e, 5); 2 }
            0xac => { self.registers.h = self.reset_bit(self.registers.h, 5); 2 }
            0xad => { self.registers.l = self.reset_bit(self.registers.l, 5); 2 }
            0xae => {
                let address = self.registers.get_hl();
                let value = self.reset_bit(self.bus.read_byte(address), 5);
                self.bus.write_byte(address, value);
                2
            }
            0xaf => { self.registers.a = self.reset_bit(self.registers.a, 5); 2 }

            0xb0 => { self.registers.b = self.reset_bit(self.registers.b, 6); 2 }
            0xb1 => { self.registers.c = self.reset_bit(self.registers.c, 6); 2 }
            0xb2 => { self.registers.d = self.reset_bit(self.registers.d, 6); 2 }
            0xb3 => { self.registers.e = self.reset_bit(self.registers.e, 6); 2 }
            0xb4 => { self.registers.h = self.reset_bit(self.registers.h, 6); 2 }
            0xb5 => { self.registers.l = self.reset_bit(self.registers.l, 6); 2 }
            0xb6 => {
                let address = self.registers.get_hl();
                let value = self.reset_bit(self.bus.read_byte(address), 6);
                self.bus.write_byte(address, value);
                2
            }
            0xb7 => { self.registers.a = self.reset_bit(self.registers.a, 6); 2 }

            0xb8 => { self.registers.b = self.reset_bit(self.registers.b, 7); 2 }
            0xb9 => { self.registers.c = self.reset_bit(self.registers.c, 7); 2 }
            0xba => { self.registers.d = self.reset_bit(self.registers.d, 7); 2 }
            0xbb => { self.registers.e = self.reset_bit(self.registers.e, 7); 2 }
            0xbc => { self.registers.h = self.reset_bit(self.registers.h, 7); 2 }
            0xbd => { self.registers.l = self.reset_bit(self.registers.l, 7); 2 }
            0xbe => {
                let address = self.registers.get_hl();
                let value = self.reset_bit(self.bus.read_byte(address), 7);
                self.bus.write_byte(address, value);
                2
            }
            0xbf => { self.registers.a = self.reset_bit(self.registers.a, 7); 2 }

            // SRL
            0x38 => { self.registers.b = self.shift_right_logical(self.registers.b); 2 }
            0x39 => { self.registers.c = self.shift_right_logical(self.registers.c); 2 }
            0x3a => { self.registers.d = self.shift_right_logical(self.registers.d); 2 }
            0x3b => { self.registers.e = self.shift_right_logical(self.registers.e); 2 }
            0x3c => { self.registers.h = self.shift_right_logical(self.registers.h); 2 }
            0x3d => { self.registers.l = self.shift_right_logical(self.registers.l); 2 }
            0x3e => {
                let address = self.registers.get_hl();
                let value = self.shift_right_logical(self.bus.read_byte(address));
                self.bus.write_byte(address, value);
                2
            }
            0x3f => { self.registers.a = self.shift_right_logical(self.registers.a); 2 }

            _ => panic!("Unknown instruction found for: 0x{:x}", byte),
        }
    }

    fn add(&mut self, value: u8, with_carry: bool) {
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
        self.registers.a = new_value2;
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

    fn and(&mut self, value: u8) {
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
        self.registers.a = new_value;
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
        self.registers.f.half_carry = value & 0xF == 0xF;
        self.registers.f.carry = false;
        new_value
    }

    fn inc_16bit(&mut self, value: u16) -> u16 {
        value.wrapping_add(1)
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

    fn reset_bit(&mut self, value: u8, bit_position: u8) -> u8 {
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
    
    fn sub(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.a = new_value;
    }
    
    fn sbc(&mut self, value: u8) {
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
        self.registers.a = new_value2;
    }

    fn or(&mut self, value: u8) {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        self.registers.a = new_value;
    }
    
    fn xor(&mut self, value: u8) {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        self.registers.a = new_value;
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
    
    fn test_bit(&mut self, value: u8, bit_pos: u8) {
        let new_value = (value >> bit_pos) & 0b1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }
    
    fn set_bit(&mut self, value: u8, bit_pos: u8) -> u8 {
        value | (1 << bit_pos)
    }
}
