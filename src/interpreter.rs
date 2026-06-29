use crate::engine::{Engine, EngineError, Opcode, stack};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Unknown opcode: {0:04X}")]
    UnknownOpcode(u16),
    #[error(transparent)]
    Engine(#[from] EngineError),
}

struct OpcodeArgs {
    pub x: u8,
    pub y: u8,
    pub nn: u8,
    pub nnn: u16
}

enum OpcodeCallback {
    Callback(fn(&mut Engine) -> Result<(), EngineError>),
    CallbackXY(fn(&mut Engine, u8, u8) -> Result<(), EngineError>),
    CallbackXNN(fn(&mut Engine, u8, u8) -> Result<(), EngineError>),
    CallbackNNN(fn(&mut Engine, u16) -> Result<(), EngineError>)
}

impl OpcodeArgs {
    fn from(opcode: u16) -> Self {
        Self {
            x: ((opcode & 0x0F00) >> 8) as u8,
            y: ((opcode & 0x00F0) >> 4) as u8,
            nn: opcode as u8,
            nnn: (opcode & 0x0FFF) as u16
        }
    }

    fn call_opcode(&self, engine: &mut Engine, opcode_callback: OpcodeCallback) -> Result<(), EngineError> {
        match opcode_callback {
            OpcodeCallback::Callback(f) => f(engine),
            OpcodeCallback::CallbackXY(f) => f(engine, self.x, self.y),
            OpcodeCallback::CallbackXNN(f) => f(engine, self.x, self.nn),
            OpcodeCallback::CallbackNNN(f) => f(engine, self.nnn),
        }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn run(&self, engine: &mut Engine) -> Result<(), InterpreterError> {
        // current engine.program_counter == 0x200

        // stop when detect 0x0000 or when program_counter is out of bounds (end of memory)
        while let Some(opcode) = fetch(engine) {
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
        use OpcodeCallback::*;

        let opcode_args = OpcodeArgs::from(opcode);
        let opcode_callbacks = [
            (0xFFFF, 0x00EE, Callback(Engine::opcode_0x00_ee)),
            (0xF000, 0x1000, CallbackNNN(Engine::opcode_0x1_nnn)),
            (0xF000, 0x2000, CallbackNNN(Engine::opcode_0x2_nnn)),
            (0xF000, 0x3000, CallbackXNN(Engine::opcode_0x3_xnn)),
            (0xF000, 0x4000, CallbackXNN(Engine::opcode_0x4_xnn)),
            (0xF00F, 0x5000, CallbackXY(Engine::opcode_0x5_xy0)),
            (0xF00F, 0x9000, CallbackXY(Engine::opcode_0x9_xy0)),
            (0xF000, 0x6000, CallbackXNN(Engine::opcode_0x6_xnn)),
            (0xF000, 0x7000, CallbackXNN(Engine::opcode_0x7_xnn)),
            (0xF00F, 0x8000, CallbackXY(Engine::opcode_0x8_xy0)),
            (0xF00F, 0x8001, CallbackXY(Engine::opcode_0x8_xy1)),
            (0xF00F, 0x8002, CallbackXY(Engine::opcode_0x8_xy2)),
            (0xF00F, 0x8003, CallbackXY(Engine::opcode_0x8_xy3)),
            (0xF00F, 0x8004, CallbackXY(Engine::opcode_0x8_xy4)),
            (0xF00F, 0x8005, CallbackXY(Engine::opcode_0x8_xy5)),
            (0xF00F, 0x8007, CallbackXY(Engine::opcode_0x8_xy7))
        ];

        for (mask, result, callback) in opcode_callbacks {
            if opcode & mask == result {
                return opcode_args.call_opcode(engine, callback).map_err(InterpreterError::Engine);
            }
        }
        Err(InterpreterError::UnknownOpcode(opcode))
    }
}

// -- utils --

// Fetch the next opcode (2 bytes) : 0x60, 0x0A => 0x600A
fn fetch(engine: &Engine) -> Option<u16> {
    let pc = engine.program_counter as usize;

    if engine.out_of_bounds() {
        None
    } else {
        Some((engine.memory[pc] as u16) << 8 | (engine.memory[pc + 1] as u16))
    }
}
