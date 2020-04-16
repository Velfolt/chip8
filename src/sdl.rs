extern crate sdl2;

use crate::{ApplicationState, KeyboardHandler, StateHandler};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::audio::{AudioCallback, AudioSpecDesired};

use std::collections::HashMap;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct SdlEngine {
    device: sdl2::audio::AudioDevice<SquareWave>,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    keyboard: HashMap<u8, bool>,
}

impl SdlEngine {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("chip8", 800, 400)
            .position_centered()
            .build().unwrap();

        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let bg_color = Color::RGB(0, 0, 0);
        canvas.set_draw_color(bg_color);
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        let mut keyboard = HashMap::new();
        keyboard.insert(1, false);
        keyboard.insert(2, false);
        keyboard.insert(3, false);
        keyboard.insert(0xC, false);
        keyboard.insert(0x4, false);
        keyboard.insert(0x5, false);
        keyboard.insert(0x6, false);
        keyboard.insert(0xD, false);
        keyboard.insert(0x7, false);
        keyboard.insert(0x8, false);
        keyboard.insert(0x9, false);
        keyboard.insert(0xE, false);
        keyboard.insert(0xA, false);
        keyboard.insert(0x0, false);
        keyboard.insert(0xB, false);
        keyboard.insert(0xF, false);

        SdlEngine { device, canvas, event_pump, keyboard }
    }
}

impl StateHandler for SdlEngine {
    fn handle_state(&mut self, state: crate::chip8::State) { 
        let bg_color = Color::RGB(0, 0, 0);
        let fg_color = Color::RGB(255, 255, 255);

        if state.update_display {
            self.canvas.set_draw_color(bg_color);
            self.canvas.clear();
            self.canvas.set_draw_color(fg_color);

            let mut rects = vec!();
            
            for y in 0..32 {
                for x in 0..64 {
                    if state.display[y*64+x] > 0 {
                        rects.push(Rect::new((x * (800/64)) as i32, (y * (400/32))as i32, 800/64 - 1, 400/32 - 1));
                    }
                }
            }

            self.canvas.fill_rects(&rects).unwrap();
            self.canvas.present();
        }

        if state.play_audio {
            self.device.resume();
        } else {
            self.device.pause();
        }
     }
}

impl KeyboardHandler for SdlEngine {
    fn handle_keyboard(&mut self) -> (&std::collections::HashMap<u8, bool>, std::option::Option<u8>, ApplicationState) { 
        let mut keydown = None;
        let mut application_state = ApplicationState::Running;
       
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    application_state = ApplicationState::Stopping;
                },
                Event::KeyDown { keycode, .. } => {
                    let key = match keycode {
                        Some(Keycode::Num1) => 1,
                        Some(Keycode::Num2) => 2,
                        Some(Keycode::Num3) => 3,
                        Some(Keycode::Num4) => 0xC,
                        Some(Keycode::Q) => 4,
                        Some(Keycode::W) => 5,
                        Some(Keycode::E) => 6,
                        Some(Keycode::R) => 0xD,
                        Some(Keycode::A) => 7,
                        Some(Keycode::S) => 8,
                        Some(Keycode::D) => 9,
                        Some(Keycode::F) => 0xE,
                        Some(Keycode::Z) => 0xA,
                        Some(Keycode::X) => 0,
                        Some(Keycode::C) => 0xB,
                        Some(Keycode::V) => 0xF,
                        _ => 0
                    };

                    keydown = Some(key);
                   
                    self.keyboard.insert(key, true);
                },
                Event::KeyUp { keycode, .. } => {
                    let key = match keycode {
                        Some(Keycode::Num1) => 1,
                        Some(Keycode::Num2) => 2,
                        Some(Keycode::Num3) => 3,
                        Some(Keycode::Num4) => 0xC,
                        Some(Keycode::Q) => 4,
                        Some(Keycode::W) => 5,
                        Some(Keycode::E) => 6,
                        Some(Keycode::R) => 0xD,
                        Some(Keycode::A) => 7,
                        Some(Keycode::S) => 8,
                        Some(Keycode::D) => 9,
                        Some(Keycode::F) => 0xE,
                        Some(Keycode::Z) => 0xA,
                        Some(Keycode::X) => 0,
                        Some(Keycode::C) => 0xB,
                        Some(Keycode::V) => 0xF,
                        _ => 0
                    };
                   
                    self.keyboard.insert(key, false);
                },
                _ => {}
            }
        }

        (&self.keyboard, keydown, application_state)
     }
}