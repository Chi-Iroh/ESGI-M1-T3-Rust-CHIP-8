pub mod cpu;

use cpu::{CPU, CPUAction};
use thiserror::Error;

const MEMORY_SIZE: usize = 4096; // 4KB of memory 
pub const PROGRAM_RAM_ADDRESS_RANGE: (u16, u16) = (0x200, 0xFFF);

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Program counter out of bounds: {0}")]
    ProgramCounterOutOfBounds(u16),
    #[error("Load program error: {0}")]
    LoadProgramError(String),
}

pub struct Engine {
    cpu: cpu::CPU,
    pub program_counter: u16,
    pub memory: [u8; MEMORY_SIZE],
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
        let max_size = (PROGRAM_RAM_ADDRESS_RANGE.1 - PROGRAM_RAM_ADDRESS_RANGE.0 + 1) as usize;
        if program.len() > max_size {
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

pub trait Opcode {
    fn opcode_0x1_nnn(&mut self, adress: u16) -> Result<(), EngineError>; // jump to address NNN 
    fn opcode_0x2_nnn(&mut self, adress: u16) -> Result<(), EngineError>; // call subroutine at NNN
    fn opcode_0x3_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError>; // skip next instruction if Vx == NN 
    fn opcode_0x4_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError>; // skip next instruction if Vx != NN 
    fn opcode_0x00_ee(&mut self) -> Result<(), EngineError>; // return from subroutine
    fn opcode_0x5_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError>; // skip next instruction if Vx == Vy 
    fn opcode_0x9_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError>; // skip next instruction if Vx != Vy 
    fn opcode_0x6_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError>; // set Vx = NN 
    fn opcode_0x7_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError>; // set Vx = Vx + NN 
    fn opcode_0x8_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError>; // set Vx = Vy 
    fn opcode_0x8_xy1(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError>; // set Vx = Vx OR Vy 
    fn opcode_0x8_xy2(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError>; // set Vx = Vx AND Vy 
    fn opcode_0x8_xy3(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError>; // set Vx = Vx XOR Vy 
    fn opcode_0x8_xy4(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError>; // set Vx = Vx + Vy 
    fn opcode_0x8_xy5(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError>; // set Vx = Vx - Vy 
    fn opcode_0x8_xy7(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError>; // set Vx = Vy - Vx
}

impl Opcode for Engine {
    fn opcode_0x1_nnn(&mut self, adress: u16) -> Result<(), EngineError> {
        if adress < PROGRAM_RAM_ADDRESS_RANGE.0 || adress > PROGRAM_RAM_ADDRESS_RANGE.1 {
            return Err(EngineError::ProgramCounterOutOfBounds(adress));
        }

        self.program_counter = adress;
        Ok(())
    }

    // TODO: implementer l'opcode 0x2_nnn
    fn opcode_0x2_nnn(&mut self, adress: u16) -> Result<(), EngineError> {
        panic!("Not implemented yet: opcode_0x2_nnn");
    }

    fn opcode_0x3_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError> {
        if self.cpu.get_register(no_register) == value {
            self.program_counter += 2; // Skip next instruction
        }
        Ok(())
    }

    fn opcode_0x4_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError> {
        if self.cpu.get_register(no_register) != value {
            self.program_counter += 2; // Skip next instruction
        }
        Ok(())
    }

    // TODO: implementer l'opcode 0x00_ee
    fn opcode_0x00_ee(&mut self) -> Result<(), EngineError> {
        panic!("Not implemented yet: opcode_0x00_ee");
    }

    fn opcode_0x5_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError> {
        if self.cpu.get_register(no_register_x) == self.cpu.get_register(no_register_y) {
            self.program_counter += 2; // Skip next instruction
        }
        Ok(())
    }

    fn opcode_0x9_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError> {
        if self.cpu.get_register(no_register_x) != self.cpu.get_register(no_register_y) {
            self.program_counter += 2; // Skip next instruction
        }
        Ok(())
    }

    fn opcode_0x6_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError> {
        self.cpu.set_register(no_register, value);
        Ok(())
    }

    fn opcode_0x7_xnn(&mut self, no_register: u8, value: u8) -> Result<(), EngineError> {
        self.cpu
            .action(CPUAction::AddValue(no_register as usize, value));
        Ok(())
    }

    fn opcode_0x8_xy0(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError> {
        self.cpu.action(CPUAction::Copy(
            no_register_x as usize,
            no_register_y as usize,
        ));
        Ok(())
    }

    fn opcode_0x8_xy1(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError> {
        self.cpu.action(CPUAction::Or(
            no_register_x as usize,
            no_register_y as usize,
        ));
        Ok(())
    }

    fn opcode_0x8_xy2(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError> {
        self.cpu.action(CPUAction::And(
            no_register_x as usize,
            no_register_y as usize,
        ));
        Ok(())
    }

    fn opcode_0x8_xy3(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError> {
        self.cpu.action(CPUAction::Xor(
            no_register_x as usize,
            no_register_y as usize,
        ));
        Ok(())
    }

    fn opcode_0x8_xy4(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError> {
        self.cpu.action(CPUAction::Add(
            no_register_x as usize,
            no_register_y as usize,
        ));
        Ok(())
    }

    fn opcode_0x8_xy5(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError> {
        self.cpu.action(CPUAction::Sub(
            no_register_x as usize,
            no_register_y as usize,
        ));
        Ok(())
    }

    fn opcode_0x8_xy7(&mut self, no_register_x: u8, no_register_y: u8) -> Result<(), EngineError> {
        self.cpu.action(CPUAction::SubFrom(
            no_register_x as usize,
            no_register_y as usize,
        ));
        Ok(())
    }
}
