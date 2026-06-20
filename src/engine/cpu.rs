use std::collections::HashMap;
use thiserror::Error;

const NUM_REGISTERS: u8 = 16;

pub struct Register {
    pub value: u8,
}

#[derive(Error, Debug)]
pub enum CPUError {
    #[error("Register not found: {0}")]
    RegisterNotFound(u8),
    #[error("Value overflow occurred")]
    ValOverflow,
    #[error("Value underflow occurred")]
    ValUnderflow,
}

pub struct CPU {
    // 16 registers, represented by a Register struct
    pub registers: HashMap<u8, Register>,
}

impl CPU {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        for i in 0..NUM_REGISTERS {
            registers.insert(i, Register { value: 0 });
        }
        CPU {
            registers,
        }
    }

    pub fn draw_current_registers(&self) {
        // X X X X
        for i in 0..16 {
            if let Some(register) = self.registers.get(&i) {
                print!("{:02X} ", register.value);
            }
            if i % 4 == 3 {
                println!();
            }
        }
        println!();
    }
}

pub trait CPUOpcode {
    // fn opcode_0x1_nnn(&mut self, adress: u16) -> Result<(), ()>; // jump to address NNN
    // fn opcode_0x2_nnn(&mut self, adress: u16) -> Result<(), ()>; // call subroutine at NNN
    // fn opcode_0x3_xnn(&mut self, no_register: u8, value: u8) -> Result<(), ()>; // skip next instruction if Vx == NN
    // fn opcode_0x4_xnn(&mut self, no_register: u8, value: u8) -> Result<(), ()>; // skip next instruction if Vx != NN
    // fn opcode_0x00_ee(&mut self) -> Result<(), ()>; // return from subroutine
    // fn opcode_0x5_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), ()>; // skip next instruction if Vx == Vy
    // fn opcode_0x9_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), ()>; // skip next instruction if Vx != Vy
    fn opcode_0x6_xnn(&mut self, no_register: u8, value: u8) -> Result<(), CPUError>; // set Vx = NN
    fn opcode_0x7_xnn(&mut self, no_register: u8, value: u8) -> Result<(), CPUError>; // set Vx = Vx + NN
    fn opcode_0x8_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError>; // set Vx = Vy
    fn opcode_0x8_xy1(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError>; // set Vx = Vx OR Vy
    fn opcode_0x8_xy2(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError>; // set Vx = Vx AND Vy
    fn opcode_0x8_xy3(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError>; // set Vx = Vx XOR Vy
    fn opcode_0x8_xy4(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError>; // set Vx = Vx + Vy
    fn opcode_0x8_xy5(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError>; // set Vx = Vx - Vy
    fn opcode_0x8_xy7(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError>; // set Vx = Vy - Vx
}

impl CPUOpcode for CPU {

    fn opcode_0x6_xnn(&mut self, no_register: u8, value: u8) -> Result<(), CPUError> {
        if let Some(register) = self.registers.get_mut(&no_register) {
            register.value = value;
            Ok(())
        } else {
            Err(CPUError::RegisterNotFound(no_register))
        }
    }

    fn opcode_0x7_xnn(&mut self, no_register: u8, value: u8) -> Result<(), CPUError> {
        if let Some(register) = self.registers.get_mut(&no_register) {
            if (register.value as u16 + value as u16) > 255 {
                return Err(CPUError::ValOverflow);
            }
            register.value += value; // rx = rx + NN
            Ok(())
        } else {
            Err(CPUError::RegisterNotFound(no_register))
        }
    }

    fn opcode_0x8_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError> {
        let value_y;

        if let Some(register_y) = self.registers.get(&no_register_y) {
            value_y = register_y.value;
        } else {
            return Err(CPUError::RegisterNotFound(no_register_y));
        }

        if let Some(register_x) = self.registers.get_mut(&no_register_x) {
            register_x.value = value_y;
            Ok(())
        } else {
            Err(CPUError::RegisterNotFound(no_register_x))
        }
    }

    fn opcode_0x8_xy1(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError> {
        let value_y;

        if let Some(register_y) = self.registers.get(&no_register_y) {
            value_y = register_y.value;
        } else {
            return Err(CPUError::RegisterNotFound(no_register_y));
        }

        if let Some(register_x) = self.registers.get_mut(&no_register_x) {
            register_x.value |= value_y; // rx = rx OR ry
            Ok(())
        } else {
            Err(CPUError::RegisterNotFound(no_register_x))
        }
    }

    fn opcode_0x8_xy2(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError> {
        let value_y;

        if let Some(register_y) = self.registers.get(&no_register_y) {
            value_y = register_y.value;
        } else {
            return Err(CPUError::RegisterNotFound(no_register_y));
        }

        if let Some(register_x) = self.registers.get_mut(&no_register_x) {
            register_x.value &= value_y; // rx = rx AND ry
            Ok(())
        } else {
            Err(CPUError::RegisterNotFound(no_register_x))
        }
    }

    fn opcode_0x8_xy3(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError> {
        let value_y;

        if let Some(register_y) = self.registers.get(&no_register_y) {
            value_y = register_y.value;
        } else {
            return Err(CPUError::RegisterNotFound(no_register_y));
        }

        if let Some(register_x) = self.registers.get_mut(&no_register_x) {
            register_x.value ^= value_y; // rx = rx XOR ry
            Ok(())
        } else {
            Err(CPUError::RegisterNotFound(no_register_x))
        }
    }

    fn opcode_0x8_xy4(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError> {
        let value_y;

        if let Some(register_y) = self.registers.get(&no_register_y) {
            value_y = register_y.value;
        } else {
            return Err(CPUError::RegisterNotFound(no_register_y));
        }

        if let Some(register_x) = self.registers.get_mut(&no_register_x) {

            if (register_x.value as u16 + value_y as u16) > 255 {
                return Err(CPUError::ValOverflow);
            }

            register_x.value += value_y;
            Ok(())
        } else {
            Err(CPUError::RegisterNotFound(no_register_x))
        }
    }

    fn opcode_0x8_xy5(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError> {
        let value_y;

        if let Some(register_y) = self.registers.get(&no_register_y) {
            value_y = register_y.value;
        } else {
            return Err(CPUError::RegisterNotFound(no_register_y));
        }

        if let Some(register_x) = self.registers.get_mut(&no_register_x) {

            // ? may add `&& value_y > 0` check, since it's u8, it shouldn't be needed
            if register_x.value < value_y {
                return Err(CPUError::ValUnderflow);
            }

            register_x.value -= value_y; // rx = rx - ry
            Ok(())
        } else {
            Err(CPUError::RegisterNotFound(no_register_x))
        }
    }

    fn opcode_0x8_xy7(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), CPUError> {
        let value_x;
        let value_y;

        if let Some(register_x) = self.registers.get(&no_register_x) {
            value_x = register_x.value;
        } else {
            return Err(CPUError::RegisterNotFound(no_register_x));
        }

        if let Some(register_y) = self.registers.get(&no_register_y) {
            value_y = register_y.value;
        } else {
            return Err(CPUError::RegisterNotFound(no_register_y));
        }

        if let Some(register_x) = self.registers.get_mut(&no_register_x) {

            // ? may add `&& value_x > 0` check, since it's u8, it shouldn't be needed
            if value_y < value_x {
                return Err(CPUError::ValUnderflow);
            }

            register_x.value = value_y - value_x; // rx = ry - rx
            Ok(())
        } else {
            Err(CPUError::RegisterNotFound(no_register_x))
        }

    }


}
