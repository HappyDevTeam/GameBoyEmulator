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
            0x80 => { self.add(self.regs.b, false); 1 }
            0x81 => { self.add(self.regs.c, false); 1 }
            0x82 => { self.add(self.regs.d, false); 1 }
            0x83 => { self.add(self.regs.e, false); 1 }
            0x84 => { self.add(self.regs.h, false); 1 }
            0x85 => { self.add(self.regs.l, false); 1 }
            0x86 => { self.add(self.bus.read_byte(self.regs.get_hl()), false); 1 }
            0x87 => { self.add(self.regs.a, false); 1 }

            // ADC
            0x88 => { self.add(self.regs.b, true); 1 }
            0x89 => { self.add(self.regs.c, true); 1 }
            0x8A => { self.add(self.regs.d, true); 1 }
            0x8B => { self.add(self.regs.e, true); 1 }
            0x8C => { self.add(self.regs.h, true); 1 }
            0x8D => { self.add(self.regs.l, true); 1 }
            0x8E => { self.add(self.bus.read_byte(self.regs.get_hl()), true); 1 }
            0x8F => { self.add(self.regs.a, true); 1 }

            // ADDHL
            0x09 => {
                let value = self.regs.get_bc();
                self.add_hl(value);
                1
            }
            0x19 => {
                let value = self.regs.get_de();
                self.add_hl(value);
                1
            }
            0x29 => {
                let value = self.regs.get_hl();
                self.add_hl(value);
                1
            }
            0x39 => { self.add_hl(self.sp); 1 }

            // AND
            0xA0 => { self.and(self.regs.b); 1 }
            0xA1 => { self.and(self.regs.c); 1 }
            0xA2 => { self.and(self.regs.d); 1 }
            0xA3 => { self.and(self.regs.e); 1 }
            0xA4 => { self.and(self.regs.h); 1 }
            0xA5 => { self.and(self.regs.l); 1 }
            0xA6 => { self.and(self.bus.read_byte(self.regs.get_hl())); 1 }
            0xA7 => { self.and(self.regs.b); 1 }

            // CP
            0xB8 => { self.cp(self.regs.b); 1 }
            0xB9 => { self.cp(self.regs.c); 1 }
            0xBA => { self.cp(self.regs.d); 1 }
            0xBB => { self.cp(self.regs.e); 1 }
            0xBC => { self.cp(self.regs.h); 1 }
            0xBD => { self.cp(self.regs.l); 1 }
            0xBE => { self.cp(self.bus.read_byte(self.regs.get_hl())); 1 }
            0xBF => { self.cp(self.regs.a); 1 }

            // INC
            0x04 => { self.regs.b = self.inc_8bit(self.regs.b); 1 }
            0x0C => { self.regs.c = self.inc_8bit(self.regs.c); 1 }
            0x14 => { self.regs.d = self.inc_8bit(self.regs.d); 1 }
            0x1C => { self.regs.e = self.inc_8bit(self.regs.e); 1 }
            0x24 => { self.regs.h = self.inc_8bit(self.regs.h); 1 }
            0x2C => { self.regs.l = self.inc_8bit(self.regs.l); 1 }
            0x34 => {
                let addr = self.regs.get_hl();
                let value = self.inc_8bit(self.bus.read_byte(addr));
                self.bus.write_byte(addr, value);
                1
            }
            0x3C => {self.regs.a = self.inc_8bit(self.regs.a); 1 }
            0x03 => { self.regs.set_af(CPU::inc_16bit(self.regs.get_bc())); 1 }
            0x13 => { self.regs.set_de(CPU::inc_16bit(self.regs.get_de())); 1 }
            0x23 => { self.regs.set_hl(CPU::inc_16bit(self.regs.get_hl())); 1 }
            0x33 => { self.sp = CPU::inc_16bit(self.sp); 1 }

            // CCF
            0x3F => { self.regs.f.carry = !self.regs.f.carry; 1 }

            // RRA
            0x1F => {
                self.regs.a = self.rotate_right_circular(self.regs.a, true);
                1
            }

            // RRCA
            0x0F => {
                self.regs.a = self.rotate_right(self.regs.a, true);
                1
            }

            // CPL
            0x2F => {
                self.regs.a = !self.regs.a;
                1
            }

            // SUB
            0x97 => { self.sub(self.regs.a); 1 }
            0x90 => { self.sub(self.regs.b); 1 }
            0x91 => { self.sub(self.regs.c); 1 }
            0x92 => { self.sub(self.regs.d); 1 }
            0x93 => { self.sub(self.regs.e); 1 }
            0x94 => { self.sub(self.regs.h); 1 }
            0x95 => { self.sub(self.regs.l); 1 }
            0x96 => { self.sub(self.bus.read_byte(self.regs.get_hl())); 1 }
            
            // SBC
            0x9F => { self.sbc(self.regs.a); 1 }
            0x98 => { self.sbc(self.regs.b); 1 }
            0x99 => { self.sbc(self.regs.c); 1 }
            0x9A => { self.sbc(self.regs.d); 1 }
            0x9B => { self.sbc(self.regs.e); 1 }
            0x9C => { self.sbc(self.regs.h); 1 }
            0x9D => { self.sbc(self.regs.l); 1 }
            0x9E => { self.sbc(self.bus.read_byte(self.regs.get_hl())); 1 }
            
            // OR
            0xB7 => { self.or(self.regs.a); 1 }
            0xB0 => { self.or(self.regs.b); 1 }
            0xB1 => { self.or(self.regs.c); 1 }
            0xB2 => { self.or(self.regs.d); 1 }
            0xB3 => { self.or(self.regs.e); 1 }
            0xB4 => { self.or(self.regs.h); 1 }
            0xB5 => { self.or(self.regs.l); 1 }
            0xB6 => { self.or(self.bus.read_byte(self.regs.get_hl())); 1 }
            
            // XOR
            0xAF => { self.xor(self.regs.a); 1 }
            0xA8 => { self.xor(self.regs.b); 1 }
            0xA9 => { self.xor(self.regs.c); 1 }
            0xAA => { self.xor(self.regs.d); 1 }
            0xAB => { self.xor(self.regs.e); 1 }
            0xAC => { self.xor(self.regs.h); 1 }
            0xAD => { self.xor(self.regs.l); 1 }
            0xAE => { self.xor(self.bus.read_byte(self.regs.get_hl())); 1 }

            // DEC
            0x3D => { self.regs.a = self.dec_8bit(self.regs.a); 1 }
            0x05 => { self.regs.b = self.dec_8bit(self.regs.b); 1 }
            0x0D => { self.regs.c = self.dec_8bit(self.regs.c); 1 }
            0x15 => { self.regs.d = self.dec_8bit(self.regs.d); 1 }
            0x1D => { self.regs.e = self.dec_8bit(self.regs.e); 1 }
            0x25 => { self.regs.h = self.dec_8bit(self.regs.h); 1 }
            0x2D => { self.regs.l = self.dec_8bit(self.regs.l); 1 }
            0x35 => {
                let addr = self.regs.get_hl();
                let value = self.dec_8bit(self.bus.read_byte(addr));
                self.bus.write_byte(addr, value);
                1
            }
            0x0B => { self.regs.set_af(CPU::dec_16bit(self.regs.get_bc())); 1 }
            0x1B => { self.regs.set_de(CPU::dec_16bit(self.regs.get_de())); 1 }
            0x2B => { self.regs.set_hl(CPU::dec_16bit(self.regs.get_hl())); 1 }
            0x3B => { self.sp = CPU::dec_16bit(self.sp); 1 }

            // SCF
            0x37 => { self.scf(); 1 }
            
            // RLA
            0x17 => { self.regs.a = self.rotate_left_circular(self.regs.a, false); 1 }
            
            // RLCA
            0x07 => { self.regs.a = self.rotate_left(self.regs.a, false); 1 }
            
            // JP
            0xC3 => { self.jump(true); 0 }
            0xC2 => { self.jump(self.regs.f.zero == false); 0 }
            0xD2 => { self.jump(self.regs.f.carry == false); 0 }
            0xCA => { self.jump(self.regs.f.zero == true); 0 }
            0xDA => { self.jump(self.regs.f.carry == true); 0 }
            0xE9 => { self.pc = self.regs.get_hl(); 0 }

            // JR
            0x18 => { self.jump_relative(true); 0 }
            0x20 => { self.jump_relative(self.regs.f.zero == false); 0 }
            0x30 => { self.jump_relative(self.regs.f.carry == false); 0 }
            0x28 => { self.jump_relative(self.regs.f.zero == true); 0 }
            0x38 => { self.jump_relative(self.regs.f.carry == true); 0 }

            // LD immediate to register
            0x06 => { self.regs.b = self.bus.read_byte(self.pc + 1); 2 }
            0x0E => { self.regs.c = self.bus.read_byte(self.pc + 1); 2 }
            0x16 => { self.regs.d = self.bus.read_byte(self.pc + 1); 2 }
            0x1E => { self.regs.e = self.bus.read_byte(self.pc + 1); 2 }
            0x2E => { self.regs.h = self.bus.read_byte(self.pc + 1); 2 }
            0x26 => { self.regs.l = self.bus.read_byte(self.pc + 1); 2 }
            0x3E => {
                let addr = self.regs.get_hl();
                let value = self.bus.read_byte(self.pc + 1);
                self.bus.write_byte(addr, value);
                2
            }
            0x36 => { self.regs.a = self.bus.read_byte(self.pc + 1); 2 }
            0x01 => { self.regs.set_bc(self.get_d16()); 3 }
            0x11 => { self.regs.set_de(self.get_d16()); 3 }
            0x21 => { self.regs.set_hl(self.get_d16()); 3 }
            0x31 => { self.sp = self.get_d16(); 3 }

            // LD A
            0x02 => { let addr = self.regs.get_bc(); self.bus.write_byte(addr, self.regs.a); 1 }
            0x12 => { let addr = self.regs.get_de(); self.bus.write_byte(addr, self.regs.a); 1 }
            0x22 => { let addr = self.regs.get_hl(); self.bus.write_byte(addr, self.regs.a); self.regs.set_hl(addr + 1); 1 }
            0x32 => { let addr = self.regs.get_hl(); self.bus.write_byte(addr, self.regs.a); self.regs.set_hl(addr - 1); 1 }

            0x0A => { self.regs.a = self.bus.read_byte(self.regs.get_bc()); 1 }
            0x1A => { self.regs.a = self.bus.read_byte(self.regs.get_de()); 1 }
            0x2A => { let addr = self.regs.get_hl(); self.regs.a = self.bus.read_byte(addr); self.regs.set_hl(addr + 1); 1 }
            0x3A => { let addr = self.regs.get_hl(); self.regs.a = self.bus.read_byte(addr); self.regs.set_hl(addr - 1); 1 }

            // LD SP
            0x08 => {
                let addr = self.get_d16();
                let lower_byte = (self.sp & 0xFF) as u8;
                let upper_byte = ((self.sp & 0xFF00) >> 2) as u8;
                self.bus.write_byte(addr, lower_byte);
                self.bus.write_byte(addr, upper_byte);
                3
            }
            
            // LD from A to a8 and vice versa
            0xE0 => { self.bus.write_byte(self.bus.read_byte(self.pc + 1) as u16 | 0xFF00, self.regs.a); 2 }
            0xF0 => { self.regs.a = self.bus.read_byte(self.pc + 1); 2 }
            
            // LD from A to C and vice versa
            0xE2 => { self.bus.write_byte(self.regs.c as u16 | 0xFF00, self.regs.a); 1 }
            0xF2 => { self.regs.a = self.bus.read_byte(self.regs.c as u16 | 0xFF00); 1 }
            
            // LD A to a16 and vice versa
            0xEA => { self.bus.write_byte(self.get_d16(), self.regs.a); 3 }
            0xFA => { self.regs.a = self.bus.read_byte(self.get_d16()); 3 }

            _ => panic!("Unknown instruction found for: 0x{:x}", byte),
        }
    }

    fn execute_prefixed(&mut self, byte: u8) -> u16 {
        match byte {
            // RES
            0x80 => { self.regs.b = self.reset_bit(self.regs.b, 0); 2 }
            0x81 => { self.regs.c = self.reset_bit(self.regs.c, 0); 2 }
            0x82 => { self.regs.d = self.reset_bit(self.regs.d, 0); 2 }
            0x83 => { self.regs.e = self.reset_bit(self.regs.e, 0); 2 }
            0x84 => { self.regs.h = self.reset_bit(self.regs.h, 0); 2 }
            0x85 => { self.regs.l = self.reset_bit(self.regs.l, 0); 2 }
            0x86 => {
                let addr = self.regs.get_hl();
                let value = self.reset_bit(self.bus.read_byte(addr), 0);
                self.bus.write_byte(addr, value);
                2
            }
            0x87 => { self.regs.a = self.reset_bit(self.regs.a, 0); 2 }

            0x88 => { self.regs.b = self.reset_bit(self.regs.b, 1); 2 }
            0x89 => { self.regs.c = self.reset_bit(self.regs.c, 1); 2 }
            0x8a => { self.regs.d = self.reset_bit(self.regs.d, 1); 2 }
            0x8b => { self.regs.e = self.reset_bit(self.regs.e, 1); 2 }
            0x8c => { self.regs.h = self.reset_bit(self.regs.h, 1); 2 }
            0x8d => { self.regs.l = self.reset_bit(self.regs.l, 1); 2 }
            0x8e => {
                let addr = self.regs.get_hl();
                let value = self.reset_bit(self.bus.read_byte(addr), 1);
                self.bus.write_byte(addr, value);
                2
            }
            0x8f => { self.regs.a = self.reset_bit(self.regs.a, 1); 2 }

            0x90 => { self.regs.b = self.reset_bit(self.regs.b, 2); 2 }
            0x91 => { self.regs.c = self.reset_bit(self.regs.c, 2); 2 }
            0x92 => { self.regs.d = self.reset_bit(self.regs.d, 2); 2 }
            0x93 => { self.regs.e = self.reset_bit(self.regs.e, 2); 2 }
            0x94 => { self.regs.h = self.reset_bit(self.regs.h, 2); 2 }
            0x95 => { self.regs.l = self.reset_bit(self.regs.l, 2); 2 }
            0x96 => {
                let addr = self.regs.get_hl();
                let value = self.reset_bit(self.bus.read_byte(addr), 2);
                self.bus.write_byte(addr, value);
                2
            }
            0x97 => { self.regs.a = self.reset_bit(self.regs.a, 2); 2 }

            0x98 => { self.regs.b = self.reset_bit(self.regs.b, 3); 2 }
            0x99 => { self.regs.c = self.reset_bit(self.regs.c, 3); 2 }
            0x9a => { self.regs.d = self.reset_bit(self.regs.d, 3); 2 }
            0x9b => { self.regs.e = self.reset_bit(self.regs.e, 3); 2 }
            0x9c => { self.regs.h = self.reset_bit(self.regs.h, 3); 2 }
            0x9d => { self.regs.l = self.reset_bit(self.regs.l, 3); 2 }
            0x9e => {
                let addr = self.regs.get_hl();
                let value = self.reset_bit(self.bus.read_byte(addr), 3);
                self.bus.write_byte(addr, value);
                2
            }
            0x9f => { self.regs.a = self.reset_bit(self.regs.a, 3); 2 }

            0xa0 => { self.regs.b = self.reset_bit(self.regs.b, 4); 2 }
            0xa1 => { self.regs.c = self.reset_bit(self.regs.c, 4); 2 }
            0xa2 => { self.regs.d = self.reset_bit(self.regs.d, 4); 2 }
            0xa3 => { self.regs.e = self.reset_bit(self.regs.e, 4); 2 }
            0xa4 => { self.regs.h = self.reset_bit(self.regs.h, 4); 2 }
            0xa5 => { self.regs.l = self.reset_bit(self.regs.l, 4); 2 }
            0xa6 => {
                let addr = self.regs.get_hl();
                let value = self.reset_bit(self.bus.read_byte(addr), 4);
                self.bus.write_byte(addr, value);
                2
            }
            0xa7 => { self.regs.a = self.reset_bit(self.regs.a, 4); 2 }

            0xa8 => { self.regs.b = self.reset_bit(self.regs.b, 5); 2 }
            0xa9 => { self.regs.c = self.reset_bit(self.regs.c, 5); 2 }
            0xaa => { self.regs.d = self.reset_bit(self.regs.d, 5); 2 }
            0xab => { self.regs.e = self.reset_bit(self.regs.e, 5); 2 }
            0xac => { self.regs.h = self.reset_bit(self.regs.h, 5); 2 }
            0xad => { self.regs.l = self.reset_bit(self.regs.l, 5); 2 }
            0xae => {
                let addr = self.regs.get_hl();
                let value = self.reset_bit(self.bus.read_byte(addr), 5);
                self.bus.write_byte(addr, value);
                2
            }
            0xaf => { self.regs.a = self.reset_bit(self.regs.a, 5); 2 }

            0xb0 => { self.regs.b = self.reset_bit(self.regs.b, 6); 2 }
            0xb1 => { self.regs.c = self.reset_bit(self.regs.c, 6); 2 }
            0xb2 => { self.regs.d = self.reset_bit(self.regs.d, 6); 2 }
            0xb3 => { self.regs.e = self.reset_bit(self.regs.e, 6); 2 }
            0xb4 => { self.regs.h = self.reset_bit(self.regs.h, 6); 2 }
            0xb5 => { self.regs.l = self.reset_bit(self.regs.l, 6); 2 }
            0xb6 => {
                let addr = self.regs.get_hl();
                let value = self.reset_bit(self.bus.read_byte(addr), 6);
                self.bus.write_byte(addr, value);
                2
            }
            0xb7 => { self.regs.a = self.reset_bit(self.regs.a, 6); 2 }

            0xb8 => { self.regs.b = self.reset_bit(self.regs.b, 7); 2 }
            0xb9 => { self.regs.c = self.reset_bit(self.regs.c, 7); 2 }
            0xba => { self.regs.d = self.reset_bit(self.regs.d, 7); 2 }
            0xbb => { self.regs.e = self.reset_bit(self.regs.e, 7); 2 }
            0xbc => { self.regs.h = self.reset_bit(self.regs.h, 7); 2 }
            0xbd => { self.regs.l = self.reset_bit(self.regs.l, 7); 2 }
            0xbe => {
                let addr = self.regs.get_hl();
                let value = self.reset_bit(self.bus.read_byte(addr), 7);
                self.bus.write_byte(addr, value);
                2
            }
            0xbf => { self.regs.a = self.reset_bit(self.regs.a, 7); 2 }

            // SRL
            0x38 => { self.regs.b = self.shift_right_logical(self.regs.b); 2 }
            0x39 => { self.regs.c = self.shift_right_logical(self.regs.c); 2 }
            0x3a => { self.regs.d = self.shift_right_logical(self.regs.d); 2 }
            0x3b => { self.regs.e = self.shift_right_logical(self.regs.e); 2 }
            0x3c => { self.regs.h = self.shift_right_logical(self.regs.h); 2 }
            0x3d => { self.regs.l = self.shift_right_logical(self.regs.l); 2 }
            0x3e => {
                let addr = self.regs.get_hl();
                let value = self.shift_right_logical(self.bus.read_byte(addr));
                self.bus.write_byte(addr, value);
                2
            }
            0x3f => { self.regs.a = self.shift_right_logical(self.regs.a); 2 }

            // RR
            0x18 => { self.regs.b = self.rotate_right_circular(self.regs.b, false); 2 }
            0x19 => { self.regs.c = self.rotate_right_circular(self.regs.c, false); 2 }
            0x1a => { self.regs.d = self.rotate_right_circular(self.regs.d, false); 2 }
            0x1b => { self.regs.e = self.rotate_right_circular(self.regs.e, false); 2 }
            0x1c => { self.regs.h = self.rotate_right_circular(self.regs.h, false); 2 }
            0x1d => { self.regs.l = self.rotate_right_circular(self.regs.l, false); 2 }
            0x1e => {
                let addr = self.regs.get_hl();
                let value = self.rotate_right_circular(self.bus.read_byte(addr), false);
                self.bus.write_byte(addr, value);
                2
            }
            0x1f => { self.regs.a = self.rotate_right_circular(self.regs.a, false); 2 }

            // RRC
            0x08 => { self.regs.b = self.rotate_right(self.regs.b, false); 2 }
            0x09 => { self.regs.c = self.rotate_right(self.regs.c, false); 2 }
            0x0a => { self.regs.d = self.rotate_right(self.regs.d, false); 2 }
            0x0b => { self.regs.e = self.rotate_right(self.regs.e, false); 2 }
            0x0c => { self.regs.h = self.rotate_right(self.regs.h, false); 2 }
            0x0d => { self.regs.l = self.rotate_right(self.regs.l, false); 2 }
            0x0e => {
                let addr = self.regs.get_hl();
                let value = self.rotate_right(self.bus.read_byte(addr), false);
                self.bus.write_byte(addr, value);
                2
            }
            0x0f => { self.regs.a = self.rotate_right(self.regs.a, false); 2 }

            // SRA
            0x28 => { self.regs.b = self.shift_right_arithmetic(self.regs.b); 2 }
            0x29 => { self.regs.c = self.shift_right_arithmetic(self.regs.c); 2 }
            0x2a => { self.regs.d = self.shift_right_arithmetic(self.regs.d); 2 }
            0x2b => { self.regs.e = self.shift_right_arithmetic(self.regs.e); 2 }
            0x2c => { self.regs.h = self.shift_right_arithmetic(self.regs.h); 2 }
            0x2d => { self.regs.l = self.shift_right_arithmetic(self.regs.l); 2 }
            0x2e => {
                let addr = self.regs.get_hl();
                let value = self.shift_right_arithmetic(self.bus.read_byte(addr));
                self.bus.write_byte(addr, value);
                2
            }
            0x2f => { self.regs.a = self.shift_right_arithmetic(self.regs.a); 2 }
            
            // BIT
            0x40 => { self.test_bit(self.regs.b,                            0); 2 }
            0x41 => { self.test_bit(self.regs.c,                            0); 2 }
            0x42 => { self.test_bit(self.regs.d,                            0); 2 }
            0x43 => { self.test_bit(self.regs.e,                            0); 2 }
            0x44 => { self.test_bit(self.regs.h,                            0); 2 }
            0x45 => { self.test_bit(self.regs.l,                            0); 2 }
            0x46 => { self.test_bit(self.bus.read_byte(self.regs.get_hl()), 0); 2 }
            0x47 => { self.test_bit(self.regs.a,                            0); 2 }
            0x48 => { self.test_bit(self.regs.b,                            1); 2 }
            0x49 => { self.test_bit(self.regs.c,                            1); 2 }
            0x4a => { self.test_bit(self.regs.d,                            1); 2 }
            0x4b => { self.test_bit(self.regs.e,                            1); 2 }
            0x4c => { self.test_bit(self.regs.h,                            1); 2 }
            0x4d => { self.test_bit(self.regs.l,                            1); 2 }
            0x4e => { self.test_bit(self.bus.read_byte(self.regs.get_hl()), 1); 2 }
            0x4f => { self.test_bit(self.regs.a,                            1); 2 }
            0x50 => { self.test_bit(self.regs.b,                            2); 2 }
            0x51 => { self.test_bit(self.regs.c,                            2); 2 }
            0x52 => { self.test_bit(self.regs.d,                            2); 2 }
            0x53 => { self.test_bit(self.regs.e,                            2); 2 }
            0x54 => { self.test_bit(self.regs.h,                            2); 2 }
            0x55 => { self.test_bit(self.regs.l,                            2); 2 }
            0x56 => { self.test_bit(self.bus.read_byte(self.regs.get_hl()), 2); 2 }
            0x57 => { self.test_bit(self.regs.a,                            2); 2 }
            0x58 => { self.test_bit(self.regs.b,                            3); 2 }
            0x59 => { self.test_bit(self.regs.c,                            3); 2 }
            0x5a => { self.test_bit(self.regs.d,                            3); 2 }
            0x5b => { self.test_bit(self.regs.e,                            3); 2 }
            0x5c => { self.test_bit(self.regs.h,                            3); 2 }
            0x5d => { self.test_bit(self.regs.l,                            3); 2 }
            0x5e => { self.test_bit(self.bus.read_byte(self.regs.get_hl()), 3); 2 }
            0x5f => { self.test_bit(self.regs.a,                            3); 2 }
            0x60 => { self.test_bit(self.regs.b,                            4); 2 }
            0x61 => { self.test_bit(self.regs.c,                            4); 2 }
            0x62 => { self.test_bit(self.regs.d,                            4); 2 }
            0x63 => { self.test_bit(self.regs.e,                            4); 2 }
            0x64 => { self.test_bit(self.regs.h,                            4); 2 }
            0x65 => { self.test_bit(self.regs.l,                            4); 2 }
            0x66 => { self.test_bit(self.bus.read_byte(self.regs.get_hl()), 4); 2 }
            0x67 => { self.test_bit(self.regs.a,                            4); 2 }
            0x68 => { self.test_bit(self.regs.b,                            5); 2 }
            0x69 => { self.test_bit(self.regs.c,                            5); 2 }
            0x6a => { self.test_bit(self.regs.d,                            5); 2 }
            0x6b => { self.test_bit(self.regs.e,                            5); 2 }
            0x6c => { self.test_bit(self.regs.h,                            5); 2 }
            0x6d => { self.test_bit(self.regs.l,                            5); 2 }
            0x6e => { self.test_bit(self.bus.read_byte(self.regs.get_hl()), 5); 2 }
            0x6f => { self.test_bit(self.regs.a,                            5); 2 }
            0x70 => { self.test_bit(self.regs.b,                            6); 2 }
            0x71 => { self.test_bit(self.regs.c,                            6); 2 }
            0x72 => { self.test_bit(self.regs.d,                            6); 2 }
            0x73 => { self.test_bit(self.regs.e,                            6); 2 }
            0x74 => { self.test_bit(self.regs.h,                            6); 2 }
            0x75 => { self.test_bit(self.regs.l,                            6); 2 }
            0x76 => { self.test_bit(self.bus.read_byte(self.regs.get_hl()), 6); 2 }
            0x77 => { self.test_bit(self.regs.a,                            6); 2 }
            0x78 => { self.test_bit(self.regs.b,                            7); 2 }
            0x79 => { self.test_bit(self.regs.c,                            7); 2 }
            0x7a => { self.test_bit(self.regs.d,                            7); 2 }
            0x7b => { self.test_bit(self.regs.e,                            7); 2 }
            0x7c => { self.test_bit(self.regs.h,                            7); 2 }
            0x7d => { self.test_bit(self.regs.l,                            7); 2 }
            0x7e => { self.test_bit(self.bus.read_byte(self.regs.get_hl()), 7); 2 }
            0x7f => { self.test_bit(self.regs.a,                            7); 2 }
            
            // SET
            0xC0 => { self.regs.b = CPU::set_bit(self.regs.b,                                                             0); 2 }
            0xC1 => { self.regs.c = CPU::set_bit(self.regs.c,                                                             0); 2 }
            0xC2 => { self.regs.d = CPU::set_bit(self.regs.d,                                                             0); 2 }
            0xC3 => { self.regs.e = CPU::set_bit(self.regs.e,                                                             0); 2 }
            0xC4 => { self.regs.h = CPU::set_bit(self.regs.h,                                                             0); 2 }
            0xC5 => { self.regs.l = CPU::set_bit(self.regs.l,                                                             0); 2 }
            0xC6 => { let addr = self.regs.get_hl(); self.bus.write_byte(addr, CPU::set_bit(self.bus.read_byte(addr), 0)); 2 }
            0xC7 => { self.regs.a = CPU::set_bit(self.regs.a,                                                             0); 2 }
            0xC8 => { self.regs.b = CPU::set_bit(self.regs.b,                                                             1); 2 }
            0xC9 => { self.regs.c = CPU::set_bit(self.regs.c,                                                             1); 2 }
            0xCA => { self.regs.d = CPU::set_bit(self.regs.d,                                                             1); 2 }
            0xCB => { self.regs.e = CPU::set_bit(self.regs.e,                                                             1); 2 }
            0xCC => { self.regs.h = CPU::set_bit(self.regs.h,                                                             1); 2 }
            0xCD => { self.regs.l = CPU::set_bit(self.regs.l,                                                             1); 2 }
            0xCE => { let addr = self.regs.get_hl(); self.bus.write_byte(addr, CPU::set_bit(self.bus.read_byte(addr), 1)); 2 }
            0xCF => { self.regs.a = CPU::set_bit(self.regs.a,                                                             1); 2 }
            0xD0 => { self.regs.b = CPU::set_bit(self.regs.b,                                                             2); 2 }
            0xD1 => { self.regs.c = CPU::set_bit(self.regs.c,                                                             2); 2 }
            0xD2 => { self.regs.d = CPU::set_bit(self.regs.d,                                                             2); 2 }
            0xD3 => { self.regs.e = CPU::set_bit(self.regs.e,                                                             2); 2 }
            0xD4 => { self.regs.h = CPU::set_bit(self.regs.h,                                                             2); 2 }
            0xD5 => { self.regs.l = CPU::set_bit(self.regs.l,                                                             2); 2 }
            0xD6 => { let addr = self.regs.get_hl(); self.bus.write_byte(addr, CPU::set_bit(self.bus.read_byte(addr), 2)); 2 }
            0xD7 => { self.regs.a = CPU::set_bit(self.regs.a,                                                             2); 2 }
            0xD8 => { self.regs.b = CPU::set_bit(self.regs.b,                                                             3); 2 }
            0xD9 => { self.regs.c = CPU::set_bit(self.regs.c,                                                             3); 2 }
            0xDA => { self.regs.d = CPU::set_bit(self.regs.d,                                                             3); 2 }
            0xDB => { self.regs.e = CPU::set_bit(self.regs.e,                                                             3); 2 }
            0xDC => { self.regs.h = CPU::set_bit(self.regs.h,                                                             3); 2 }
            0xDD => { self.regs.l = CPU::set_bit(self.regs.l,                                                             3); 2 }
            0xDE => { let addr = self.regs.get_hl(); self.bus.write_byte(addr, CPU::set_bit(self.bus.read_byte(addr), 3)); 2 }
            0xDF => { self.regs.a = CPU::set_bit(self.regs.a,                                                             3); 2 }
            0xE0 => { self.regs.b = CPU::set_bit(self.regs.b,                                                             4); 2 }
            0xE1 => { self.regs.c = CPU::set_bit(self.regs.c,                                                             4); 2 }
            0xE2 => { self.regs.d = CPU::set_bit(self.regs.d,                                                             4); 2 }
            0xE3 => { self.regs.e = CPU::set_bit(self.regs.e,                                                             4); 2 }
            0xE4 => { self.regs.h = CPU::set_bit(self.regs.h,                                                             4); 2 }
            0xE5 => { self.regs.l = CPU::set_bit(self.regs.l,                                                             4); 2 }
            0xE6 => { let addr = self.regs.get_hl(); self.bus.write_byte(addr, CPU::set_bit(self.bus.read_byte(addr), 4)); 2 }
            0xE7 => { self.regs.a = CPU::set_bit(self.regs.a,                                                             4); 2 }
            0xE8 => { self.regs.b = CPU::set_bit(self.regs.b,                                                             5); 2 }
            0xE9 => { self.regs.c = CPU::set_bit(self.regs.c,                                                             5); 2 }
            0xEA => { self.regs.d = CPU::set_bit(self.regs.d,                                                             5); 2 }
            0xEB => { self.regs.e = CPU::set_bit(self.regs.e,                                                             5); 2 }
            0xEC => { self.regs.h = CPU::set_bit(self.regs.h,                                                             5); 2 }
            0xED => { self.regs.l = CPU::set_bit(self.regs.l,                                                             5); 2 }
            0xEE => { let addr = self.regs.get_hl(); self.bus.write_byte(addr, CPU::set_bit(self.bus.read_byte(addr), 5)); 2 }
            0xEF => { self.regs.a = CPU::set_bit(self.regs.a,                                                             5); 2 }
            0xF0 => { self.regs.b = CPU::set_bit(self.regs.b,                                                             6); 2 }
            0xF1 => { self.regs.c = CPU::set_bit(self.regs.c,                                                             6); 2 }
            0xF2 => { self.regs.d = CPU::set_bit(self.regs.d,                                                             6); 2 }
            0xF3 => { self.regs.e = CPU::set_bit(self.regs.e,                                                             6); 2 }
            0xF4 => { self.regs.h = CPU::set_bit(self.regs.h,                                                             6); 2 }
            0xF5 => { self.regs.l = CPU::set_bit(self.regs.l,                                                             6); 2 }
            0xF6 => { let addr = self.regs.get_hl(); self.bus.write_byte(addr, CPU::set_bit(self.bus.read_byte(addr), 6)); 2 }
            0xF7 => { self.regs.a = CPU::set_bit(self.regs.a,                                                             6); 2 }
            0xF8 => { self.regs.b = CPU::set_bit(self.regs.b,                                                             7); 2 }
            0xF9 => { self.regs.c = CPU::set_bit(self.regs.c,                                                             7); 2 }
            0xFA => { self.regs.d = CPU::set_bit(self.regs.d,                                                             7); 2 }
            0xFB => { self.regs.e = CPU::set_bit(self.regs.e,                                                             7); 2 }
            0xFC => { self.regs.h = CPU::set_bit(self.regs.h,                                                             7); 2 }
            0xFD => { self.regs.l = CPU::set_bit(self.regs.l,                                                             7); 2 }
            0xFE => { let addr = self.regs.get_hl(); self.bus.write_byte(addr, CPU::set_bit(self.bus.read_byte(addr), 7)); 2 }
            0xFF => { self.regs.a = CPU::set_bit(self.regs.a,                                                             7); 2 }
            
            // RL
            0x10 => { self.regs.b = self.rotate_left(self.regs.b, false); 2 }
            0x11 => { self.regs.c = self.rotate_left(self.regs.c, false); 2 }
            0x12 => { self.regs.d = self.rotate_left(self.regs.d, false); 2 }
            0x13 => { self.regs.e = self.rotate_left(self.regs.e, false); 2 }
            0x14 => { self.regs.h = self.rotate_left(self.regs.h, false); 2 }
            0x15 => { self.regs.l = self.rotate_left(self.regs.l, false); 2 }
            0x16 => {
                let addr = self.regs.get_hl();
                let value = self.rotate_left(self.bus.read_byte(addr), true);
                self.bus.write_byte(addr, value);
                2
            }
            0x17 => { self.regs.a = self.rotate_left(self.regs.a, false); 2 }
            
            // RLC
            0x00 => { self.regs.b = self.rotate_left(self.regs.b, false); 2 }
            0x01 => { self.regs.c = self.rotate_left(self.regs.c, false); 2 }
            0x02 => { self.regs.d = self.rotate_left(self.regs.d, false); 2 }
            0x03 => { self.regs.e = self.rotate_left(self.regs.e, false); 2 }
            0x04 => { self.regs.h = self.rotate_left(self.regs.h, false); 2 }
            0x05 => { self.regs.l = self.rotate_left(self.regs.l, false); 2 }
            0x06 => {
                let addr = self.regs.get_hl();
                let value = self.rotate_left(self.bus.read_byte(addr), true);
                self.bus.write_byte(addr, value);
                2
            }
            0x07 => { self.regs.a = self.rotate_left(self.regs.a, false); 2 }
            
            // SLA
            0x20 => { self.regs.b = self.shift_left_arithmetic(self.regs.b); 2 }
            0x21 => { self.regs.c = self.shift_left_arithmetic(self.regs.c); 2 }
            0x22 => { self.regs.d = self.shift_left_arithmetic(self.regs.d); 2 }
            0x23 => { self.regs.e = self.shift_left_arithmetic(self.regs.e); 2 }
            0x24 => { self.regs.h = self.shift_left_arithmetic(self.regs.h); 2 }
            0x25 => { self.regs.l = self.shift_left_arithmetic(self.regs.l); 2 }
            0x26 => {
                let addr = self.regs.get_hl();
                let value = self.shift_left_arithmetic(self.bus.read_byte(addr));
                self.bus.write_byte(addr, value);
                2
            }
            0x27 => { self.regs.a = self.shift_left_arithmetic(self.regs.a); 2 }
            
            // SWAP
            0x30 => { self.regs.b = self.swap(self.regs.b); 2 }
            0x31 => { self.regs.c = self.swap(self.regs.c); 2 }
            0x32 => { self.regs.d = self.swap(self.regs.d); 2 }
            0x33 => { self.regs.e = self.swap(self.regs.e); 2 }
            0x34 => { self.regs.h = self.swap(self.regs.h); 2 }
            0x35 => { self.regs.l = self.swap(self.regs.l); 2 }
            0x36 => {
                let addr = self.regs.get_hl();
                let value = self.swap(self.bus.read_byte(addr));
                self.bus.write_byte(addr, value);
                2
            }
            0x37 => { self.regs.a = self.swap(self.regs.a); 2 }
        }
    }
    
    fn add(&mut self, value: u8, with_carry: bool) {
        let add_carry = if with_carry && self.regs.f.carry {
            1
        } else {
            0
        };
        let (new_value, did_overflow) = self.regs.a.overflowing_add(value);
        let (new_value2, did_overflow2) = new_value.overflowing_add(add_carry);
        self.regs.f.zero = new_value2 == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = (self.regs.a & 0xF) + (value & 0xF) + add_carry > 0xF;
        self.regs.f.carry = did_overflow || did_overflow2;
        self.regs.a = new_value2;
    }

    fn add_hl(&mut self, value: u16) {
        let (new_value, did_overflow) = self.regs.get_hl().overflowing_add(value);
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = (self.regs.get_hl() & 0b111_1111_1111)
            + (value & 0b111_1111_1111)
            > 0b111_1111_1111;
        self.regs.f.carry = did_overflow;
        self.regs.set_hl(new_value);
    }

    fn and(&mut self, value: u8) {
        let new_value = self.regs.a & value;
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = true;
        self.regs.f.carry = false;
        self.regs.a = new_value;
    }

    fn cp(&mut self, value: u8) {
        self.regs.f.zero = self.regs.a == value;
        self.regs.f.subtract = true;
        self.regs.f.half_carry = (self.regs.a & 0xF) < (value & 0xF);
        self.regs.f.carry = self.regs.a < value;
    }

    fn inc_8bit(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.regs.f.zero = value == new_value;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = value & 0xF == 0xF;
        self.regs.f.carry = false;
        new_value
    }

    fn inc_16bit(value: u16) -> u16 {
        value.wrapping_add(1)
    }

    fn rotate_right_circular(&mut self, value: u8, set_zero: bool) -> u8 {
        let new_value = value.rotate_right(1);
        self.regs.f.zero = set_zero && new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = false;
        self.regs.f.carry = value & 0b1 == 0b1;
        new_value
    }

    fn rotate_right(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = (self.regs.f.carry as u8) << 7;
        let new_value = carry_bit | (value >> 1);
        self.regs.f.zero = set_zero && new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = false;
        self.regs.f.carry = value & 0b1 == 0b1;
        new_value
    }

    fn reset_bit(&mut self, value: u8, bit_position: u8) -> u8 {
        value & !(1 << bit_position)
    }

    fn shift_right_logical(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = false;
        self.regs.f.carry = value & 0b1 == 0b1;
        new_value
    }

    fn shift_right_arithmetic(&mut self, value: u8) -> u8 {
        let new_value = (value >> 1) | (value & 0x80);
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = false;
        self.regs.f.carry = value & 0b1 == 0b1;
        new_value
    }
    
    fn sub(&mut self, value: u8) {
        let (new_value, did_overflow) = self.regs.a.overflowing_sub(value);
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = true;
        self.regs.f.carry = did_overflow;
        self.regs.f.half_carry = (self.regs.a & 0xF) + (value & 0xF) > 0xF;
        self.regs.a = new_value;
    }
    
    fn sbc(&mut self, value: u8) {
        let (new_value, did_overflow) = self.regs.a.overflowing_sub(value);
        let (new_value2, did_overflow2) = if self.regs.f.carry {
            new_value.overflowing_sub(1)
        } else {
            (new_value, did_overflow)
        };
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = true;
        self.regs.f.carry = did_overflow || did_overflow2;
        self.regs.f.half_carry = (self.regs.a & 0xF) + (value & 0xF) + 1 > 0xF;
        self.regs.a = new_value2;
    }

    fn or(&mut self, value: u8) {
        let new_value = self.regs.a | value;
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.carry = false;
        self.regs.f.half_carry = false;
        self.regs.a = new_value;
    }
    
    fn xor(&mut self, value: u8) {
        let new_value = self.regs.a ^ value;
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.carry = false;
        self.regs.f.half_carry = false;
        self.regs.a = new_value;
    }
    
    fn dec_8bit(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.regs.f.zero = value == new_value;
        self.regs.f.subtract = false;
        self.regs.f.carry = false;
        self.regs.f.half_carry = value & 0xF == 0xF;
        new_value
    }

    fn dec_16bit(value: u16) -> u16 {
        value.wrapping_sub(1)
    }

    fn scf(&mut self) {
        self.regs.f.subtract = false;
        self.regs.f.half_carry = false;
        self.regs.f.carry = true;
    }

    fn rotate_left_circular(&mut self, value: u8, set_zero: bool) -> u8 {
        let new_value = value.rotate_left(1);
        self.regs.f.zero = set_zero && new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = false;
        self.regs.f.carry = value & 0x80 == 0x80;
        new_value
    }
    fn rotate_left(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = if self.regs.f.carry { 1 } else { 0 };
        let new_value = carry_bit | (value << 1);
        self.regs.f.zero = set_zero && new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = false;
        self.regs.f.carry = value & 0x80 == 0x80;
        new_value
    }
    
    fn test_bit(&mut self, value: u8, bit_pos: u8) {
        let new_value = (value >> bit_pos) & 0b1;
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = false;
    }
    
    fn set_bit(value: u8, bit_pos: u8) -> u8 {
        value | (1 << bit_pos)
    }
    
    fn shift_left_arithmetic(&mut self, value: u8) -> u8 {
        let new_value = value << 1;
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = false;
        self.regs.f.carry = value & 0x80 == 0x80;
        new_value
    }
    
    fn swap(&mut self, value: u8) -> u8 {
        let new_value = value >> 4 | value << 4;
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = false;
        self.regs.f.carry = false;
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
