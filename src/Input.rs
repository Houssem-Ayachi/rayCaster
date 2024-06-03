#![allow(nonstandard_style)]

use sdl2::keyboard::Keycode;

#[derive(Debug)]
pub struct KeysPressed{
    pub W: bool,
    pub A: bool,
    pub S: bool,
    pub D: bool,
}

impl KeysPressed{
    pub fn new() -> Self{
        KeysPressed { 
            W: false,
            A: false,
            S: false,
            D: false,
        }
    }

    pub fn keyUp(&mut self, keycode: Keycode){
        match keycode {
            Keycode::W => {self.W = false},
            Keycode::A => {self.A = false},
            Keycode::S => {self.S = false},
            Keycode::D => {self.D = false},
            _ => {}
        }
    }

    pub fn keyDown(&mut self, keycode: Keycode){
        match keycode {
            Keycode::W => {self.W = true},
            Keycode::A => {self.A = true},
            Keycode::S => {self.S = true},
            Keycode::D => {self.D = true},
            _ => {}
        }
    }
}