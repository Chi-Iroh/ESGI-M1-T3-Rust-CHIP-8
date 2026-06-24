use crate::engine::{Engine, EngineError};
use crate::interpreter::{Interpreter, InterpreterError};

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

    pub fn run(&mut self) -> Result<(), InterpreterError> {
        self.interpreter.run(&mut self.engine)
    }

    pub fn new_from_byte(program: &[u8]) -> Result<Self, EngineError> {
        let mut app = App::new();
        app.engine.load_program(program)?;
        Ok(app)
    }

    pub fn draw_current_registers(&self) {
        self.engine.draw_current_registers();
    }
}
