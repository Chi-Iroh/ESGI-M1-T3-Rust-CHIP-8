pub mod cpu;
pub mod stack;

use cpu::{CPU, CPUAction};
use stack::{Stack, StackError};
use thiserror::Error;

const MEMORY_SIZE: usize = 4096; // 4KB of memory 
pub const PROGRAM_RAM_ADDRESS_RANGE: (u16, u16) = (0x200, 0xFFF);

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Program counter out of bounds: {0}")]
    ProgramCounterOutOfBounds(u16),
    #[error("Load program error: {0}")]
    LoadProgramError(String),
    #[error("Stack error: {0}")]
    StackError(#[from] StackError),
}

pub struct Engine {
    cpu: cpu::CPU,
    pub program_counter: u16,
    pub memory: [u8; MEMORY_SIZE],
    stack: Stack,
    program_size: usize
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            cpu: CPU::new(),
            program_counter: PROGRAM_RAM_ADDRESS_RANGE.0,
            memory: [0; MEMORY_SIZE],
            stack: Stack::new(),
            program_size: 0
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
        self.program_size = program.len();

        Ok(())
    }

    pub fn out_of_bounds(&self) -> bool {
        self.program_counter >= (PROGRAM_RAM_ADDRESS_RANGE.0 + self.program_size as u16)
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

    fn opcode_0x2_nnn(&mut self, adress: u16) -> Result<(), EngineError> {
        if adress < PROGRAM_RAM_ADDRESS_RANGE.0 || adress > PROGRAM_RAM_ADDRESS_RANGE.1 {
            return Err(EngineError::ProgramCounterOutOfBounds(adress));
        }

        self.stack.push(self.program_counter)?;
        self.program_counter = adress;
        Ok(())
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

    fn opcode_0x00_ee(&mut self) -> Result<(), EngineError> {
        let return_address = self.stack.pop()?;
        self.program_counter = return_address;
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Interpreter;



    #[test]
    fn engine_new_pc_demarre_a_0x200() {
        let engine = Engine::new();
        assert_eq!(engine.program_counter, 0x200);
    }


    #[test]
    fn opcode_0x1_saute_a_l_adresse() {
        let mut engine = Engine::new();
        let resultat = engine.opcode_0x1_nnn(0x300);
        assert!(resultat.is_ok());
        assert_eq!(engine.program_counter, 0x300);
    }

    #[test]
    fn opcode_0x1_hors_bornes_renvoie_une_erreur() {
        let mut engine = Engine::new();
        let resultat = engine.opcode_0x1_nnn(0x100);
        assert!(resultat.is_err());
    }


    #[test]
    fn opcode_0x6_ecrit_le_registre() {
        let mut engine = Engine::new();
        engine.opcode_0x6_xnn(0, 42).unwrap();
        assert_eq!(engine.cpu.get_register(0), 42);
    }

    #[test]
    fn opcode_0x7_ajoute_au_registre() {
        let mut engine = Engine::new();
        engine.opcode_0x6_xnn(0, 10).unwrap();
        engine.opcode_0x7_xnn(0, 5).unwrap();
        assert_eq!(engine.cpu.get_register(0), 15);
    }

    #[test]
    fn opcode_0x8xy4_additionne_et_pose_vf() {
        let mut engine = Engine::new();
        engine.opcode_0x6_xnn(0, 250).unwrap();
        engine.opcode_0x6_xnn(1, 10).unwrap();
        engine.opcode_0x8_xy4(0, 1).unwrap();
        assert_eq!(engine.cpu.get_register(0), 4);
        assert_eq!(engine.cpu.get_register(0xF), 1);
    }


    #[test]
    fn opcode_0x3_saute_si_egal() {
        let mut engine = Engine::new();
        engine.opcode_0x6_xnn(0, 5).unwrap();
        engine.opcode_0x3_xnn(0, 5).unwrap();
        assert_eq!(engine.program_counter, 0x202);
    }

    #[test]
    fn opcode_0x3_ne_saute_pas_si_different() {
        let mut engine = Engine::new();
        engine.opcode_0x6_xnn(0, 5).unwrap();
        engine.opcode_0x3_xnn(0, 9).unwrap();
        assert_eq!(engine.program_counter, 0x200);
    }

    #[test]
    fn opcode_0x4_saute_si_different() {
        let mut engine = Engine::new();
        engine.opcode_0x6_xnn(0, 5).unwrap();
        engine.opcode_0x4_xnn(0, 9).unwrap();
        assert_eq!(engine.program_counter, 0x202);
    }

    #[test]
    fn opcode_0x5_saute_si_vx_egal_vy() {
        let mut engine = Engine::new();
        engine.opcode_0x6_xnn(0, 7).unwrap();
        engine.opcode_0x6_xnn(1, 7).unwrap();
        engine.opcode_0x5_xy0(0, 1).unwrap();
        assert_eq!(engine.program_counter, 0x202);
    }

    #[test]
    fn opcode_0x9_saute_si_vx_different_vy() {
        let mut engine = Engine::new();
        engine.opcode_0x6_xnn(0, 7).unwrap();
        engine.opcode_0x6_xnn(1, 8).unwrap();
        engine.opcode_0x9_xy0(0, 1).unwrap();
        assert_eq!(engine.program_counter, 0x202);
    }


    #[test]
    fn opcode_0x2_call_puis_0x00ee_return() {
        let mut engine = Engine::new();
        engine.opcode_0x2_nnn(0x400).unwrap();
        assert_eq!(engine.program_counter, 0x400);
        engine.opcode_0x00_ee().unwrap();
        assert_eq!(engine.program_counter, 0x200);
    }

    #[test]
    fn opcode_0x00ee_pile_vide_renvoie_une_erreur() {
        let mut engine = Engine::new();
        let resultat = engine.opcode_0x00_ee();
        assert!(resultat.is_err());
    }


    #[test]
    fn load_program_copie_a_0x200() {
        let mut engine = Engine::new();
        engine.load_program(&[0xAB, 0xCD]).unwrap();
        assert_eq!(engine.memory[0x200], 0xAB);
        assert_eq!(engine.memory[0x201], 0xCD);
    }

    #[test]
    fn load_program_trop_gros_renvoie_une_erreur() {
        let mut engine = Engine::new();
        let gros = vec![0u8; 5000];
        let resultat = engine.load_program(&gros);
        assert!(resultat.is_err());
    }


    #[test]
    fn integration_programme_additionne_dans_v0() {
        let programme = [
            0x60, 0x05,
            0x61, 0x0A,
            0x62, 0x0A,
            0x63, 0x0A,
            0x80, 0x14,
            0x80, 0x24,
            0x80, 0x34,
        ];
        let mut engine = Engine::new();
        engine.load_program(&programme).unwrap();
        let resultat = Interpreter::new().run(&mut engine);
        assert!(resultat.is_ok());
        assert_eq!(engine.cpu.get_register(0), 35);
    }

    #[test]
    fn integration_opcode_inconnu_renvoie_une_erreur() {
        let programme = [0xF0, 0x00];
        let mut engine = Engine::new();
        engine.load_program(&programme).unwrap();
        let resultat = Interpreter::new().run(&mut engine);
        assert!(resultat.is_err());
    }
}
