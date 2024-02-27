pub mod behaviour;

use crate::learning::reinforcement::Agent;
use crate::world::{Element, Environment};
use crate::types::Position;
use std::sync::{Arc, RwLock};
use std::cmp::max;

use self::behaviour::{Behaviour, RlBehaviour};

pub struct Human {
    pub position : Position,
    pub age : u32,
    pub hunger : Need,
    pub thirst : Need,
    pub energy : Need,
    pub money : Need,
    pub alive : bool,
    pub behaviour : Arc<RwLock<RlBehaviour>>,
    pub environment : Arc<RwLock<Environment>>
}

impl Human {
    pub fn new(x : i32, y : i32, behaviour : Arc<RwLock<RlBehaviour>>, environment :Arc<RwLock<Environment>>) -> Self{
        Human{
            position : Position{x, y},
            age : 0,
            hunger : Need{value : 100, min_value : 0, max_value : 100},
            thirst : Need{value : 100, min_value : 0, max_value : 100},
            energy : Need{value : 100, min_value : 0, max_value : 100},
            money : Need{value : 0, min_value : 0, max_value : i32::MAX},
            alive : true,
            behaviour : behaviour.clone(),
            environment
        }
    }

    pub fn step_time(&mut self) {
        {
            self.hunger.value = max(self.hunger.value - 1, 0);
            if self.hunger.value <= 0 {
                self.alive = false;
            }
        }
        {
            self.thirst.value = max(self.thirst.value - 1, 0);
            if self.thirst.value <= 0 {
                self.alive = false;
            }
        }
        {
            self.energy.value = max(self.energy.value - 1, 0);
        }
        {
            self.money.value = max(self.money.value - 1, 0);
        }

        self.age += 1;
        
        self.step();
    }

    fn find_closest(&self, element : Element) -> Option<Position> {
        let environment = &self.environment.read().unwrap();
        let cells = &environment.cells;
        
        let (m, n)= environment.world_limits;

        let curr_x = self.position.x;
        let curr_y = self.position.y;

        let max_dist = curr_x
            .max(curr_y
            .max((m as i32 - curr_x)
            .max((n as i32 - curr_y))));
        
        let mut to_test = Vec::new();
        to_test.push(self.position);

                
        for i in 0..max_dist {
            
            for p in to_test.iter() {
                if p.x < 0 || p.y < 0 || p.x >= m as i32 || p.y >= n as i32 {
                    continue;
                }
    
                match cells[p.x as usize][p.y as usize] {
                    Element::Grass(_) => if matches!(element, Element::Grass(_)) {return Some(*p)},
                    Element::Water(_) => if matches!(element, Element::Water(_)) {return Some(*p)},
                    Element::Tree(_) => if matches!(element, Element::Tree(_)) {return Some(*p)},
                    Element::House(_) => if matches!(element, Element::House(_)) {return Some(*p)},
                    _ => ()
                }
            }

            to_test.clear();
            for j in 0..i {
                to_test.push(Position::new(i - j + curr_x, j + curr_y));
                to_test.push(Position::new(-i + j + curr_x, j + curr_y));
                to_test.push(Position::new(i - j + curr_x, -j + curr_y));
                to_test.push(Position::new(-i + j + curr_x, -j + curr_y));
            }
        }

        return None;
    }
}

pub struct Need {
    pub value : i32, 
    pub min_value : i32,
    pub max_value : i32
}