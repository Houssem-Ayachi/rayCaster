#![allow(nonstandard_style)]

use crate::{
    Vectors::vec2d::Vec2d,
    Traits::SceneTrait::TSceneObj,
    Input::KeysPressed, Angles::Angle
};
use sdl2::{
    render::Canvas,
    video::Window,
    pixels::Color,
    rect::{Rect, Point}
};

pub struct Options{
    pub tileWidth: u32,
    pub tileHeight: u32
}

pub struct PlayerS{
    pub position: Vec2d,
    pub lookingAngle: Angle,
    pub options: Options
}

const SPEED: f32 = 200.0;
const TURN_SPEED: f32 = 200.0;
const PLAYER_WIDTH: u32 = 10;

impl PlayerS{
    pub fn new(x: f32, y: f32, tileWidth: u32, tileHeight: u32) -> Self{
        PlayerS {
            position: Vec2d {x, y},
            lookingAngle: Angle::new(400.0),
            options: Options { 
                tileWidth,
                tileHeight,
            }
        }
    }

    //this moves the player in all directions
    pub fn movePlayer(&mut self, delta: f32, keyHandler: &KeysPressed){
        let mut translation = Vec2d::zeros();
        let mut moved = false;
        if keyHandler.A {
            translation.x = -1.0;
            moved = true;
        }else if keyHandler.D {
            translation.x = 1.0;
            moved = true;
        }

        if keyHandler.W {
            translation.y = -1.0;
            moved = true;
        }else if keyHandler.S{
            translation.y = 1.0;
            moved = true;
        }

        if moved {
            translation.normalize();
            translation.multiplyBy(SPEED);
            translation.multiplyBy(delta);
            // translation = Vec2d::normalize(&self.position, &Vec2d::add(&self.position, &translation));

            self.position = Vec2d::add(&self.position, &translation);
        }
    }

    pub fn movePlayerAlongViewDirection(&mut self, delta: f32, keyHandler: &KeysPressed){
        let mut rotated = false;
        let mut direction = 0.0;
        let mut moved = false;
        if keyHandler.A {
            rotated = true;
            self.lookingAngle.value -= 1.0 * delta * TURN_SPEED;
            self.lookingAngle.bind();
        }
        if keyHandler.D {
            rotated = true;
            self.lookingAngle.value += 1.0 * delta * TURN_SPEED;
            self.lookingAngle.bind();
        }

        if keyHandler.W {
            moved = true;
            direction = 1.0;
        }else if keyHandler.S {
            moved = true;
            direction = -1.0;
        }

        if moved {
            let mut translation = self.getLookingDirectionVector();
            translation.multiplyBy(direction);
            translation.multiplyBy(SPEED);
            translation.multiplyBy(delta);
            self.position = Vec2d::add(&self.position, &translation);
        }
    }

    pub fn getLookingDirectionVector(&self) -> Vec2d{
        let lookAngleRads = self.lookingAngle.value.to_radians(); // negative value caus somehow the trigonometry circle is inverted (positive is down, negative is up)
        let lookDirVec = Vec2d::new(lookAngleRads.cos(), lookAngleRads.sin());
        return lookDirVec;
    }

    //DEBUG CODE:
    pub fn drawLookingDirectionLine(&self, canvas: &mut Canvas<Window>){
        // let p1 = Point::new((self.position.x + (PLAYER_WIDTH / 2) as f32) as i32, (self.position.y + (PLAYER_WIDTH / 2) as f32) as i32);
        let p1 = Point::new(self.position.x as i32, self.position.y as i32);
        let mut lookDirVec = self.getLookingDirectionVector();
        lookDirVec.multiplyBy(50.0);
        let p2 = Point::new((self.position.x + lookDirVec.x) as i32, (self.position.y + lookDirVec.y) as i32);
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_line(p1, p2).ok().unwrap();
    }
}

impl TSceneObj for PlayerS{
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::BLUE);
        let rect = Rect::new(self.position.x as i32, self.position.y as i32, PLAYER_WIDTH, PLAYER_WIDTH);
        canvas.fill_rect(rect).unwrap();

        self.drawLookingDirectionLine(canvas);
    }

    fn update(&mut self, delta: f32) {
    }
}