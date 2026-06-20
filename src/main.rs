mod engine;
// use engine::cpu::{CPU, CPUError, CPUOpcode};


// fn main() {
//     let mut engine = CPU::new();

//     engine.draw_current_registers();

//     engine.opcode_0x6_xnn(0, 10).unwrap();
//     engine.opcode_0x7_xnn(1, 50).unwrap();
//     engine.opcode_0x8_xy0(2, 0).unwrap();
//     engine.opcode_0x8_xy1(3, 1).unwrap();
//     engine.opcode_0x8_xy5(1, 0).unwrap();
//     engine.draw_current_registers();
// }


mod interpreter;
use interpreter::interpreter;
use engine::Engine;

fn main() {
    let mut engine = Engine::new();

    let program: [u8; 12] = [
        0x60, 0x0A, // 0x600A: Set V0 to 10
        0x61, 0x14, // 0x6114: Set V1 to 20
        0x80, 0x14, // 0x8014: Add V1 to V0
        0x90, 0x10, // 0x9010: Skip next instruction if V0 != V1
        0x00, 0x01, // * 0x0001: Unknow opcode (will be skipped if V0 != V1)
        0x00, 0x00 // ! (non comforme), markeur de fin de programme
    ];

    engine.draw_current_registers();

    match interpreter(&program, &mut engine) {
        Ok(_) => {
            println!("Program executed successfully.");
            engine.draw_current_registers();
        }
        Err(e) => {
            println!("Error during execution: {}", e);
        }
    }
}
