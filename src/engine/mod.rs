pub mod cpu;
use self::cpu::{CPU, CPUError};

use thiserror::Error;

const MEMORY_SIZE: usize = 4096; // 4KB of memory
const PROGRAM_RAM_ADDRESS_RANGE: (u16, u16) = (0x200, 0xFFF);

pub struct Engine {
    pub cpu: cpu::CPU,
    pub program_counter: u16,
    pub memory: [u8; MEMORY_SIZE],
}

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Program counter out of bounds: {0}")]
    ProgramCounterOutOfBounds(u16),
    #[error("CPU error: {0}")]
    CPUError(#[from] CPUError),
    #[error("Load program error: {0}")]
    LoadProgramError(String),
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            cpu: CPU::new(),
            program_counter: PROGRAM_RAM_ADDRESS_RANGE.0,
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn draw_current_registers(&self) {
        self.cpu.draw_current_registers();
    }

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), EngineError> {
        if program.len() > (PROGRAM_RAM_ADDRESS_RANGE.1 - PROGRAM_RAM_ADDRESS_RANGE.0 + 1) as usize
        {
            return Err(EngineError::LoadProgramError(format!(
                "Program size exceeds ROM address range: {} bytes",
                program.len()
            )));
        }

        self.memory[PROGRAM_RAM_ADDRESS_RANGE.0 as usize
            ..(PROGRAM_RAM_ADDRESS_RANGE.0 as usize + program.len())]
            .copy_from_slice(program);

        Ok(())
    }
}

pub trait EngineOpcode {
    fn opcode_0x1_nnn(&mut self, adress: u16) -> Result<(), EngineError>; // jump to address NNN
    // fn opcode_0x2_nnn(&mut self, adress: u16) -> Result<(), EngineError>; // call subroutine at NNN
    fn opcode_0x3_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError>; // skip next instruction if Vx == NN
    fn opcode_0x4_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError>; // skip next instruction if Vx != NN
    // fn opcode_0x00_ee(&mut self) -> Result<(), EngineError>; // return from subroutine
    fn opcode_0x5_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError>; // skip next instruction if Vx == Vy
    fn opcode_0x9_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError>; // skip next instruction if Vx != Vy
}

impl EngineOpcode for Engine {
    fn opcode_0x1_nnn(&mut self, adress: u16) -> Result<(), EngineError> {
        
        if adress < PROGRAM_RAM_ADDRESS_RANGE.0 || adress > PROGRAM_RAM_ADDRESS_RANGE.1 {
            return Err(EngineError::ProgramCounterOutOfBounds(adress));
        }

        self.program_counter = adress;
        Ok(())
    }

    // fn opcode_0x2_nnn(&mut self, adress: u16) -> Result<(), EngineError>;

    fn opcode_0x3_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError> {
        if let Some(register) = self.cpu.registers.get(&no_register) {
            if register.value == value {
                self.program_counter += 2; // Skip next instruction
            }
            Ok(())
        } else {
            Err(EngineError::CPUError(CPUError::RegisterNotFound(no_register)))
        }
    }

    fn opcode_0x4_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError> {
        if let Some(register) = self.cpu.registers.get(&no_register) {
            if register.value != value {
                self.program_counter += 2; // Skip next instruction
            }
            Ok(())
        } else {
            Err(EngineError::CPUError(CPUError::RegisterNotFound(no_register)))
        }
    }

    // fn opcode_0x00_ee(&mut self) -> Result<(), EngineError>;

    fn opcode_0x5_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError> {
        let value_x;
        let value_y;

        if let Some(register_x) = self.cpu.registers.get(&no_register_x) {
            value_x = register_x.value;
        } else {
            return Err(EngineError::CPUError(CPUError::RegisterNotFound(no_register_x)));
        }

        if let Some(register_y) = self.cpu.registers.get(&no_register_y) {
            value_y = register_y.value;
        } else {
            return Err(EngineError::CPUError(CPUError::RegisterNotFound(no_register_y)));
        }

        if value_x == value_y {
            self.program_counter += 2; // Skip next instruction
        }
        Ok(())
    }

    fn opcode_0x9_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError> {
        let value_x;
        let value_y;

        if let Some(register_x) = self.cpu.registers.get(&no_register_x) {
            value_x = register_x.value;
        } else {
            return Err(EngineError::CPUError(CPUError::RegisterNotFound(no_register_x)));
        }

        if let Some(register_y) = self.cpu.registers.get(&no_register_y) {
            value_y = register_y.value;
        } else {
            return Err(EngineError::CPUError(CPUError::RegisterNotFound(no_register_y)));
        }

        if value_x != value_y {
            self.program_counter += 2; // Skip next instruction
        }
        Ok(())
    }

}
