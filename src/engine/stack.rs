use thiserror::Error;

const STACK_SIZE: usize = 16; // Maximum stack size

#[derive(Error, Debug)]
pub enum StackError {
    #[error("Stack overflow")]
    Overflow,
    #[error("Stack underflow")]
    Underflow,
}

pub struct Stack {
    stack: Vec<u16>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn push(&mut self, value: u16) -> Result<(), StackError> {
        if self.stack.len() >= STACK_SIZE {
            return Err(StackError::Overflow);
        }
        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<u16, StackError> {
        if let Some(value) = self.stack.pop() {
            Ok(value)
        } else {
            Err(StackError::Underflow)
        } 
    }
}
