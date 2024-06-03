#![allow(nonstandard_style)]

use sdl2::{video::Window, render::Canvas};

use crate::{Square::{Square, SquareType}, Traits::SceneTrait::TSceneObj};

pub struct S_Map{
    pub rows: u32,
    pub cols: u32,
    pub squareWidth: u32,
    pub squareHeight: u32,
    pub cells: Vec<Vec<Square>>
}

impl S_Map{
    pub fn new(rows: u32, cols: u32, size:(u32,u32)) -> Self {
        let mut map = S_Map { 
            rows,
            cols,
            squareWidth: size.0,
            squareHeight: size.1,
            cells: S_Map::initCells(rows, cols, size)
        };
        map.addWalls();
        return map;
    }

    fn initCells(rows: u32, cols: u32, size: (u32, u32)) -> Vec<Vec<Square>>{
        let mut cells: Vec<Vec<Square>> = vec![];
        for i in 0..rows{
            let mut row: Vec<Square> = vec![];
            for j in 0..cols{
                row.push(Square { x: i, y: j, width: size.0, height: size.1, sType: SquareType::EMPTY});
            }
            cells.push(row);
        }
        cells.iter_mut().nth(1).unwrap().iter_mut().nth(2).unwrap().sType = SquareType::WALL;
        cells.iter_mut().nth(4).unwrap().iter_mut().nth(4).unwrap().sType = SquareType::WALL;
        return cells;
    }

    fn addWalls(&mut self){
        //vertical boundaries
        for i in 0..self.rows{
            self.cells.iter_mut().nth(i as usize).unwrap().iter_mut().nth(0).unwrap().sType = SquareType::WALL;
            self.cells.iter_mut().nth(i as usize).unwrap().iter_mut().nth((self.cols-1)as usize).unwrap().sType = SquareType::WALL;
        }

        //horizontal boundaries
        for i in 0..self.cols{
            self.cells.iter_mut().nth(0 as usize).unwrap().iter_mut().nth(i as usize).unwrap().sType = SquareType::WALL;
            self.cells.iter_mut().nth((self.rows - 1) as usize).unwrap().iter_mut().nth(i as usize).unwrap().sType = SquareType::WALL;
        }
    }
}

impl TSceneObj for S_Map{
    fn draw(&self, canvas: &mut Canvas<Window>){
        for squaresRow in self.cells.iter(){
            for square in squaresRow.iter(){
                square.draw(canvas);
            }
        }
    }
    fn update(&mut self, delta: f32){

    }
}
