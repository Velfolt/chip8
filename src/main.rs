use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;
use std::env;

use chip8::{StateHandler, KeyboardHandler, ApplicationState};

fn read_opcodes(filename: &String) -> ([u8; 3584], usize) {
    let mut f = File::open(filename).expect("file not found");
    let mut buffer = [0u8; 3584];

    let bytes_read = if let Ok(bytes_read) = f.read(&mut buffer) {
        bytes_read
    } else {
        0
    };

    (buffer, bytes_read)
}

fn print_opcodes(buffer: &[u8], bytes_read: usize) {
    for i in (0..bytes_read).step_by(2) {
        let opcode: chip8::opcode::OpCode = chip8::instruction::Instruction::from(((buffer[i] as u16) << 8) + buffer[i+1] as u16).into();
        println!("{}: {:02X}{:02X} {:?}", i + 0x200, buffer[i], buffer[i+1], opcode);
    }
}

fn main() {
    println!("chip8 emulator by Velfolt");
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        println!("Usage: {} romfile", args[0]);
        return;
    }
    
    let (buffer, bytes_read) = read_opcodes(&args[1]);
    print_opcodes(&buffer, bytes_read);

    let mut chip8 = chip8::chip8::Chip8::new_program(buffer.to_vec());
    let mut engine = chip8::sdl::SdlEngine::new();

    loop {
        let (keyboard, keydown, application_state) = engine.handle_keyboard();

        if let ApplicationState::Stopping = application_state {
            break;
        }

        let state = chip8.step(keyboard, keydown);
        engine.handle_state(state);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1200));
    }
}
