#![allow(nonstandard_style)]

#[derive(Debug)]
pub struct Vec2d{
    pub x: f32,
    pub y: f32
}

impl Vec2d{
    pub fn new(x: f32, y: f32) -> Self{
        Vec2d {
            x,
            y,
        }
    }

    pub fn zeros() -> Self{
        Vec2d { 
            x: 0.0,
            y: 0.0
        }
    }

    pub fn difference(v1: &Vec2d, v2: &Vec2d) -> Self{
        return Vec2d {
            x: v1.x - v2.x,
            y: v2.y - v2.y
        }
    }

    pub fn length(&self) -> f32{
        return ((self.x * self.x) + (self.y * self.y)).sqrt();
    }

    pub fn multiplyBy(&mut self, value: f32){
        self.x *= value; 
        self.y *= value;
    }

    pub fn add(source: &Vec2d, other: &Vec2d) -> Vec2d{
        let x = source.x + other.x;
        let y = source.y + other.y;
        Vec2d { 
            x,
            y,
        }
    }

    pub fn normalize(&mut self){
        let distance = (self.x * self.x + self.y * self.y).sqrt();
        self.x /= distance;
        self.y /= distance;
    }

}