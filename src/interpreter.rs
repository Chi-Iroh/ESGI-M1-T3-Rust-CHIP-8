use crate::engine::{Engine, EngineOpcode};
use crate::engine::cpu::CPUOpcode;

// TODO: mettre un enum pour les erreurs d'interpréteur (avec thiserror::Error) et les renvoyer à la place de String

pub fn interpreter(program: &[u8], engine: &mut Engine) -> Result<(), String> {
    // current engine.program_counter == 0x200
    engine.load_program(program).map_err(|e| e.to_string())?; 

    // stop when detect 0x0000 or when program_counter is out of bounds (end of memory)
    while engine.program_counter < 0xFFF {
        let pc = engine.program_counter as usize;
        let opcode = (engine.memory[pc] as u16) << 8 | (engine.memory[pc + 1] as u16); // Fetch the next opcode (2 bytes) : 0x60, 0x0A => 0x600A
        engine.program_counter += 2; // move to the next instruction

        match opcode {
            0x0000 => break, // fin de prog // ! non comforme
            0x1000..=0x1FFF => {
                let address = opcode & 0x0FFF;
                engine.opcode_0x1_nnn(address).map_err(|e| e.to_string())?;
            },
            0x3000..=0x3FFF => {
                let no_register = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;
                engine.opcode_0x3_xnn(no_register, value).map_err(|e| e.to_string())?;
            },
            0x4000..=0x4FFF => {
                let no_register = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;
                engine.opcode_0x4_xnn(no_register, value).map_err(|e| e.to_string())?;
            },
            0x5000..=0x5FFF => {
                let no_register_x = ((opcode & 0x0F00) >> 8) as u8;
                let no_register_y = ((opcode & 0x00F0) >> 4) as u8;
                engine.opcode_0x5_xy0(no_register_x, no_register_y).map_err(|e| e.to_string())?;
            },
            0x9000..=0x9FFF => {
                let no_register_x = ((opcode & 0x0F00) >> 8) as u8;
                let no_register_y = ((opcode & 0x00F0) >> 4) as u8;
                engine.opcode_0x9_xy0(no_register_x, no_register_y).map_err(|e| e.to_string())?;
            },
            0x6000..=0x6FFF => {
                let no_register = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;
                engine.cpu.opcode_0x6_xnn(no_register, value).map_err(|e| e.to_string())?;
            },
            0x7000..=0x7FFF => {
                let no_register = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;
                engine.cpu.opcode_0x7_xnn(no_register, value).map_err(|e| e.to_string())?;
            },
            0x8000..=0x8FFF => {
                let no_register_x = ((opcode & 0x0F00) >> 8) as u8;
                let no_register_y = ((opcode & 0x00F0) >> 4) as u8;
                match opcode & 0x000F {
                    0x0000 => engine.cpu.opcode_0x8_xy0(no_register_x, no_register_y).map_err(|e| e.to_string())?,
                    0x0001 => engine.cpu.opcode_0x8_xy1(no_register_x, no_register_y).map_err(|e| e.to_string())?,
                    0x0002 => engine.cpu.opcode_0x8_xy2(no_register_x, no_register_y).map_err(|e| e.to_string())?,
                    0x0003 => engine.cpu.opcode_0x8_xy3(no_register_x, no_register_y).map_err(|e| e.to_string())?,
                    0x0004 => engine.cpu.opcode_0x8_xy4(no_register_x, no_register_y).map_err(|e| e.to_string())?,
                    0x0005 => engine.cpu.opcode_0x8_xy5(no_register_x, no_register_y).map_err(|e| e.to_string())?,
                    0x0007 => engine.cpu.opcode_0x8_xy7(no_register_x, no_register_y).map_err(|e| e.to_string())?,
                    _ => return Err(format!("Unknown opcode: {:04X}", opcode)),
                }
            },
            
            _ => return Err(format!("Unknown opcode: {:04X}", opcode)),
        }
    }

    Ok(())

}