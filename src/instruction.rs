pub use crate::opcode::OpCode;

#[derive(Debug)]
pub struct Instruction {
    pub instruction: u16
}

impl Instruction {
    pub fn new(instruction: u16) -> Instruction {
        Instruction { instruction }
    }

    fn addr(&self) -> u16 {
        self.instruction & 0x0FFF
    }
    
    fn nibble(&self) -> u16 {
        self.instruction & 0x000F
    }
    
    fn x(&self) -> u16 {
        (self.instruction & 0x0F00) >> 8
    }
    
    fn y(&self) -> u16 {
        (self.instruction & 0x00F0) >> 4
    }
    
    fn byte(&self) -> u16 {
        self.instruction & 0x00FF
    }

    fn tuple(&self) -> (u16, u16, u16, u16) {
        (
            (self.instruction & 0xF000) >> 12,
            (self.instruction & 0x0F00) >> 8,
            (self.instruction & 0x00F0) >> 4,
            (self.instruction & 0x000F)
        )
    }
}

#[test]
fn test_addr() {
    let inst = Instruction::new(0xDE1A);
    assert_eq!(0xE1A, inst.addr());
}

#[test]
fn test_nibble() {
    let inst = Instruction::new(0xDE1A);
    assert_eq!(0xA, inst.nibble());
}

#[test]
fn test_x() {
    let inst = Instruction::new(0xDE1A);
    assert_eq!(0xE, inst.x());
}

#[test]
fn test_y() {
    let inst = Instruction::new(0xDE1A);
    assert_eq!(0x1, inst.y());
}

#[test]
fn test_byte() {
    let inst = Instruction::new(0xDE1A);
    assert_eq!(0x1A, inst.byte());
}


#[test]
fn test_tuple() {
    let inst = Instruction::new(0xDE1A);
    assert_eq!((0xD, 0xE, 0x1, 0xA), inst.tuple());
}

impl From<u16> for Instruction {
    fn from(instruction: u16) -> Self {
        Instruction::new(instruction)
    }
}

impl From<Instruction> for OpCode {
    fn from(instruction: Instruction) -> Self {
        match instruction.tuple() {
            (0, 0, 0xE, 0) => OpCode::CLS,
            (0, 0, 0xE, 0xE) => OpCode::RET,
            (0, _, _, _) => OpCode::NOOP,
            (1, _, _, _) => OpCode::JP { addr: instruction.addr() },
            (2, _, _, _) => OpCode::CALL { addr: instruction.addr() },
            (3, _, _, _) => OpCode::SE { vx: instruction.x(), other: instruction.byte(), by_value: true },
            (4, _, _, _) => OpCode::SNE { vx: instruction.x(), other: instruction.byte(), by_value: true },
            (5, _, _, 0) => OpCode::SE { vx: instruction.x(), other: instruction.y(), by_value: false },
            (6, _, _, _) => OpCode::LD { vx: instruction.x(), other: instruction.byte(), by_value: true },
            (7, _, _, _) => OpCode::ADD { vx: instruction.x(), byte: instruction.byte() },
            (8, _, _, 0) => OpCode::LD { vx: instruction.x(), other: instruction.y(), by_value: false },
            (8, _, _, 1) => OpCode::OR { vx: instruction.x(), vy: instruction.y() },
            (8, _, _, 2) => OpCode::AND { vx: instruction.x(), vy: instruction.y() },
            (8, _, _, 3) => OpCode::XOR { vx: instruction.x(), vy: instruction.y() },
            (8, _, _, 4) => OpCode::ADDREG { vx: instruction.x(), vy: instruction.y() },
            (8, _, _, 5) => OpCode::SUB { vx: instruction.x(), vy: instruction.y() },
            (8, _, _, 6) => OpCode::SHR { vx: instruction.x(), vy: instruction.y() },
            (8, _, _, 7) => OpCode::SUBN { vx: instruction.x(), vy: instruction.y() },
            (8, _, _, 0xE) => OpCode::SHL { vx: instruction.x(), vy: instruction.y() },
            (9, _, _, 0) => OpCode::SNE { vx: instruction.x(), other: instruction.y(), by_value: false },
            (0xA, _, _, _) => OpCode::LDI { addr: instruction.addr() },
            (0xB, _, _, _) => OpCode::JPV0 { addr: instruction.addr() },
            (0xC, _, _, _) => OpCode::RND { vx: instruction.x(), byte: instruction.byte() },
            (0xD, _, _, _) => OpCode::DRW { vx: instruction.x(), vy: instruction.y(), nibble: instruction.nibble() },
            (0xE, _, 9, 0xE) => OpCode::SKP { vx: instruction.x() },
            (0xE, _, 0xA, 0x1) => OpCode::SKNP { vx: instruction.x() },
            (0xF, _, 0x0, 0x7) => OpCode::LDVXDT { vx: instruction.x() },
            (0xF, _, 0x0, 0xA) => OpCode::LDK { vx: instruction.x() },
            (0xF, _, 0x1, 0x5) => OpCode::LDDTVX { vx: instruction.x() },
            (0xF, _, 0x1, 0x8) => OpCode::LDSTVX { vx: instruction.x() },
            (0xF, _, 0x1, 0xE) => OpCode::ADDI { vx: instruction.x() },
            (0xF, _, 0x2, 0x9) => OpCode::LDF { vx: instruction.x() },
            (0xF, _, 0x3, 0x3) => OpCode::LDB { vx: instruction.x() },
            (0xF, _, 0x5, 0x5) => OpCode::LDMEMI { vx: instruction.x() },
            (0xF, _, 0x6, 0x5) => OpCode::LDVXMEMI { vx: instruction.x() },
            _ => OpCode::NOOP
        }
    }
}