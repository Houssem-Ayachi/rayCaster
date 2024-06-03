#![allow(nonstandard_style)]

use crate::Traits::SceneTrait::TSceneObj;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(PartialEq, Eq)]
pub enum SquareType {
    EMPTY,
    WALL
}

pub struct Square{
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub sType: SquareType
}

impl Square{
    pub fn isEmpty(&self) -> bool {
        return self.sType == SquareType::EMPTY;
    }
}

impl TSceneObj for Square{
    fn draw(&self, canvas: &mut Canvas<Window>){
        let color = match self.sType{
            SquareType::EMPTY => Color::GREEN,
            SquareType::WALL => Color::RED
        };
        canvas.set_draw_color(color);
        let rect = Rect::new((self.y * self.width) as i32, (self.x * self.height) as i32, self.width-1, self.height-1);
        canvas.fill_rect(rect).ok().unwrap();
        
    }

    fn update(&mut self, delta: f32){

    }
}