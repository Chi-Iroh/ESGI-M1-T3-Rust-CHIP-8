const NUM_REGISTERS: u8 = 16;

/**
 * ELM(Taget, Source)
 * target: index of the register to modify
 * source: index of the register to use as source OR a value (u8) to add
 */
pub enum CPUAction {
    AddValue(usize, u8),   // Add NN to register Vx
    Copy(usize, usize),    // Set Vx = Vy
    Or(usize, usize),      // Set Vx = Vx OR Vy
    And(usize, usize),     // Set Vx = Vx AND Vy
    Xor(usize, usize),     // Set Vx = Vx XOR Vy
    Add(usize, usize),     // Set Vx = Vx + Vy
    Sub(usize, usize),     // Set Vx = Vx - Vy
    SubFrom(usize, usize), // Set Vx = Vy - Vx
}

pub struct CPU {
    // 16 registers, represented by an array of u8 values
    registers: [u8; NUM_REGISTERS as usize],
}

impl CPU {
    pub fn new() -> Self {
        let registers = [0u8; NUM_REGISTERS as usize];
        CPU { registers }
    }

    pub fn draw_current_registers(&self) {
        // X X X X
        for i in 0..NUM_REGISTERS {
            print!("{:02X} ", self.registers[i as usize]);
            if i % 4 == 3 {
                println!();
            }
        }
    }

    pub fn set_register(&mut self, no_register: u8, value: u8) {
        self.registers[no_register as usize] = value;
    }

    pub fn get_register(&self, no_register: u8) -> u8 {
        self.registers[no_register as usize]
    }

    pub fn action(&mut self, action: CPUAction) {
        match action {
            CPUAction::AddValue(reg, value) => {
                let (result, _) = self.registers[reg].overflowing_add(value);
                // * overflow ignored according to the spec
                self.registers[reg] = result;
            }
            CPUAction::Copy(dest, src) => {
                self.registers[dest] = self.registers[src];
            }
            CPUAction::Or(dest, src) => {
                self.registers[dest] |= self.registers[src];
            }
            CPUAction::And(dest, src) => {
                self.registers[dest] &= self.registers[src];
            }
            CPUAction::Xor(dest, src) => {
                self.registers[dest] ^= self.registers[src];
            }
            CPUAction::Add(dest, src) => {
                let (result, overflowed) =
                    self.registers[dest].overflowing_add(self.registers[src]);

                self.registers[dest] = result;

                if overflowed {
                    self.registers[0xf] = 1; // Set VF to 1 on overflow
                } else {
                    self.registers[0xf] = 0; // Set VF to 0 if no overflow
                }
            }
            CPUAction::Sub(dest, src) => {
                let (result, underflowed) =
                    self.registers[dest].overflowing_sub(self.registers[src]);
                self.registers[dest] = result;

                if underflowed {
                    self.registers[0xf] = 0; // Set VF to 0 on underflow
                } else {
                    self.registers[0xf] = 1; // Set VF to 1 if no underflow
                }
            }
            CPUAction::SubFrom(dest, src) => {
                let (result, underflowed) =
                    self.registers[src].overflowing_sub(self.registers[dest]);
                self.registers[dest] = result;

                if underflowed {
                    self.registers[0xf] = 0; // Set VF to 0 on underflow
                } else {
                    self.registers[0xf] = 1; // Set VF to 1 if no underflow
                }
            }
        }
    }
}
