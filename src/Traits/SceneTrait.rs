#![allow(nonstandard_style)]

use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait TSceneObj {
    fn draw(&self, canvas: &mut Canvas<Window>);
    fn update(&mut self, delta: f32);
}