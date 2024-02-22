pub mod behaviour;

use crate::types::Position;
use std::cmp::max;

use self::behaviour::{Action, Drink};

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

    pub fn step_time(&mut self) {
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

        self.make_optimal_action();
    }

    pub fn make_optimal_action(&mut self) {
        self.do_action::<Drink>(30)
    }

    pub fn do_action<A>(&mut self, value : i32) 
        where A : Action {
            A::execute(self, value);
        }
}

pub enum Need {
    Hunger(i32), 
    Thirst(i32),
    Energy(i32),
    Money(i32)
}