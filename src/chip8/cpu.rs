// "targets" of a CPU instruction. can be read from for an operation or written to
#[derive(Debug)]
pub enum CPUInstrTarget {
    IRegister,
    VRegister(usize),
    MemoryAddress(usize),
    Constant(u16),
    CurrentKeyPressed,
    CurrentDelayTimer,
    CurrentSoundTimer,
    SpriteAddress(usize),
    RandomNum(u8),
}

// all possible ALU operations
#[derive(Debug)]
pub enum ALUOperations {
    Assign,
    Add,
    Subtract,
    SubtractFlipped,
    Or,
    And,
    Xor,
    ShiftRight,
    ShiftLeft,
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
    ALUOperation { op: ALUOperations, left: CPUInstrTarget, right: CPUInstrTarget, },
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
                        addr: CPUInstrTarget::MemoryAddress(instruction_operands as usize),
                    }
                }
            },

            // 0x1NNN: jumps to address 0xNNN
            0x1 => CPUInstruction::Jump {
                addr: CPUInstrTarget::MemoryAddress(instruction_operands as usize),
            },

            // 0x2NNN: calls subroutine at address 0xNNN
            0x2 => CPUInstruction::CallSubroutine {
                addr: CPUInstrTarget::MemoryAddress(instruction_operands as usize),
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
                op: ALUOperations::Add,
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
                    0x4 => ALUOperations::Add,
                    0x5 => ALUOperations::Subtract,
                    0x6 => ALUOperations::ShiftRight,
                    0x7 => ALUOperations::SubtractFlipped,
                    0xE => ALUOperations::ShiftLeft,
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
                left: CPUInstrTarget::CurrentKeyPressed,
                right: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
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
                        op: ALUOperations::Add,
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
                        x: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                    },

                    // 0xFX65: register load V0..Vx from I..I+x
                    0x65 => CPUInstruction::RegisterLoad {
                        x: CPUInstrTarget::VRegister(instruction_operands_list[0] as usize),
                    },

                    _ => CPUInstruction::Unknown { opcode }
                }
            }

            // unknown instruction
            _ => CPUInstruction::Unknown { opcode },
        }
    }

    // evaluates a CPUInstrTarget immutably
    pub fn evaluate_cpu_instr_target(&self, target: CPUInstrTarget) -> usize {
        match target {
            CPUInstrTarget::IRegister => self.registers.get_i_register().clone() as usize,
            CPUInstrTarget::VRegister(reg) => self.registers.get_v_register(reg).clone() as usize,
            CPUInstrTarget::MemoryAddress(addr) => self.memory.get_memory_at(addr).clone() as usize,
            CPUInstrTarget::Constant(val) => val as usize,
            CPUInstrTarget::CurrentKeyPressed => todo!(),
            CPUInstrTarget::CurrentDelayTimer => todo!(),
            CPUInstrTarget::CurrentSoundTimer => todo!(),
            CPUInstrTarget::SpriteAddress(sprite) => todo!(),
            CPUInstrTarget::RandomNum(mask) => todo!(),
        }
    }

    // evaluates a CPUInstrTarget mutably (not all types are valid)
    pub fn set_cpu_instr_target(&mut self, target: CPUInstrTarget, val: usize) {
        match target {
            CPUInstrTarget::IRegister => *self.registers.get_i_register_mut() = (val as u16),
            CPUInstrTarget::VRegister(reg) => *self.registers.get_v_register_mut(reg) = (val as u8),
            CPUInstrTarget::MemoryAddress(addr) => *self.memory.get_memory_at_mut(addr) = (val as u8),
            CPUInstrTarget::CurrentDelayTimer => todo!(),
            CPUInstrTarget::CurrentSoundTimer => todo!(),

            _ => todo!(),
        }
    }

    // executes a CPUInstruction
    fn execute_instruction(&mut self, instr: CPUInstruction) {
        match instr {
            // calls machine code at given address
            CPUInstruction::CallMachineCode { addr } => todo!(),

            // clear display
            CPUInstruction::ClearDisplay => todo!(),

            // returns from subroutine
            CPUInstruction::Return => todo!(),

            // jumps (sets PC) to given address
            CPUInstruction::Jump { addr } => {
                let addr = self.evaluate_cpu_instr_target(addr);

                *self.registers.get_pc_register_mut() = addr as u16;
            }

            // calls subroutine at given address
            CPUInstruction::CallSubroutine { addr } => todo!(),

            // compares the two given arguments, if eq is true, skip the next instruction if they are equal (do nothing if not). if eq is false, skip the next instruction if they are not equal (do nothing if equal)
            CPUInstruction::CompareEq { eq, left, right } => todo!(),

            // assigns a given value to a given target
            CPUInstruction::Assignment { to, from } => {
                let from_val = self.evaluate_cpu_instr_target(from);
                self.set_cpu_instr_target(to, from_val);
            }

            // performs an ALU operation
            CPUInstruction::ALUOperation { op, left, right } => todo!(),

            // performs a "special jump": sets PC to V0 + a given number
            CPUInstruction::SpecialJump { offset } => {
                let v0_val = self.registers.get_v_register(0).clone() as usize;
                let offset_val = self.evaluate_cpu_instr_target(offset);

                *self.registers.get_pc_register_mut() = (v0_val + offset_val) as u16;
            }

            // draws a sprite to the screen at (Vx, Vy) with a height of N pixels
            CPUInstruction::Draw { x_reg, y_reg, height_px } => todo!(),

            // decomposes Vx into BCD at addresses I..I+2
            CPUInstruction::BCD { x_reg } => todo!(),

            // dumps V0..Vx starting at address I
            CPUInstruction::RegisterDump { x } => todo!(),

            // loads V0..Vx starting at address I
            CPUInstruction::RegisterLoad { x } => todo!(),

            // unknown instruction
            CPUInstruction::Unknown { opcode } => todo!(),
        }
    }
}