use crate::opcode::OpCode;
use crate::instruction::Instruction;
extern crate rand;

use std::fmt;
use std::time::Instant;
use std::collections::HashMap;

pub struct State {
    pub display: [u8; 64*32],
    pub update_display: bool,
    pub play_audio: bool,
    pub waiting_for_input: bool,
}

impl fmt::Display for State {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result { 
        let mut output: String = String::new();
        for y in 0..32 { 
            for x in 0..64 {
                output.push(self.display[y * 64 + x] as char);
            }
            output.push('|');
            output.push('\n');
        } 
        write!(fmt, "{}\n---", output)
    }
}

#[derive(Clone)]
pub struct Chip8 {
    v: [u8; 16],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    sp: u8,
    stack: Vec<u16>,
    display: [u8; 64*32],
    memory: [u8; 4096],
    last_updated: Instant,
    update_display: bool,
    waiting_for_input_vx: Option<u8>,
}

impl fmt::Debug for Chip8 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Chip8")
           .field("v", &self.v)
           .field("i", &self.i)
           .field("delay_timer", &self.delay_timer)
           .field("sound_timer", &self.sound_timer)
           .field("pc", &self.pc)
           .field("sp", &self.sp)
           .field("stack", &self.stack)
           .finish()
    }
}

impl Chip8 {
    pub fn new_program(program: Vec<u8>) -> Chip8 {
        let mut memory = [0; 4096];

        for (i, data) in program.iter().enumerate() {
            memory[i + 0x200] = *data;
        }

        let fonts = vec!(
            0xF0, // 0
            0x90,
            0x90,   
            0x90,
            0xF0,
            0x20, // 1
            0x60,
            0x20,
            0x20,
            0x70,
            0xF0, // 2
            0x10,
            0xF0,
            0x80,
            0xF0,
            0xF0, // 3
            0x10,
            0xF0,
            0x10,
            0xF0,
            0x90,
            0x90,
            0xF0,
            0x10,
            0x10,
            0xF0,
            0x80,
            0xF0,
            0x10,
            0xF0,
            0xF0,
            0x80,
            0xF0,
            0x90,
            0xF0,
            0xF0,
            0x10,
            0x20,
            0x40,
            0x40,
            0xF0,
            0x90,
            0xF0,
            0x90,
            0xF0,
            0xF0,
            0x90,
            0xF0,
            0x10,
            0xF0,
            0xF0,
            0x90,
            0xF0,
            0x90,
            0x90,
            0xE0,
            0x90,
            0xE0,
            0x90,
            0xE0,
            0xF0,
            0x80,
            0x80,
            0x80,
            0xF0,
            0xE0,
            0x90,
            0x90,
            0x90,
            0xE0,
            0xF0,
            0x80,
            0xF0,
            0x80,
            0xF0,
            0xF0,
            0x80,
            0xF0,
            0x80,
            0x80,
        );

        for (i, data) in fonts.iter().enumerate() {
            memory[i] = *data;
        }

        Chip8 {
            v: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            sp: 0,
            stack: vec!(),
            display: [0; 64 * 32],
            memory: memory,
            last_updated: Instant::now(),
            update_display: false,
            waiting_for_input_vx: None,
        }
    }

    pub fn step(&mut self, keyboard: &HashMap<u8, bool>, keydown: Option<u8>) -> State {
        if let Some(key) = keydown {
            if self.waiting_for_input_vx != None {
                self.v[self.waiting_for_input_vx.unwrap() as usize] = key;
                self.waiting_for_input_vx = None;
            }
        }

        if self.waiting_for_input_vx != None {
            return State { display: self.display, update_display: false, play_audio: self.sound_timer > 0, waiting_for_input: true };
        }

        if self.last_updated.elapsed().as_millis() > 1000/60 {
            self.last_updated = Instant::now();

            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
        }

        let opcode = self.read_opcode();
        self.apply(opcode, keyboard);

        let update_display = self.update_display;
        self.update_display = false;

        State { display: self.display, update_display, play_audio: self.sound_timer > 0, waiting_for_input: false }
    }

    pub fn read_opcode(&self) -> OpCode {
        let instruction = ((self.memory[self.pc as usize] as u16) << 8) 
            + self.memory[(self.pc + 1) as usize] as u16;

        let opcode = Instruction::from(instruction).into();

        //println!("{}: {:04X} {:?}", self.pc, instruction, opcode);

        opcode
    }

    pub fn apply(&mut self, opcode: OpCode, keyboard: &HashMap<u8, bool>) {
        match opcode {
            OpCode::NOOP => {},
            OpCode::CLS => {
                self.display.iter_mut().for_each(|x| *x = 0);
                self.update_display = true;
            },
            OpCode::RET => self.pc = self.stack.pop().unwrap(),
            OpCode::JP { addr } => self.pc = addr - 2,
            OpCode::CALL { addr } => {
                self.stack.push(self.pc); 
                self.pc = addr - 2;
            },
            OpCode::SE { vx, other, by_value } => {
                let value = if by_value { other as u8 } else { self.v[other as usize] };
                if self.v[vx as usize] == value {
                    self.pc += 2;
                }
            },
            OpCode::SNE { vx, other, by_value } => {
                let value = if by_value { other as u8 } else { self.v[other as usize] };
                if self.v[vx as usize] != value {
                    self.pc += 2;
                }
            },
            OpCode::LD { vx, other, by_value } => {
                let value = if by_value { other as u8 } else { self.v[other as usize] };
                self.v[vx as usize] = value;
            },
            OpCode::ADD { vx, byte } => self.v[vx as usize] = (self.v[vx as usize] as u16 + (byte & 0xFF) as u16) as u8,
            OpCode::OR { vx, vy } => self.v[vx as usize] = self.v[vx as usize] | self.v[vy as usize],
            OpCode::AND { vx, vy } => self.v[vx as usize] = self.v[vx as usize] & self.v[vy as usize],
            OpCode::XOR { vx, vy } => self.v[vx as usize] = self.v[vx as usize] ^ self.v[vy as usize],
            OpCode::ADDREG { vx, vy } => {
                let sum = self.v[vx as usize] as u16 + self.v[vy as usize] as u16;
                self.v[0xF] = if sum > 255 { 1 } else { 0 };

                self.v[vx as usize] = sum as u8;
            },
            OpCode::SUB { vx, vy } => {
                if self.v[vx as usize] > self.v[vy as usize] {
                    self.v[0xF] = 1;
                    self.v[vx as usize] = self.v[vx as usize] - self.v[vy as usize];
                } else {
                    self.v[0xF] = 0;
                    self.v[vx as usize] = (self.v[vx as usize] as i16 - self.v[vy as usize] as i16) as u8;
                }
            },
            OpCode::SHR { vx, vy: _ } => {
                self.v[0xF] = self.v[vx as usize] & 0b00000001;
                self.v[vx as usize] = self.v[vx as usize] >> 1;
            },
            OpCode::SUBN { vx, vy } => {
                if self.v[vy as usize] > self.v[vx as usize] {
                    self.v[0xF] = 1;
                    self.v[vx as usize] = self.v[vy as usize] - self.v[vx as usize];
                } else {
                    self.v[0xF] = 0;
                    self.v[vx as usize] = (self.v[vy as usize] as i16 - self.v[vx as usize] as i16) as u8;
                }
            },
            OpCode::SHL { vx, vy: _ } => {
                self.v[0xF] = (self.v[vx as usize] & 0b10000000) >> 7;
                self.v[vx as usize] = self.v[vx as usize] << 1;
            },
            OpCode::LDI { addr } => self.i = addr,
            OpCode::JPV0 { addr } => self.pc = self.v[0] as u16 + addr - 2,
            OpCode::RND { vx, byte } => self.v[vx as usize] = rand::random::<u8>() & byte as u8,
            OpCode::DRW { vx, vy, nibble } => {
                let x = self.v[vx as usize] as usize;
                let y = self.v[vy as usize] as usize;

                self.v[0xF] = 0;

                for yy in 0..nibble {
                    let sprite_part = self.memory[(self.i + yy) as usize];
                    let current_y = (y + yy as usize) % 32;

                    for xx in 0..8 {
                        let current_x = (x + xx) % 64;

                        let index = (current_y * 64 + current_x) as usize;

                        let pixel = (sprite_part >> (7 - xx)) & 0b1;

                        self.v[0xF] |= pixel & self.display[index];
                        self.display[index] ^= pixel;
                    }
                }

                self.update_display = true;
            },
            OpCode::SKP { vx } => if *keyboard.get(&self.v[vx as usize]).unwrap() { self.pc += 2 },
            OpCode::SKNP { vx } => if !*keyboard.get(&self.v[vx as usize]).unwrap() { self.pc += 2 },
            OpCode::LDVXDT { vx } => self.v[vx as usize] = self.delay_timer,
            OpCode::LDK { vx } => {
                self.waiting_for_input_vx = Some(vx as u8);
            },
            OpCode::LDDTVX { vx } => self.delay_timer = self.v[vx as usize],
            OpCode::LDSTVX { vx } => self.sound_timer = self.v[vx as usize],
            OpCode::ADDI { vx } => self.i += self.v[vx as usize] as u16,
            OpCode::LDF { vx } => { 
                let digit = self.v[vx as usize];
                self.i = (digit * 5).into();
            },
            OpCode::LDB { vx } => { 
                let value = self.v[vx as usize];

                self.memory[self.i as usize] = value / 100;
                self.memory[(self.i + 1) as usize] = (value % 100) / 10;
                self.memory[(self.i + 2) as usize] = value % 10;
            },
            OpCode::LDMEMI { vx } => { 
                for i in 0..=vx {
                    self.memory[(self.i + i) as usize] = self.v[i as usize];
                }
            },
            OpCode::LDVXMEMI { vx } => { 
                for i in 0..=vx {
                    self.v[i as usize] = self.memory[(self.i + i) as usize];
                }
            }
        };

        self.pc += 2;
    }
}


