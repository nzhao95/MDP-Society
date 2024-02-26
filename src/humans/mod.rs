pub mod behaviour;

use crate::world::Environment;
use crate::types::Position;
use std::cmp::max;

use self::behaviour::{Action, Drink, Move};

pub struct Human {
    pub position : Position,
    pub hunger : Need,
    pub thirst : Need,
    pub energy : Need,
    pub money : Need,
    pub alive : bool
}

impl Human {
    pub fn new(x : i32, y : i32) -> Self{
        Human{
            position : Position{x, y},
            hunger : Need{value : 100, min_value : 0, max_value : 100},
            thirst : Need{value : 100, min_value : 0, max_value : 100},
            energy : Need{value : 100, min_value : 0, max_value : 100},
            money : Need{value : 0, min_value : 0, max_value : i32::MAX},
            alive : true
        }
    }

    pub fn step_time(&mut self, environment : &Vec<Vec<Environment>>) {
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

        self.make_optimal_action(environment);
    }

    pub fn make_optimal_action(&mut self, environment : &Vec<Vec<Environment>>) {
        let x = self.position.x as usize;
        let y = self.position.y as usize;
        if environment.is_empty() || environment[0].is_empty() {
            return;
        }
        let m = environment.len();
        let n = environment[0].len();
        
        // TODO : Check if we only want positive positions
        if x >= m || y >= n {
            return;
        }

        if let Environment::Water(_) = environment[x][y] {
            self.do_action::<Drink>(30);
        }
        else {
            if let Some(closest_water) = self.find_closest(Environment::Water(0.0), environment) {
                let direction = closest_water - self.position;
                if direction.x.abs() > direction.y.abs() {
                    self.do_action::<Move>(Position::new(direction.x.signum(), 0));
                }
                else if direction.x.abs() < direction.y.abs() {
                    self.do_action::<Move>(Position::new(0, direction.y.signum()));
                }
                else {
                    self.do_action::<Move>(Position::new(direction.x.signum(), direction.y.signum()));    
                }
            }
        }
    }

    pub fn do_action<A>(&mut self, value : A::Item) 
        where A : Action {
        A::execute(self, value);
    }

    fn find_closest(&self, element : Environment, environment : &Vec<Vec<Environment>>) -> Option<Position> {
        if environment.is_empty() || environment[0].is_empty() {
            return None;
        }
        let m = environment.len();
        let n = environment[0].len();

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
    
                match environment[p.x as usize][p.y as usize] {
                    Environment::Grass(_) => if matches!(element, Environment::Grass(_)) {return Some(*p)},
                    Environment::Water(_) => if matches!(element, Environment::Water(_)) {return Some(*p)},
                    Environment::Tree(_) => if matches!(element, Environment::Tree(_)) {return Some(*p)},
                    Environment::House(_) => if matches!(element, Environment::House(_)) {return Some(*p)},
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