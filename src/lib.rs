pub mod instruction;
pub mod chip8;
pub mod opcode;
pub mod sdl;

use std::collections::HashMap;

pub trait StateHandler {
    fn handle_state(&mut self, state: crate::chip8::State);
}

pub enum ApplicationState {
    Running,
    Stopping,
}

pub trait KeyboardHandler {
    fn handle_keyboard(&mut self) -> (&HashMap<u8, bool>, Option<u8>, ApplicationState);
}