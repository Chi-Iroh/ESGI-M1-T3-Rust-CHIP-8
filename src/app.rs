use crate::engine::{Engine, EngineError};
use crate::interpreter::{Interpreter, InterpreterError};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Engine error: {0}")]
    Engine(#[from] EngineError),
    #[error("Interpreter error: {0}")]
    Interpreter(#[from] InterpreterError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct App {
    engine: Engine,
    interpreter: Interpreter,
}


impl App {
    fn new() -> Self {
        App {
            engine: Engine::new(),
            interpreter: Interpreter::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), AppError> {
        self.interpreter.run(&mut self.engine)?;
        Ok(())
    }

    pub fn _new_from_byte(program: &[u8]) -> Result<Self, AppError> {
        let mut app = App::new();
        app.engine.load_program(program)?;
        Ok(app)
    }

    pub fn new_from_file<P: AsRef<Path>>(path: P) -> Result<Self, AppError> {
        // https://doc.rust-lang.org/beta/std/fs/fn.read.html
        let program = std::fs::read(path)?;
        let mut app = App::new();
        app.engine.load_program(&program)?;
        Ok(app)
    }

    pub fn draw_current_registers(&self) {
        self.engine.draw_current_registers();
    }
}
