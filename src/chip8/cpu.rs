use super::Chip8;
use super::sprites::Chip8Sprite;
use rand::prelude::*;

// "targets" of a CPU instruction. can be read from for an operation or written to
#[derive(Debug, Clone, Copy)]
pub enum CPUInstrTarget {
    IRegister,
    VRegister(usize),
    MemoryAddress(usize),
    Constant(u16),
    IsKeyInVRegPressed(usize),
    CurrentKeyPressed,
    CurrentDelayTimer,
    CurrentSoundTimer,
    SpriteAddress(usize),
    RandomNum(u8),
    True,
}

// all possible ALU operations
#[derive(Debug, Clone, Copy)]
pub enum ALUOperations {
    Assign,
    Add { update_vf: bool },
    Subtract { update_vf: bool },
    SubtractFlipped { update_vf: bool },
    Or,
    And,
    Xor,
    ShiftRight { update_vf: bool },
    ShiftLeft { update_vf: bool },
    Unknown,
}

// all possible CPU instructions
#[derive(Debug)]
pub enum CPUInstruction {
    CallMachineCode { addr: CPUInstrTarget, },
    ClearDisplay,
    Return,
    Jump { addr: CPUInstrTarget, },
    CallSubroutine { addr: CPUInstrTarget, },
    CompareEq{ eq: bool, left: CPUInstrTarget, right: CPUInstrTarget, },
    Assignment { to: CPUInstrTarget, from: CPUInstrTarget, },
    ALUOperation { op: ALUOperations, left: CPUInstrTarget, right: CPUInstrTarget },
    SpecialJump { offset: CPUInstrTarget },
    Draw { x_reg: CPUInstrTarget, y_reg: CPUInstrTarget, height_px: CPUInstrTarget, },
    BCD { x_reg: CPUInstrTarget },
    RegisterDump { x: CPUInstrTarget },
    RegisterLoad { x: CPUInstrTarget },
    Unknown { opcode: u16 },
}

impl super::Chip8 {
    // converts a numerical opcode into a CPUInstruction
    pub fn opcode_to_instruction(opcode: u16) -> CPUInstruction {
        let instruction_type = (opcode & 0xF000) >> 12;
        let instruction_operands = opcode & 0x0FFF;
        let instruction_operands_list = [
            (instruction_operands & 0xF00) >> 8,
            (instruction_operands & 0x0F0) >> 4,
            (instruction_operands & 0x00F),
        ];

        match instruction_type {
            // 0xNNN: calls machine code at address 0xNNN (implemented: 0x0E0 = display clear, 0x0EE = return from subroutine)
            0x0 => {
                match instruction_operands {
                    // 0x00E0 = clear display
                    0x0E0 => CPUInstruction::ClearDisplay,

                    // 0x0EE = return from subroutine
                    0x0EE => CPUInstruction::Return,

                    // 0xNNN = call machine code at NNN
                    _ => CPUInstruction::CallMachineCode {
                        addr: CPUInstrTarget::Constant(instruction_operands),
                    }
                }
            },

            // 0x1NNN: jumps to address 0xNNN
            0x1 => CPUInstruction::Jump {
                addr: CPUInstrTarget::Constant(instruction_operands),
            },

            // 0x2NNN: calls subroutine at address 0xNNN
            0x2 => CPUInstruction::CallSubroutine {
                addr: CPUInstrTarget::Constant(instruction_operands),
            },

            // 0x(3/4)NNN: checks if Vx (=/!)= NN
            0x3 | 0x4 => CPUInstruction::CompareEq {
                eq: (instruction_type == 0x3),
                left: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                right: CPUInstrTarget::Constant(instruction_operands & 0xFF),
            },

            // 0x(5/9)XY0: checks if Vx (=/!)= Vy
            0x5 | 0x9 => CPUInstruction::CompareEq {
                eq: (instruction_type == 0x5),
                left: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                right: CPUInstrTarget::VRegister(instruction_operands_list[1] as usize),
            },

            // 0x6XNN: sets Vx to NN
            0x6 => CPUInstruction::Assignment {
                to: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                from: CPUInstrTarget::Constant(instruction_operands & 0xFF),
            },

            // 0x7XNN: increases Vx by NN
            0x7 => CPUInstruction::ALUOperation {
                op: ALUOperations::Add { update_vf: false },
                left: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                right: CPUInstrTarget::Constant(instruction_operands & 0xFF),
            },

            // 0x8XYN: performs various ALU operations on Vx and Vy
            0x8 => {
                let operation = match instruction_operands_list[2] {
                    0x0 => ALUOperations::Assign,
                    0x1 => ALUOperations::Or,
                    0x2 => ALUOperations::And,
                    0x3 => ALUOperations::Xor,
                    0x4 => ALUOperations::Add { update_vf: true },
                    0x5 => ALUOperations::Subtract { update_vf: true },
                    0x6 => ALUOperations::ShiftRight { update_vf: true },
                    0x7 => ALUOperations::SubtractFlipped { update_vf: true },
                    0xE => ALUOperations::ShiftLeft { update_vf: true },
                    _ => ALUOperations::Unknown,
                };
                
                CPUInstruction::ALUOperation {
                    op: operation,
                    left: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                    right: CPUInstrTarget::VRegister(instruction_operands_list[1] as usize),
                }
            }

            // 0xANNN: sets I register to NNN
            0xA => CPUInstruction::Assignment {
                to: CPUInstrTarget::IRegister,
                from: CPUInstrTarget::Constant(instruction_operands),
            },

            // 0xBNNN: sets the PC to V0 + NNN, i.e. "special jump"
            0xB => CPUInstruction::SpecialJump {
                offset: CPUInstrTarget::Constant(instruction_operands),
            },

            // 0xCXNN: sets Vx to a random number with bit mask NN
            0xC => CPUInstruction::Assignment {
                to: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                from: CPUInstrTarget::RandomNum((instruction_operands & 0xFF) as u8),
            },

            // 0xDXYN: draws sprite at (Vx, Vy) with a height of N pixels
            0xD => CPUInstruction::Draw {
                x_reg: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                y_reg: CPUInstrTarget::VRegister(instruction_operands_list[1] as usize),
                height_px: CPUInstrTarget::Constant(instruction_operands_list[2]),
            },

            // 0xEXNN: if NN = 9E, skips next instr if key pressed == Vx, if NN = A1, skips next instr if key pressed != Vx
            0xE => CPUInstruction::CompareEq {
                eq: (instruction_operands & 0xFF) == 0x9E,
                left: CPUInstrTarget::IsKeyInVRegPressed(instruction_operands_list[0] as usize),
                right: CPUInstrTarget::True,
            },

            // 0xFXNN: various
            0xF => {
                match instruction_operands & 0xFF {
                    // 0xFX(07/0A): sets Vx to the current (delay timer/key pressed)
                    0x07 | 0x0A => CPUInstruction::Assignment {
                        to: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                        from: if instruction_operands & 0xFF == 0x07 {
                            CPUInstrTarget::CurrentDelayTimer
                        } else {
                            CPUInstrTarget::CurrentKeyPressed
                        },
                    },

                    // 0xFX(15/18): sets the current (delay/sound) timer to Vx
                    0x15|0x18 => CPUInstruction::Assignment {
                        to: if instruction_operands & 0xFF == 0x15 {
                            CPUInstrTarget::CurrentDelayTimer
                        } else {
                            CPUInstrTarget::CurrentSoundTimer
                        },
                        from: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                    },

                    // 0xFX1E: increases I register by Vx
                    0x1E => CPUInstruction::ALUOperation {
                        op: ALUOperations::Add { update_vf: false },
                        left: CPUInstrTarget::IRegister,
                        right: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                    },

                    // 0xFX29: sets I register to sprite address for val in Vx
                    0x29 => CPUInstruction::Assignment {
                        to: CPUInstrTarget::IRegister,
                        from: CPUInstrTarget::SpriteAddress(instruction_operands_list[0] as usize),
                    },

                    // 0xFX33: BCD Vx into I..I+2
                    0x33 => CPUInstruction::BCD {
                        x_reg: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                    },

                    // 0xFX55: register dump V0..Vx to I..I+x
                    0x55 => CPUInstruction::RegisterDump {
                        x: CPUInstrTarget::Constant(instruction_operands_list[0]),
                    },

                    // 0xFX65: register load V0..Vx from I..I+x
                    0x65 => CPUInstruction::RegisterLoad {
                        x: CPUInstrTarget::Constant(instruction_operands_list[0]),
                    },

                    _ => CPUInstruction::Unknown { opcode }
                }
            }

            // unknown instruction
            _ => CPUInstruction::Unknown { opcode },
        }
    }

    // evaluates a CPUInstrTarget immutably, but still requires mutable CPU instance
    pub fn evaluate_cpu_instr_target(&mut self, target: &CPUInstrTarget) -> usize {
        match target {
            CPUInstrTarget::IRegister => self.registers.get_i_register().clone() as usize,
            CPUInstrTarget::VRegister(reg) => self.registers.get_v_register(*reg).clone() as usize,
            CPUInstrTarget::MemoryAddress(addr) => self.memory.get_memory_at(*addr).clone() as usize,
            CPUInstrTarget::Constant(val) => *val as usize,
            CPUInstrTarget::CurrentKeyPressed => {
                let key_opt = self.input.get_current_key();

                if let Some(key) = key_opt {
                    key
                } else {
                    *self.registers.get_pc_register_mut() -= 2;
                    0
                }
            },
            CPUInstrTarget::IsKeyInVRegPressed(reg) => if self.input.get_keys_status()[self.registers.get_v_register(*reg).clone() as usize] { 1 } else { 0 },
            CPUInstrTarget::CurrentDelayTimer => self.timers.get_delay().clone() as usize,
            CPUInstrTarget::CurrentSoundTimer => self.timers.get_sound().clone() as usize,
            CPUInstrTarget::SpriteAddress(sprite) => (self.registers.get_v_register(*sprite).clone() as usize * 5),
            CPUInstrTarget::RandomNum(mask) => (rand::random::<u8>() & mask) as usize,
            CPUInstrTarget::True => 1,
        }
    }

    // evaluates a CPUInstrTarget mutably (not all types are valid)
    pub fn set_cpu_instr_target(&mut self, target: CPUInstrTarget, val: usize) {
        match target {
            CPUInstrTarget::IRegister => *self.registers.get_i_register_mut() = (val as u16),
            CPUInstrTarget::VRegister(reg) => *self.registers.get_v_register_mut(reg) = (val as u8),
            CPUInstrTarget::MemoryAddress(addr) => *self.memory.get_memory_at_mut(addr) = (val as u8),
            CPUInstrTarget::CurrentDelayTimer => *self.timers.get_delay_mut() = (val as u8),
            CPUInstrTarget::CurrentSoundTimer => *self.timers.get_sound_mut() = (val as u8),

            _ => panic!("Attempted to assign {val} to an immutable value: {target:?}"),
        }
    }

    // executes a CPUInstruction
    fn execute_instruction(&mut self, instr: CPUInstruction) {
        match instr {
            // calls machine code at given address
            CPUInstruction::CallMachineCode { addr } => panic!("Machine code operations are not supported. Panicked at: {instr:?}"),

            // clear display
            CPUInstruction::ClearDisplay => self.output.clear_display(),

            // returns from subroutine by popping return address from stack and jumping there
            CPUInstruction::Return => {
                let return_address = self.memory.pop_from_stack_u16();
                self.registers.jump_to(return_address);
            },

            // jumps (sets PC) to given address
            CPUInstruction::Jump { addr } => {
                let addr = self.evaluate_cpu_instr_target(&addr);
                self.registers.jump_to(addr as u16);
            }

            // calls subroutine at given address by pushing current PC to stack and jumping to subroutine addr
            CPUInstruction::CallSubroutine { addr } => {
                let current_pc = self.registers.get_pc_register().clone();
                self.memory.push_to_stack_u16(current_pc);

                let jump_addr = self.evaluate_cpu_instr_target(&addr);
                self.registers.jump_to(jump_addr as u16);
            },

            // compares the two given arguments, if eq is true, skip the next instruction if they are equal (do nothing if not). if eq is false, skip the next instruction if they are not equal (do nothing if equal)
            CPUInstruction::CompareEq { eq, left, right } => {
                let left_val = self.evaluate_cpu_instr_target(&left);
                let right_val = self.evaluate_cpu_instr_target(&right);

                let skip = if eq { left_val == right_val } else { left_val != right_val };

                if skip {
                    self.registers.skip_next_instr();
                }
            },

            // assigns a given value to a given target
            CPUInstruction::Assignment { to, from } => {
                let from_val = self.evaluate_cpu_instr_target(&from);
                self.set_cpu_instr_target(to, from_val);
            }

            // performs an ALU operation
            CPUInstruction::ALUOperation { op, left, right } => {
                let left_val = self.evaluate_cpu_instr_target(&left) as isize;
                let right_val = self.evaluate_cpu_instr_target(&right) as isize;

                let (mut result, update_vf, new_vf) = match op {
                    ALUOperations::Assign => (right_val, false, false),
                    ALUOperations::Add { update_vf } => (left_val + right_val, update_vf, (left_val+right_val) > 0xFF),
                    ALUOperations::Subtract { update_vf } => (left_val - right_val, update_vf, left_val >= right_val),
                    ALUOperations::SubtractFlipped { update_vf } => (right_val - left_val, update_vf, right_val >= left_val),
                    ALUOperations::Or => (left_val | right_val, false, false),
                    ALUOperations::And => (left_val & right_val, false, false),
                    ALUOperations::Xor => (left_val ^ right_val, false, false),
                    ALUOperations::ShiftRight { update_vf } => (left_val >> 1, update_vf, (left_val & 1) == 1),
                    ALUOperations::ShiftLeft { update_vf } => (left_val << 1, update_vf, (left_val & 0x80) > 0),
                    ALUOperations::Unknown => panic!("Unknown ALU operation, panicked at instruction: {instr:?}"),
                };

                while result < 0 {
                    result += 256;
                }

                self.set_cpu_instr_target(left, result as usize);

                if update_vf {
                    *self.registers.get_v_register_mut(0xF) = if new_vf { 1 } else { 0 };
                }
            },

            // performs a "special jump": sets PC to V0 + a given number
            CPUInstruction::SpecialJump { offset } => {
                let v0_val = self.registers.get_v_register(0).clone() as usize;
                let offset_val = self.evaluate_cpu_instr_target(&offset);

                *self.registers.get_pc_register_mut() = (v0_val + offset_val) as u16;
            }

            // draws a sprite to the screen at (Vx, Vy) with a height of N pixels. VF gets set to 1 if any pixels were flipped from white to black, and 0 otherwise
            CPUInstruction::Draw { x_reg, y_reg, height_px } => {
                // evalulate instruction targets
                let x_reg_val = self.evaluate_cpu_instr_target(&x_reg);
                let y_reg_val = self.evaluate_cpu_instr_target(&y_reg);
                let height_px_val = self.evaluate_cpu_instr_target(&height_px);

                // create sprite instance
                let i_reg_val = self.registers.get_i_register().clone() as usize;
                let sprite = Chip8Sprite::new(&self.memory, i_reg_val, height_px_val);

                // draw sprite
                let new_vf_value = self.output.draw_sprite_on_display(x_reg_val, y_reg_val, sprite);

                // update VF
                *self.registers.get_v_register_mut(0xF) = if new_vf_value { 1 } else { 0 };
            },

            // decomposes Vx into BCD at addresses I..I+2
            CPUInstruction::BCD { x_reg } => {
                // extract digits
                let mut x_reg_val = self.evaluate_cpu_instr_target(&x_reg);
                let mut digits = [0; 3];
                
                for i in 0..3 {
                    digits[i] = x_reg_val % 10;
                    x_reg_val /= 10;
                }

                // write to memory
                let i_reg_val = self.registers.get_i_register().clone() as usize;
                *self.memory.get_memory_at_mut(i_reg_val) = digits[2] as u8;
                *self.memory.get_memory_at_mut(i_reg_val + 1) = digits[1] as u8;
                *self.memory.get_memory_at_mut(i_reg_val + 2) = digits[0] as u8;
            },

            // dumps V0..Vx starting at address I
            CPUInstruction::RegisterDump { x } => {
                let x_val = self.evaluate_cpu_instr_target(&x);

                // get starting address (I register)
                let i_reg_val = self.registers.get_i_register().clone() as usize;

                // dump registers to memory
                for i in 0..=x_val {
                    let vi_val = self.registers.get_v_register(i).clone();
                    *self.memory.get_memory_at_mut(i + i_reg_val) = vi_val;
                }
            },

            // loads V0..Vx starting at address I
            CPUInstruction::RegisterLoad { x } => {
                let x_val = self.evaluate_cpu_instr_target(&x);

                // get starting address (I register)
                let i_reg_val = self.registers.get_i_register().clone() as usize;

                // load registers from memory
                for i in 0..=x_val {
                    let mem_val = self.memory.get_memory_at(i_reg_val + i).clone();
                    *self.registers.get_v_register_mut(i) = mem_val;
                }
            },

            // unknown instruction
            CPUInstruction::Unknown { opcode } => panic!("Unknown instruction: {opcode:4X}."),
        }
    }

    // executes the next instruction (instruction at PC)
    pub fn execute_next_instruction(&mut self) {
        // first, read the instruction opcode at PC and convert it into a CPUInstruction
        let pc = self.registers.get_pc_register().clone() as usize;
        let opcode = self.memory.get_memory_at_u16(pc);
        let instruction = Chip8::opcode_to_instruction(opcode);


        // println!("{opcode:4X}: {instruction:?}");


        // increase PC by 2 before executing next instruction as to not interfere with jumps
        *self.registers.get_pc_register_mut() += 2;

        // now, execute that instruction
        self.execute_instruction(instruction);
    }
}