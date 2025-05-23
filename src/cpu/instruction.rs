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
            // NOP
            0x00 => { 1 }

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

            // ADDHL
            0x09 => {
                let value = self.registers.get_bc();
                self.add_hl(value);
                1
            }
            0x19 => {
                let value = self.registers.get_de();
                self.add_hl(value);
                1
            }
            0x29 => {
                let value = self.registers.get_hl();
                self.add_hl(value);
                1
            }
            0x39 => { self.add_hl(self.sp); 1 }

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
            0x03 => { self.registers.set_af(CPU::inc_16bit(self.registers.get_bc())); 1 }
            0x13 => { self.registers.set_de(CPU::inc_16bit(self.registers.get_de())); 1 }
            0x23 => { self.registers.set_hl(CPU::inc_16bit(self.registers.get_hl())); 1 }
            0x33 => { self.sp = CPU::inc_16bit(self.sp); 1 }

            // CCF
            0x3F => { self.registers.f.carry = !self.registers.f.carry; 1 }

            // RRA
            0x1F => {
                self.registers.a = self.rotate_right_circular(self.registers.a, true);
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

            // SUB
            0x97 => { self.sub(self.registers.a); 1 }
            0x90 => { self.sub(self.registers.b); 1 }
            0x91 => { self.sub(self.registers.c); 1 }
            0x92 => { self.sub(self.registers.d); 1 }
            0x93 => { self.sub(self.registers.e); 1 }
            0x94 => { self.sub(self.registers.h); 1 }
            0x95 => { self.sub(self.registers.l); 1 }
            0x96 => { self.sub(self.bus.read_byte(self.registers.get_hl())); 1 }
            
            // SBC
            0x9F => { self.sbc(self.registers.a); 1 }
            0x98 => { self.sbc(self.registers.b); 1 }
            0x99 => { self.sbc(self.registers.c); 1 }
            0x9A => { self.sbc(self.registers.d); 1 }
            0x9B => { self.sbc(self.registers.e); 1 }
            0x9C => { self.sbc(self.registers.h); 1 }
            0x9D => { self.sbc(self.registers.l); 1 }
            0x9E => { self.sbc(self.bus.read_byte(self.registers.get_hl())); 1 }
            
            // OR
            0xB7 => { self.or(self.registers.a); 1 }
            0xB0 => { self.or(self.registers.b); 1 }
            0xB1 => { self.or(self.registers.c); 1 }
            0xB2 => { self.or(self.registers.d); 1 }
            0xB3 => { self.or(self.registers.e); 1 }
            0xB4 => { self.or(self.registers.h); 1 }
            0xB5 => { self.or(self.registers.l); 1 }
            0xB6 => { self.or(self.bus.read_byte(self.registers.get_hl())); 1 }
            
            // XOR
            0xAF => { self.xor(self.registers.a); 1 }
            0xA8 => { self.xor(self.registers.b); 1 }
            0xA9 => { self.xor(self.registers.c); 1 }
            0xAA => { self.xor(self.registers.d); 1 }
            0xAB => { self.xor(self.registers.e); 1 }
            0xAC => { self.xor(self.registers.h); 1 }
            0xAD => { self.xor(self.registers.l); 1 }
            0xAE => { self.xor(self.bus.read_byte(self.registers.get_hl())); 1 }

            // DEC
            0x3D => { self.registers.a = self.dec_8bit(self.registers.a); 1 }
            0x05 => { self.registers.b = self.dec_8bit(self.registers.b); 1 }
            0x0D => { self.registers.c = self.dec_8bit(self.registers.c); 1 }
            0x15 => { self.registers.d = self.dec_8bit(self.registers.d); 1 }
            0x1D => { self.registers.e = self.dec_8bit(self.registers.e); 1 }
            0x25 => { self.registers.h = self.dec_8bit(self.registers.h); 1 }
            0x2D => { self.registers.l = self.dec_8bit(self.registers.l); 1 }
            0x35 => {
                let address = self.registers.get_hl();
                let value = self.dec_8bit(self.bus.read_byte(address));
                self.bus.write_byte(address, value);
                1
            }
            0x0B => { self.registers.set_af(CPU::dec_16bit(self.registers.get_bc())); 1 }
            0x1B => { self.registers.set_de(CPU::dec_16bit(self.registers.get_de())); 1 }
            0x2B => { self.registers.set_hl(CPU::dec_16bit(self.registers.get_hl())); 1 }
            0x3B => { self.sp = CPU::dec_16bit(self.sp); 1 }

            // SCF
            0x37 => { self.scf(); 1 }
            
            // RLA
            0x17 => { self.registers.a = self.rotate_left_circular(self.registers.a, false); 1 }
            
            // RLCA
            0x07 => { self.registers.a = self.rotate_left(self.registers.a, false); 1 }
            
            // JP
            0xC3 => { self.jump(true); 0 }
            0xC2 => { self.jump(self.registers.f.zero == false); 0 }
            0xD2 => { self.jump(self.registers.f.carry == false); 0 }
            0xCA => { self.jump(self.registers.f.zero == true); 0 }
            0xDA => { self.jump(self.registers.f.carry == true); 0 }
            0xE9 => { self.pc = self.registers.get_hl(); 0 }

            // JR
            0x18 => { self.jump_relative(true); 0 }
            0x20 => { self.jump_relative(self.registers.f.zero == false); 0 }
            0x30 => { self.jump_relative(self.registers.f.carry == false); 0 }
            0x28 => { self.jump_relative(self.registers.f.zero == true); 0 }
            0x38 => { self.jump_relative(self.registers.f.carry == true); 0 }

            // LD immediate to register
            0x06 => { self.registers.b = self.bus.read_byte(self.pc + 1); 2 }
            0x0E => { self.registers.c = self.bus.read_byte(self.pc + 1); 2 }
            0x16 => { self.registers.d = self.bus.read_byte(self.pc + 1); 2 }
            0x1E => { self.registers.e = self.bus.read_byte(self.pc + 1); 2 }
            0x2E => { self.registers.h = self.bus.read_byte(self.pc + 1); 2 }
            0x26 => { self.registers.l = self.bus.read_byte(self.pc + 1); 2 }
            0x3E => {
                let address = self.registers.get_hl();
                let value = self.bus.read_byte(self.pc + 1);
                self.bus.write_byte(address, value);
                2
            }
            0x36 => { self.registers.a = self.bus.read_byte(self.pc + 1); 2 }
            0x01 => { self.registers.set_bc(self.get_d16()); 3 }
            0x11 => { self.registers.set_de(self.get_d16()); 3 }
            0x21 => { self.registers.set_hl(self.get_d16()); 3 }
            0x31 => { self.sp = self.get_d16(); 3 }

            // LD A
            0x02 => { let address = self.registers.get_bc(); self.bus.write_byte(address, self.registers.a); 1 }
            0x12 => { let address = self.registers.get_de(); self.bus.write_byte(address, self.registers.a); 1 }
            0x22 => { let address = self.registers.get_hl(); self.bus.write_byte(address, self.registers.a); self.registers.set_hl(address + 1); 1 }
            0x32 => { let address = self.registers.get_hl(); self.bus.write_byte(address, self.registers.a); self.registers.set_hl(address - 1); 1 }

            0x0A => { self.registers.a = self.bus.read_byte(self.registers.get_bc()); 1 }
            0x1A => { self.registers.a = self.bus.read_byte(self.registers.get_de()); 1 }
            0x2A => { let address = self.registers.get_hl(); self.registers.a = self.bus.read_byte(address); self.registers.set_hl(address + 1); 1 }
            0x3A => { let address = self.registers.get_hl(); self.registers.a = self.bus.read_byte(address); self.registers.set_hl(address - 1); 1 }

            // LD SP
            0x08 => {
                let address = self.get_d16();
                let lower_byte = (self.sp & 0xFF) as u8;
                let upper_byte = ((self.sp & 0xFF00) >> 2) as u8;
                self.bus.write_byte(address, lower_byte);
                self.bus.write_byte(address, upper_byte);
                3
            }
            
            // LD from A to a8 and vice versa
            0xE0 => {
                let address = self.bus.read_byte(self.pc + 1) as u16 | 0xFF00;
                self.bus.write_byte(address, self.registers.a);
                2
            }
            0xF0 => {
                self.registers.a = self.bus.read_byte(self.pc + 1);
                2
            }
            
            // LD from A to C and vice versa
            0xE2 => {
                let address = self.registers.c as u16 | 0xFF00;
                self.bus.write_byte(address, self.registers.a);
                1
            }
            0xF2 => {
                self.registers.a = self.bus.read_byte(self.registers.c as u16 | 0xFF00);
                1
            }
            
            // LD A to a16 and vice versa
            0xE2 => {
                let address = self.get_d16();
                self.bus.write_byte(address, self.registers.a);
                3
            }
            0xF2 => {
                self.registers.a = self.bus.read_byte(self.get_d16());
                3
            }

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

            // RR
            0x18 => { self.registers.b = self.rotate_right_circular(self.registers.b, false); 2 }
            0x19 => { self.registers.c = self.rotate_right_circular(self.registers.c, false); 2 }
            0x1a => { self.registers.d = self.rotate_right_circular(self.registers.d, false); 2 }
            0x1b => { self.registers.e = self.rotate_right_circular(self.registers.e, false); 2 }
            0x1c => { self.registers.h = self.rotate_right_circular(self.registers.h, false); 2 }
            0x1d => { self.registers.l = self.rotate_right_circular(self.registers.l, false); 2 }
            0x1e => {
                let address = self.registers.get_hl();
                let value = self.rotate_right_circular(self.bus.read_byte(address), false);
                self.bus.write_byte(address, value);
                2
            }
            0x1f => { self.registers.a = self.rotate_right_circular(self.registers.a, false); 2 }

            // RRC
            0x08 => { self.registers.b = self.rotate_right(self.registers.b, false); 2 }
            0x09 => { self.registers.c = self.rotate_right(self.registers.c, false); 2 }
            0x0a => { self.registers.d = self.rotate_right(self.registers.d, false); 2 }
            0x0b => { self.registers.e = self.rotate_right(self.registers.e, false); 2 }
            0x0c => { self.registers.h = self.rotate_right(self.registers.h, false); 2 }
            0x0d => { self.registers.l = self.rotate_right(self.registers.l, false); 2 }
            0x0e => {
                let address = self.registers.get_hl();
                let value = self.rotate_right(self.bus.read_byte(address), false);
                self.bus.write_byte(address, value);
                2
            }
            0x0f => { self.registers.a = self.rotate_right(self.registers.a, false); 2 }

            // SRA
            0x28 => { self.registers.b = self.shift_right_arithmetic(self.registers.b); 2 }
            0x29 => { self.registers.c = self.shift_right_arithmetic(self.registers.c); 2 }
            0x2a => { self.registers.d = self.shift_right_arithmetic(self.registers.d); 2 }
            0x2b => { self.registers.e = self.shift_right_arithmetic(self.registers.e); 2 }
            0x2c => { self.registers.h = self.shift_right_arithmetic(self.registers.h); 2 }
            0x2d => { self.registers.l = self.shift_right_arithmetic(self.registers.l); 2 }
            0x2e => {
                let address = self.registers.get_hl();
                let value = self.shift_right_arithmetic(self.bus.read_byte(address));
                self.bus.write_byte(address, value);
                2
            }
            0x2f => { self.registers.a = self.shift_right_arithmetic(self.registers.a); 2 }
            
            // BIT
            0x40 => { self.test_bit(self.registers.b,                            0); 2 }
            0x41 => { self.test_bit(self.registers.c,                            0); 2 }
            0x42 => { self.test_bit(self.registers.d,                            0); 2 }
            0x43 => { self.test_bit(self.registers.e,                            0); 2 }
            0x44 => { self.test_bit(self.registers.h,                            0); 2 }
            0x45 => { self.test_bit(self.registers.l,                            0); 2 }
            0x46 => { self.test_bit(self.bus.read_byte(self.registers.get_hl()), 0); 2 }
            0x47 => { self.test_bit(self.registers.a,                            0); 2 }
            0x48 => { self.test_bit(self.registers.b,                            1); 2 }
            0x49 => { self.test_bit(self.registers.c,                            1); 2 }
            0x4a => { self.test_bit(self.registers.d,                            1); 2 }
            0x4b => { self.test_bit(self.registers.e,                            1); 2 }
            0x4c => { self.test_bit(self.registers.h,                            1); 2 }
            0x4d => { self.test_bit(self.registers.l,                            1); 2 }
            0x4e => { self.test_bit(self.bus.read_byte(self.registers.get_hl()), 1); 2 }
            0x4f => { self.test_bit(self.registers.a,                            1); 2 }
            0x50 => { self.test_bit(self.registers.b,                            2); 2 }
            0x51 => { self.test_bit(self.registers.c,                            2); 2 }
            0x52 => { self.test_bit(self.registers.d,                            2); 2 }
            0x53 => { self.test_bit(self.registers.e,                            2); 2 }
            0x54 => { self.test_bit(self.registers.h,                            2); 2 }
            0x55 => { self.test_bit(self.registers.l,                            2); 2 }
            0x56 => { self.test_bit(self.bus.read_byte(self.registers.get_hl()), 2); 2 }
            0x57 => { self.test_bit(self.registers.a,                            2); 2 }
            0x58 => { self.test_bit(self.registers.b,                            3); 2 }
            0x59 => { self.test_bit(self.registers.c,                            3); 2 }
            0x5a => { self.test_bit(self.registers.d,                            3); 2 }
            0x5b => { self.test_bit(self.registers.e,                            3); 2 }
            0x5c => { self.test_bit(self.registers.h,                            3); 2 }
            0x5d => { self.test_bit(self.registers.l,                            3); 2 }
            0x5e => { self.test_bit(self.bus.read_byte(self.registers.get_hl()), 3); 2 }
            0x5f => { self.test_bit(self.registers.a,                            3); 2 }
            0x60 => { self.test_bit(self.registers.b,                            4); 2 }
            0x61 => { self.test_bit(self.registers.c,                            4); 2 }
            0x62 => { self.test_bit(self.registers.d,                            4); 2 }
            0x63 => { self.test_bit(self.registers.e,                            4); 2 }
            0x64 => { self.test_bit(self.registers.h,                            4); 2 }
            0x65 => { self.test_bit(self.registers.l,                            4); 2 }
            0x66 => { self.test_bit(self.bus.read_byte(self.registers.get_hl()), 4); 2 }
            0x67 => { self.test_bit(self.registers.a,                            4); 2 }
            0x68 => { self.test_bit(self.registers.b,                            5); 2 }
            0x69 => { self.test_bit(self.registers.c,                            5); 2 }
            0x6a => { self.test_bit(self.registers.d,                            5); 2 }
            0x6b => { self.test_bit(self.registers.e,                            5); 2 }
            0x6c => { self.test_bit(self.registers.h,                            5); 2 }
            0x6d => { self.test_bit(self.registers.l,                            5); 2 }
            0x6e => { self.test_bit(self.bus.read_byte(self.registers.get_hl()), 5); 2 }
            0x6f => { self.test_bit(self.registers.a,                            5); 2 }
            0x70 => { self.test_bit(self.registers.b,                            6); 2 }
            0x71 => { self.test_bit(self.registers.c,                            6); 2 }
            0x72 => { self.test_bit(self.registers.d,                            6); 2 }
            0x73 => { self.test_bit(self.registers.e,                            6); 2 }
            0x74 => { self.test_bit(self.registers.h,                            6); 2 }
            0x75 => { self.test_bit(self.registers.l,                            6); 2 }
            0x76 => { self.test_bit(self.bus.read_byte(self.registers.get_hl()), 6); 2 }
            0x77 => { self.test_bit(self.registers.a,                            6); 2 }
            0x78 => { self.test_bit(self.registers.b,                            7); 2 }
            0x79 => { self.test_bit(self.registers.c,                            7); 2 }
            0x7a => { self.test_bit(self.registers.d,                            7); 2 }
            0x7b => { self.test_bit(self.registers.e,                            7); 2 }
            0x7c => { self.test_bit(self.registers.h,                            7); 2 }
            0x7d => { self.test_bit(self.registers.l,                            7); 2 }
            0x7e => { self.test_bit(self.bus.read_byte(self.registers.get_hl()), 7); 2 }
            0x7f => { self.test_bit(self.registers.a,                            7); 2 }
            
            // SET
            0xC0 => { self.registers.b = CPU::set_bit(self.registers.b,                                                             0); 2 }
            0xC1 => { self.registers.c = CPU::set_bit(self.registers.c,                                                             0); 2 }
            0xC2 => { self.registers.d = CPU::set_bit(self.registers.d,                                                             0); 2 }
            0xC3 => { self.registers.e = CPU::set_bit(self.registers.e,                                                             0); 2 }
            0xC4 => { self.registers.h = CPU::set_bit(self.registers.h,                                                             0); 2 }
            0xC5 => { self.registers.l = CPU::set_bit(self.registers.l,                                                             0); 2 }
            0xC6 => { let address = self.registers.get_hl(); self.bus.write_byte(address, CPU::set_bit(self.bus.read_byte(address), 0)); 2 }
            0xC7 => { self.registers.a = CPU::set_bit(self.registers.a,                                                             0); 2 }
            0xC8 => { self.registers.b = CPU::set_bit(self.registers.b,                                                             1); 2 }
            0xC9 => { self.registers.c = CPU::set_bit(self.registers.c,                                                             1); 2 }
            0xCA => { self.registers.d = CPU::set_bit(self.registers.d,                                                             1); 2 }
            0xCB => { self.registers.e = CPU::set_bit(self.registers.e,                                                             1); 2 }
            0xCC => { self.registers.h = CPU::set_bit(self.registers.h,                                                             1); 2 }
            0xCD => { self.registers.l = CPU::set_bit(self.registers.l,                                                             1); 2 }
            0xCE => { let address = self.registers.get_hl(); self.bus.write_byte(address, CPU::set_bit(self.bus.read_byte(address), 1)); 2 }
            0xCF => { self.registers.a = CPU::set_bit(self.registers.a,                                                             1); 2 }
            0xD0 => { self.registers.b = CPU::set_bit(self.registers.b,                                                             2); 2 }
            0xD1 => { self.registers.c = CPU::set_bit(self.registers.c,                                                             2); 2 }
            0xD2 => { self.registers.d = CPU::set_bit(self.registers.d,                                                             2); 2 }
            0xD3 => { self.registers.e = CPU::set_bit(self.registers.e,                                                             2); 2 }
            0xD4 => { self.registers.h = CPU::set_bit(self.registers.h,                                                             2); 2 }
            0xD5 => { self.registers.l = CPU::set_bit(self.registers.l,                                                             2); 2 }
            0xD6 => { let address = self.registers.get_hl(); self.bus.write_byte(address, CPU::set_bit(self.bus.read_byte(address), 2)); 2 }
            0xD7 => { self.registers.a = CPU::set_bit(self.registers.a,                                                             2); 2 }
            0xD8 => { self.registers.b = CPU::set_bit(self.registers.b,                                                             3); 2 }
            0xD9 => { self.registers.c = CPU::set_bit(self.registers.c,                                                             3); 2 }
            0xDA => { self.registers.d = CPU::set_bit(self.registers.d,                                                             3); 2 }
            0xDB => { self.registers.e = CPU::set_bit(self.registers.e,                                                             3); 2 }
            0xDC => { self.registers.h = CPU::set_bit(self.registers.h,                                                             3); 2 }
            0xDD => { self.registers.l = CPU::set_bit(self.registers.l,                                                             3); 2 }
            0xDE => { let address = self.registers.get_hl(); self.bus.write_byte(address, CPU::set_bit(self.bus.read_byte(address), 3)); 2 }
            0xDF => { self.registers.a = CPU::set_bit(self.registers.a,                                                             3); 2 }
            0xE0 => { self.registers.b = CPU::set_bit(self.registers.b,                                                             4); 2 }
            0xE1 => { self.registers.c = CPU::set_bit(self.registers.c,                                                             4); 2 }
            0xE2 => { self.registers.d = CPU::set_bit(self.registers.d,                                                             4); 2 }
            0xE3 => { self.registers.e = CPU::set_bit(self.registers.e,                                                             4); 2 }
            0xE4 => { self.registers.h = CPU::set_bit(self.registers.h,                                                             4); 2 }
            0xE5 => { self.registers.l = CPU::set_bit(self.registers.l,                                                             4); 2 }
            0xE6 => { let address = self.registers.get_hl(); self.bus.write_byte(address, CPU::set_bit(self.bus.read_byte(address), 4)); 2 }
            0xE7 => { self.registers.a = CPU::set_bit(self.registers.a,                                                             4); 2 }
            0xE8 => { self.registers.b = CPU::set_bit(self.registers.b,                                                             5); 2 }
            0xE9 => { self.registers.c = CPU::set_bit(self.registers.c,                                                             5); 2 }
            0xEA => { self.registers.d = CPU::set_bit(self.registers.d,                                                             5); 2 }
            0xEB => { self.registers.e = CPU::set_bit(self.registers.e,                                                             5); 2 }
            0xEC => { self.registers.h = CPU::set_bit(self.registers.h,                                                             5); 2 }
            0xED => { self.registers.l = CPU::set_bit(self.registers.l,                                                             5); 2 }
            0xEE => { let address = self.registers.get_hl(); self.bus.write_byte(address, CPU::set_bit(self.bus.read_byte(address), 5)); 2 }
            0xEF => { self.registers.a = CPU::set_bit(self.registers.a,                                                             5); 2 }
            0xF0 => { self.registers.b = CPU::set_bit(self.registers.b,                                                             6); 2 }
            0xF1 => { self.registers.c = CPU::set_bit(self.registers.c,                                                             6); 2 }
            0xF2 => { self.registers.d = CPU::set_bit(self.registers.d,                                                             6); 2 }
            0xF3 => { self.registers.e = CPU::set_bit(self.registers.e,                                                             6); 2 }
            0xF4 => { self.registers.h = CPU::set_bit(self.registers.h,                                                             6); 2 }
            0xF5 => { self.registers.l = CPU::set_bit(self.registers.l,                                                             6); 2 }
            0xF6 => { let address = self.registers.get_hl(); self.bus.write_byte(address, CPU::set_bit(self.bus.read_byte(address), 6)); 2 }
            0xF7 => { self.registers.a = CPU::set_bit(self.registers.a,                                                             6); 2 }
            0xF8 => { self.registers.b = CPU::set_bit(self.registers.b,                                                             7); 2 }
            0xF9 => { self.registers.c = CPU::set_bit(self.registers.c,                                                             7); 2 }
            0xFA => { self.registers.d = CPU::set_bit(self.registers.d,                                                             7); 2 }
            0xFB => { self.registers.e = CPU::set_bit(self.registers.e,                                                             7); 2 }
            0xFC => { self.registers.h = CPU::set_bit(self.registers.h,                                                             7); 2 }
            0xFD => { self.registers.l = CPU::set_bit(self.registers.l,                                                             7); 2 }
            0xFE => { let address = self.registers.get_hl(); self.bus.write_byte(address, CPU::set_bit(self.bus.read_byte(address), 7)); 2 }
            0xFF => { self.registers.a = CPU::set_bit(self.registers.a,                                                             7); 2 }
            
            // RL
            0x10 => { self.registers.b = self.rotate_left(self.registers.b, false); 2 }
            0x11 => { self.registers.c = self.rotate_left(self.registers.c, false); 2 }
            0x12 => { self.registers.d = self.rotate_left(self.registers.d, false); 2 }
            0x13 => { self.registers.e = self.rotate_left(self.registers.e, false); 2 }
            0x14 => { self.registers.h = self.rotate_left(self.registers.h, false); 2 }
            0x15 => { self.registers.l = self.rotate_left(self.registers.l, false); 2 }
            0x16 => {
                let address = self.registers.get_hl();
                let value = self.rotate_left(self.bus.read_byte(address), true);
                self.bus.write_byte(address, value);
                2
            }
            0x17 => { self.registers.a = self.rotate_left(self.registers.a, false); 2 }
            
            // RLC
            0x00 => { self.registers.b = self.rotate_left(self.registers.b, false); 2 }
            0x01 => { self.registers.c = self.rotate_left(self.registers.c, false); 2 }
            0x02 => { self.registers.d = self.rotate_left(self.registers.d, false); 2 }
            0x03 => { self.registers.e = self.rotate_left(self.registers.e, false); 2 }
            0x04 => { self.registers.h = self.rotate_left(self.registers.h, false); 2 }
            0x05 => { self.registers.l = self.rotate_left(self.registers.l, false); 2 }
            0x06 => {
                let address = self.registers.get_hl();
                let value = self.rotate_left(self.bus.read_byte(address), true);
                self.bus.write_byte(address, value);
                2
            }
            0x07 => { self.registers.a = self.rotate_left(self.registers.a, false); 2 }
            
            // SLA
            0x20 => { self.registers.b = self.shift_left_arithmetic(self.registers.b); 2 }
            0x21 => { self.registers.c = self.shift_left_arithmetic(self.registers.c); 2 }
            0x22 => { self.registers.d = self.shift_left_arithmetic(self.registers.d); 2 }
            0x23 => { self.registers.e = self.shift_left_arithmetic(self.registers.e); 2 }
            0x24 => { self.registers.h = self.shift_left_arithmetic(self.registers.h); 2 }
            0x25 => { self.registers.l = self.shift_left_arithmetic(self.registers.l); 2 }
            0x26 => {
                let address = self.registers.get_hl();
                let value = self.shift_left_arithmetic(self.bus.read_byte(address));
                self.bus.write_byte(address, value);
                2
            }
            0x27 => { self.registers.a = self.shift_left_arithmetic(self.registers.a); 2 }
            
            // SWAP
            0x30 => { self.registers.b = self.swap(self.registers.b); 2 }
            0x31 => { self.registers.c = self.swap(self.registers.c); 2 }
            0x32 => { self.registers.d = self.swap(self.registers.d); 2 }
            0x33 => { self.registers.e = self.swap(self.registers.e); 2 }
            0x34 => { self.registers.h = self.swap(self.registers.h); 2 }
            0x35 => { self.registers.l = self.swap(self.registers.l); 2 }
            0x36 => {
                let address = self.registers.get_hl();
                let value = self.swap(self.bus.read_byte(address));
                self.bus.write_byte(address, value);
                2
            }
            0x37 => { self.registers.a = self.swap(self.registers.a); 2 }
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

    fn add_hl(&mut self, value: u16) {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.get_hl() & 0b111_1111_1111)
            + (value & 0b111_1111_1111)
            > 0b111_1111_1111;
        self.registers.f.carry = did_overflow;
        self.registers.set_hl(new_value);
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

    fn inc_16bit(value: u16) -> u16 {
        value.wrapping_add(1)
    }

    fn rotate_right_circular(&mut self, value: u8, set_zero: bool) -> u8 {
        let new_value = value.rotate_right(1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    fn rotate_right(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = (self.registers.f.carry as u8) << 7;
        let new_value = carry_bit | (value >> 1);
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

    fn dec_16bit(value: u16) -> u16 {
        value.wrapping_sub(1)
    }

    fn scf(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = true;
    }

    fn rotate_left_circular(&mut self, value: u8, set_zero: bool) -> u8 {
        let new_value = value.rotate_left(1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0x80 == 0x80;
        new_value
    }
    fn rotate_left(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = if self.registers.f.carry { 1 } else { 0 };
        let new_value = carry_bit | (value << 1);
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
    
    fn set_bit(value: u8, bit_pos: u8) -> u8 {
        value | (1 << bit_pos)
    }
    
    fn shift_left_arithmetic(&mut self, value: u8) -> u8 {
        let new_value = value << 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0x80 == 0x80;
        new_value
    }
    
    fn swap(&mut self, value: u8) -> u8 {
        let new_value = value >> 4 | value << 4;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }

    fn get_d16(&self) -> u16 {
        let lsb = self.bus.read_byte(self.pc + 1) as u16;
        let msb = self.bus.read_byte(self.pc + 2) as u16;
        lsb | (msb << 8)
    }
    
    fn jump(&mut self, should_jump: bool) {
        if should_jump {
            self.pc = self.get_d16();
        } else {
            self.pc = self.pc.wrapping_add(3);
        }
    }

    fn jump_relative(&mut self, should_jump: bool) {
        if should_jump {
            self.pc = self.pc.wrapping_add_signed(self.bus.read_byte(self.pc + 1) as i16);
        } else {
            self.pc = self.pc.wrapping_add(1)
        }
    }
}
