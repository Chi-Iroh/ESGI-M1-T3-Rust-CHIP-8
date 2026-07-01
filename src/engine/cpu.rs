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

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn cpu_new_initialise_16_registres_a_zero() {
        let cpu = CPU::new();
        for n in 0u8..16 {
            assert_eq!(cpu.get_register(n), 0);
        }
    }

    #[test]
    fn set_register_puis_get_register() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 42);
        assert_eq!(cpu.get_register(0), 42);
    }


    #[test]
    fn action_addvalue_ajoute_une_constante() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 10);
        cpu.action(CPUAction::AddValue(0, 5));
        assert_eq!(cpu.get_register(0), 15);
    }

    #[test]
    fn action_addvalue_wrap_ne_touche_pas_vf() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 250);
        cpu.action(CPUAction::AddValue(0, 10));
        assert_eq!(cpu.get_register(0), 4);
        assert_eq!(cpu.get_register(0xF), 0);
    }


    #[test]
    fn action_copy_copie_vy_dans_vx() {
        let mut cpu = CPU::new();
        cpu.set_register(1, 7);
        cpu.action(CPUAction::Copy(0, 1));
        assert_eq!(cpu.get_register(0), 7);
    }

    #[test]
    fn action_or_bit_a_bit() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 0b1100);
        cpu.set_register(1, 0b1010);
        cpu.action(CPUAction::Or(0, 1));
        assert_eq!(cpu.get_register(0), 0b1110);
    }

    #[test]
    fn action_and_bit_a_bit() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 0b1100);
        cpu.set_register(1, 0b1010);
        cpu.action(CPUAction::And(0, 1));
        assert_eq!(cpu.get_register(0), 0b1000);
    }

    #[test]
    fn action_xor_bit_a_bit() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 0b1100);
        cpu.set_register(1, 0b1010);
        cpu.action(CPUAction::Xor(0, 1));
        assert_eq!(cpu.get_register(0), 0b0110);
    }


    #[test]
    fn action_add_sans_overflow_vf_0() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 10);
        cpu.set_register(1, 5);
        cpu.action(CPUAction::Add(0, 1));
        assert_eq!(cpu.get_register(0), 15);
        assert_eq!(cpu.get_register(0xF), 0);
    }

    #[test]
    fn action_add_avec_overflow_vf_1() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 250);
        cpu.set_register(1, 10);
        cpu.action(CPUAction::Add(0, 1));
        assert_eq!(cpu.get_register(0), 4);
        assert_eq!(cpu.get_register(0xF), 1);
    }


    #[test]
    fn action_sub_sans_underflow_vf_1() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 20);
        cpu.set_register(1, 5);
        cpu.action(CPUAction::Sub(0, 1));
        assert_eq!(cpu.get_register(0), 15);
        assert_eq!(cpu.get_register(0xF), 1);
    }

    #[test]
    fn action_sub_avec_underflow_vf_0() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 5);
        cpu.set_register(1, 20);
        cpu.action(CPUAction::Sub(0, 1));
        assert_eq!(cpu.get_register(0), 241);
        assert_eq!(cpu.get_register(0xF), 0);
    }


    #[test]
    fn action_subfrom_sans_underflow_vf_1() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 5);
        cpu.set_register(1, 20);
        cpu.action(CPUAction::SubFrom(0, 1));
        assert_eq!(cpu.get_register(0), 15);
        assert_eq!(cpu.get_register(0xF), 1);
    }

    #[test]
    fn action_subfrom_avec_underflow_vf_0() {
        let mut cpu = CPU::new();
        cpu.set_register(0, 20);
        cpu.set_register(1, 5);
        cpu.action(CPUAction::SubFrom(0, 1));
        assert_eq!(cpu.get_register(0), 241);
        assert_eq!(cpu.get_register(0xF), 0);
    }
}
