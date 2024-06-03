#![allow(nonstandard_style)]

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{render::Canvas, rect::Point};
use sdl2::video::Window;

use crate::{
    Traits::SceneTrait::TSceneObj, 
    Map::S_Map, 
    Player::PlayerS, 
    Input::KeysPressed, Rays::Ray
};

pub struct SceneHandler{
    numRays: i32,
    Map: S_Map,
    Player: PlayerS,
    pub keyHandler: KeysPressed
}

impl SceneHandler{
    pub fn new(_width: u32, _height: u32, keyHandler: KeysPressed) -> Self{
        let rows = 10;
        let cols = 10;
        let width = _width / rows;
        let height = _height / cols;
        return SceneHandler {
            Map: S_Map::new(rows, cols, (width, height)),
            Player: PlayerS::new(200.0, 200.0, width, height),
            keyHandler,
            numRays: 100,
        };
    }

    fn drawRay(&self, canvas: &mut Canvas<Window>){
        let yRay = Ray::castRay(&self.Map, &self.Player.position, self.Player.lookingAngle.value, self.Player.lookingAngle.value.to_radians());
        let p1 = Point::new(self.Player.position.x as i32, self.Player.position.y as i32);
        let p2 = Point::new(yRay.endPoint.x as i32, yRay.endPoint.y as i32);
        canvas.set_draw_color(Color::BLACK);
        canvas.draw_line(p1, p2).unwrap();
    }

    fn drawRays(&self, canvas: &mut Canvas<Window>){
        let rays = Ray::castRays(&self.Map, &self.Player.position, 40.0, self.numRays, &self.Player.lookingAngle);
        let p1 = Point::new(self.Player.position.x as i32, self.Player.position.y as i32);
        for ray in rays.iter(){
            let p2 = Point::new(ray.endPoint.x as i32, ray.endPoint.y as i32);
            canvas.set_draw_color(Color::BLACK);
            canvas.draw_line(p1, p2).unwrap();
        }
    }

    pub fn drawRays3d(&self, canvas: &mut Canvas<Window>){
        let step = crate::SCREEN_WIDTH as i32 / self.numRays;
        let rays = Ray::castRays(&self.Map, &self.Player.position, 90.0, self.numRays, &self.Player.lookingAngle);
        // let screenMiddle = crate::SCREEN_HEIGHT as i32 / 2;
        for (i, ray) in rays.iter().enumerate(){
            println!("here");
            let lineHeight = crate::SCREEN_HEIGHT as f32 / ray.length;
            let mut startY = -(lineHeight as i32) / 2 + crate::SCREEN_HEIGHT as i32 / 2;
            if startY > crate::SCREEN_HEIGHT as i32{
                startY = crate::SCREEN_HEIGHT as i32 - 20;
            }
            let p1 = Point::new(
                i as i32 * step,
                -(lineHeight as i32) / 2 + crate::SCREEN_HEIGHT as i32 / 2
            );
            let p2 = Point::new(
                i as i32 * step,
                (lineHeight as i32 / 2) + crate::SCREEN_HEIGHT as i32 / 2
            );
            canvas.set_draw_color(Color::BLACK);
            canvas.draw_line(p1,p2).ok().unwrap();
            
            // canvas.fill_rect(Rect::new(
            //     i as i32 * step,
            //     screenMiddle - (lineHeight as i32 / 2),
            //     step as u32,
            //     lineHeight as u32
            // )).ok().unwrap();
        }
    }
}

impl TSceneObj for SceneHandler{
    fn draw(&self, canvas: &mut Canvas<Window>){
        self.Map.draw(canvas);
        self.drawRays(canvas);
        // self.drawRays3d(canvas);
        self.Player.draw(canvas);
    }

    fn update(&mut self, delta: f32){
        self.Player.movePlayerAlongViewDirection(delta, &self.keyHandler);
        self.Player.update(delta);
    }
}