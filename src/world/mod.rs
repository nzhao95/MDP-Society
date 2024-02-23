use crate::types::Position;
use crate::humans::Human;
use std::cmp::{max, min};

#[derive(Clone, Copy)]
pub enum Environment {
    None, 
    Tree(f64), 
    Water(f64), 
    Grass(f64),
    House(f64)
}

pub struct World {
    pub humans : Vec<Human>,
    pub cells : Vec<Vec<Environment>>,
    pub cell_size : usize
}

impl World {
    pub fn new(width : usize, height : usize, cell_size : usize) -> Self {
        World{
            humans : Vec::new(),
            cells : vec![vec![Environment::None; height]; width],
            cell_size
        }
    }

    pub fn add_human(&mut self, x: i32, y:i32) {
        self.humans.push(Human::new(x, y));
    }

    fn set_cell(&mut self, start : Position, stop : Position, val : Environment) {
        for x in max(start.x as usize, 0)..min(stop.x as usize, self.cells.len()) {
            for y in max(start.y as usize, 0)..min(stop.y as usize, self.cells[x].len()) {
                self.cells[x][y] = val;
            }
        }
    }

    pub fn add_forest(&mut self, start : Position, stop : Position) {
        self.set_cell(start, stop, Environment::Tree(1.0));
    }

    pub fn add_lake(&mut self, start : Position, stop : Position) {
        self.set_cell(start, stop, Environment::Water(1.0));
    }

    pub fn step_time(&mut self) {
        for human in self.humans.iter_mut() {
            human.step_time(&self.cells);
        }
    }
}