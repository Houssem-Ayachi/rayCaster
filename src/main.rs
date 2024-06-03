#![allow(nonstandard_style)]
extern crate sdl2;

use sdl2::{event::Event, pixels::Color, rect::Rect};
use std::time::Instant;

mod Map;
mod Square;
mod Scene;
mod Traits;
mod Vectors;
mod Player;
mod Input;
mod Rays;
mod Angles;

use Scene::SceneHandler;
use Traits::SceneTrait::TSceneObj;
use Input::KeysPressed;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 750;

fn main() {
    //sdl inits
    //first window map only
    let context = sdl2::init().unwrap();
    let videoSubSystem: sdl2::VideoSubsystem = context.video().unwrap();
    let window = videoSubSystem.window("Map", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut eventPump = context.event_pump().unwrap();

    //game logic init
    let keyHandler = KeysPressed::new();

    let mut lastCheck = Instant::now(); // used to calculate delta time

    let (width, height) = canvas.output_size().unwrap();
    let mut Scene= SceneHandler::new(width, height, keyHandler);

    //game loop
    'running: loop {
        canvas.clear();
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)).ok().unwrap();
        //calculate delta time
        let now = Instant::now();
        let delta = now.duration_since(lastCheck).as_secs_f32();
        lastCheck = now;

        //handle events
        for event in eventPump.poll_iter(){
            match event {
                Event::Quit { .. } => break 'running,
                // Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {},
                Event::KeyDown {keycode, ..} => {
                    Scene.keyHandler.keyDown(keycode.unwrap());
                },
                Event::KeyUp { keycode , ..} => {
                    Scene.keyHandler.keyUp(keycode.unwrap());
                }
                _ => {}
            }
        }

        //update game state
        Scene.update(delta);

        //render frames
        Scene.draw(&mut canvas);

        canvas.present();
    }
}
