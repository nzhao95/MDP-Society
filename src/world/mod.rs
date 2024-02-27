use crate::types::Position;
use crate::humans::Human;
use std::{cell::RefCell, cmp::{max, min}, ops::DerefMut, rc::Rc, sync::{Arc, Mutex, RwLock}};

#[derive(Clone, Copy)]
pub enum Element {
    None, 
    Tree(f64), 
    Water(f64), 
    Grass(f64),
    House(f64)
}

pub struct World {
    pub humans : Vec<Human>,
    pub environment : Arc<RwLock<Environment>>,
    pub cell_size : usize
}

pub struct Environment {
    pub cells : Vec<Vec<Element>>,
    pub world_limits : (usize, usize),
}

impl World {
    pub fn new(height : usize, width : usize, cell_size : usize) -> Self {
        World{
            humans : Vec::new(),
            environment : Arc::new(RwLock::new(Environment{
                cells : vec![vec![Element::None; width]; height],
                world_limits : (height, width)
            })),
            cell_size
        }
    }

    pub fn add_human(&mut self, human : Human) {
        self.humans.push(human);
    }

    fn set_cell(environment : &mut Environment, start : Position, stop : Position, val : Element) {
        for x in max(start.x as usize, 0)
        ..min(stop.x as usize, environment.cells.len()) {
            for y in max(start.y as usize, 0)
            ..min(stop.y as usize, environment.cells[x].len()) {
                environment.cells[x][y] = val;
            }
        }
    }

    pub fn add_forest(&mut self, start : Position, stop : Position) {
        World::set_cell(self.environment.write().unwrap().deref_mut(), start, stop, Element::Tree(1.0));
    }

    pub fn add_lake(&mut self, start : Position, stop : Position) {
        World::set_cell(self.environment.write().unwrap().deref_mut(), start, stop, Element::Water(1.0));
    }

    pub fn step_time(&mut self) {
        for human in self.humans.iter_mut() {
            human.step_time();
        }
    }
}