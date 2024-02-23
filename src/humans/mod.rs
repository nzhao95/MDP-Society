pub mod behaviour;

use crate::world::Environment;
use crate::types::Position;
use std::cmp::max;

use self::behaviour::{Action, Drink, Move};

pub struct Human {
    pub position : Position,
    pub needs : Vec<Need>,
    pub alive : bool
}

impl Human {
    pub fn new(x : i32, y : i32) -> Self{
        Human{
            position : Position{x, y},
            needs : vec![Need::Hunger(100),
            Need::Thirst(100),
            Need::Energy(100),
            Need::Money(0)],
            alive : true
        }
    }

    pub fn step_time(&mut self, environment : &Vec<Vec<Environment>>) {
        for need in self.needs.iter_mut() {
            match need {
                Need::Hunger(val) => {
                    *val = max(*val - 1, 0);
                    if *val <= 0 {
                        self.alive = false;
                    }
                } ,
                Need::Thirst(val) => {
                    *val = max(*val - 1, 0);
                    if *val <= 0 {
                        self.alive = false;
                    }
                },
                Need::Energy(val) => {
                    *val = max(*val - 1, 0);
                },
                Need::Money(val) => {
                    *val = max(*val - 1, 0);
                }
            }
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

pub enum Need {
    Hunger(i32), 
    Thirst(i32),
    Energy(i32),
    Money(i32)
}