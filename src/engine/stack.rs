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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_puis_pop() {
        let mut stack = Stack::new();
        stack.push(0x300).unwrap();
        assert_eq!(stack.pop().unwrap(), 0x300);
    }

    #[test]
    fn pop_pile_vide_renvoie_une_erreur() {
        let mut stack = Stack::new();
        assert!(stack.pop().is_err());
    }

    #[test]
    fn pile_est_lifo() {
        let mut stack = Stack::new();
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();
        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 1);
    }

    #[test]
    fn stack_overflow_au_dela_de_16() {
        let mut stack = Stack::new();
        for i in 0..16 {
            assert!(stack.push(i).is_ok());
        }
        assert!(stack.push(99).is_err());
    }
}
