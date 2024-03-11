use crate::types::Position;
use crate::simulation::actors::humans::Human;
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
    pub forests : Vec<Position>,
    pub lakes : Vec<Position>
}

impl Environment {
    pub fn get_element(&self, x : usize, y : usize) -> &Element {
        &self.cells[x][y]
    }

    pub fn distance_to_lake(&self, human : &Human) -> i32 {
        self.lakes
        .iter()
        .map(|x| x.manhattan_dist(&human.position))
        .min()
        .unwrap()
    }

    pub fn distance_to_forest(&self, human : &Human) -> i32 {
        self.forests.iter()
        .map(|x| x.manhattan_dist(&human.position))
        .min()
        .unwrap()
    }

    pub fn closest_lake(&self, human : &Human) -> &Position {
        self.lakes
        .iter()
        .min_by(|a,b| a.manhattan_dist(&human.position).cmp(&b.manhattan_dist(&human.position)))
        .unwrap()
    } 

    pub fn closest_forest(&self, human : &Human) -> &Position {
        self.forests
        .iter()
        .min_by(|a,b| a.manhattan_dist(&human.position).cmp(&b.manhattan_dist(&human.position)))
        .unwrap()
    } 
}

impl World {
    pub fn new(height : usize, width : usize, cell_size : usize) -> Self {
        World{
            humans : Vec::new(),
            environment : Arc::new(RwLock::new(Environment{
                cells : vec![vec![Element::None; width]; height],
                world_limits : (height, width),
                forests : Vec::new(),
                lakes : Vec::new()
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
        let mut environment = self.environment.write().unwrap();
        World::set_cell(environment.deref_mut(), start, stop, Element::Tree(1.0));
        let mid = Position{ x : (start.x + stop.x)/2, y : (start.y + stop.y)};
        environment.forests.push(mid);
    }

    pub fn add_lake(&mut self, start : Position, stop : Position) {
        let mut environment = self.environment.write().unwrap();
        World::set_cell(environment.deref_mut(), start, stop, Element::Water(1.0));
        let mid = Position{ x : (start.x + stop.x)/2, y : (start.y + stop.y)};
        environment.lakes.push(mid);
    }

    pub fn step_time(&mut self) {
        for human in self.humans.iter_mut() {
            human.step_time();
        }
    }
}