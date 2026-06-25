use crate::engine::{Engine, EngineError, Opcode, PROGRAM_RAM_ADDRESS_RANGE, stack};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Unknown opcode: {0:04X}")]
    UnknownOpcode(u16),
    #[error(transparent)]
    Engine(#[from] EngineError),
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn run(&self, engine: &mut Engine) -> Result<(), InterpreterError> {
        // current engine.program_counter == 0x200

        // stop when detect 0x0000 or when program_counter is out of bounds (end of memory)
        while engine.program_counter < PROGRAM_RAM_ADDRESS_RANGE.1 {
            let opcode = fetch(engine);

            engine.program_counter += 2; // move to the next instruction

            match self.execute(opcode, engine) {
                Ok(_) => {}
                // The stack underflow error indicates that the program has finished:
                // we use the 0x00ee opcode as end of program signal, so we can break the loop
                Err(InterpreterError::Engine(EngineError::StackError(
                    stack::StackError::Underflow,
                ))) => break,
                
                Err(e) => return Err(e),
            }

        }

        Ok(())
    }

    fn execute(&self, opcode: u16, engine: &mut Engine) -> Result<(), InterpreterError> {
        match opcode {
            0x00EE => engine.opcode_0x00_ee()?,
            0x1000..=0x1FFF => {
                let address = opcode & 0x0FFF;
                engine.opcode_0x1_nnn(address)?;
            }
            0x2000..=0x2FFF => {
                let address = opcode & 0x0FFF;
                engine.opcode_0x2_nnn(address)?;
            }
            0x3000..=0x3FFF => {
                let (no_register, value) = decode_xnn(opcode);
                engine.opcode_0x3_xnn(no_register, value)?;
            }
            0x4000..=0x4FFF => {
                let (no_register, value) = decode_xnn(opcode);
                engine.opcode_0x4_xnn(no_register, value)?;
            }
            0x5000..=0x5FFF => {
                let (x, y) = decode_xy(opcode);
                engine.opcode_0x5_xy0(x, y)?;
            }
            0x9000..=0x9FFF => {
                let (x, y) = decode_xy(opcode);
                engine.opcode_0x9_xy0(x, y)?;
            }
            0x6000..=0x6FFF => {
                let (no_register, value) = decode_xnn(opcode);
                engine.opcode_0x6_xnn(no_register, value)?;
            }
            0x7000..=0x7FFF => {
                let (no_register, value) = decode_xnn(opcode);
                engine.opcode_0x7_xnn(no_register, value)?;
            }
            0x8000..=0x8FFF => {
                let (x, y) = decode_xy(opcode);
                match opcode & 0x000F {
                    0x0000 => engine.opcode_0x8_xy0(x, y)?,
                    0x0001 => engine.opcode_0x8_xy1(x, y)?,
                    0x0002 => engine.opcode_0x8_xy2(x, y)?,
                    0x0003 => engine.opcode_0x8_xy3(x, y)?,
                    0x0004 => engine.opcode_0x8_xy4(x, y)?,
                    0x0005 => engine.opcode_0x8_xy5(x, y)?,
                    0x0007 => engine.opcode_0x8_xy7(x, y)?,
                    _ => return Err(InterpreterError::UnknownOpcode(opcode)),
                }
            }
            _ => return Err(InterpreterError::UnknownOpcode(opcode)),
        }

        Ok(())
    }
}

// -- utils --

// Fetch the next opcode (2 bytes) : 0x60, 0x0A => 0x600A
fn fetch(engine: &Engine) -> u16 {
    let pc = engine.program_counter as usize;
    (engine.memory[pc] as u16) << 8 | (engine.memory[pc + 1] as u16)
}

// 0x_XNN => (X, NN)
fn decode_xnn(opcode: u16) -> (u8, u8) {
    let no_register = ((opcode & 0x0F00) >> 8) as u8;
    let value = (opcode & 0x00FF) as u8;
    (no_register, value)
}

// 0x_XY_ => (X, Y)
fn decode_xy(opcode: u16) -> (u8, u8) {
    let no_register_x = ((opcode & 0x0F00) >> 8) as u8;
    let no_register_y = ((opcode & 0x00F0) >> 4) as u8;
    (no_register_x, no_register_y)
}
