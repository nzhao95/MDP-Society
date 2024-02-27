use std::cmp::max;

use rand::Rng;

use crate::types::Position;

use crate::humans::Human;
use crate::learning::reinforcement::{Agent, Policy, State};
use crate::world::Element;

fn encode(human : &Human) -> State {
    let key = ((
      human.position.x as usize    * human.environment.read().unwrap().world_limits.1 
    + human.position.y as usize  ) * human.thirst.max_value as usize
    + human.thirst.value as usize) * human.hunger.max_value as usize
    + human.hunger.value as usize;
    
    State{key : key as usize}
}

fn nb_states(human : &Human) -> usize {
    let env = human.environment.read().unwrap();
    env.world_limits.0 
    * env.world_limits.1 
    * (human.thirst.max_value as usize + 1)
    * (human.hunger.max_value as usize + 1)
}
pub struct RlBehaviour {
    policy : Policy
}

impl RlBehaviour {
    pub fn new() -> RlBehaviour {
        RlBehaviour {
            policy : Policy::new()
        }
    }

    fn init(&mut self, train_agent : &mut Human) {
        self.policy.init(nb_states(&train_agent), 7);
    }

    pub fn train(&mut self, train_agent: &mut Human, iterations: usize, alpha: f64, gamma: f64, epsilon: f64) {
        self.init(train_agent);
        self.policy.train(train_agent, iterations, alpha, gamma, epsilon)
    }

    pub fn evaluate(&self, test_agent : &mut Human, iterations: usize) {
        self.policy.evaluate(test_agent, iterations)
    }
}


pub trait Behaviour {
    fn predict_action(&self, human : &Human) -> usize;    
    fn step(&self, human : &mut Human);
}

impl Behaviour for RlBehaviour {
    fn predict_action(&self, human : &Human) -> usize {
        let current_state = encode(human);
        self.policy.predict_action(&current_state)
    }
    
    fn step(&self, human : &mut Human) {
        human.do_action(self.predict_action(human));
    }
}

impl Agent for Human{
    fn reset(&mut self) -> State {
        let mut rng = rand::thread_rng();
        self.position = Position{x : rng.gen_range(0, self.environment.read().unwrap().world_limits.0 as i32), 
                                 y : rng.gen_range(0, self.environment.read().unwrap().world_limits.1 as i32)};
        self.age = 0;
        self.hunger.value = 100;
        self.thirst.value = 100;
        self.energy.value = 100;
        self.money.value = 0;
        self.alive = true;

        encode(&self)
    }

    fn simulation_step_time(&mut self) {
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
    }

    fn compute_reward(&self) -> f64{
        if self.age > 10000 {
            return 1000.0;
        }

        let hunger_reward = if self.hunger.value > 80 {
            5.0
        } else if self.hunger.value > 30 {
            1.0
        } else if self.hunger.value > 10 {
            1.0
        } else {
            -5.0
        };
        let thirst_reward = if self.thirst.value > 80 {
            1.0
        } else if self.thirst.value > 30 {
            0.0
        } else if self.thirst.value > 10 {
            -1.0
        } else {
            -5.0
        };

        hunger_reward + thirst_reward
    }

    fn do_action(&mut self, action : usize) -> (State, f64, bool) {
        let mut reward = 0.0;
        match action {
            0 => reward = Move::execute(self, Position::new(1, 0)),
            1 => reward = Move::execute(self, Position::new(-1, 0)),
            2 => reward = Move::execute(self, Position::new(0, 1)),
            3 => reward = Move::execute(self, Position::new(0, -1)),
            4 => reward = Drink::execute(self, 100),
            5 => reward = Eat::execute(self, 100),
            _ => ()
            
        }
        (encode(&self), reward + self.compute_reward(), !self.alive || self.age > 10000)
    }

    fn choose_action(&self) -> usize {
        let behaviour = self.behaviour.read().unwrap();
        behaviour.predict_action(self)
    }

    fn step(&mut self) {
        self.do_action(self.choose_action());
    }
}

pub trait Action {
    type Item;
    fn execute(human: &mut Human, value : Self::Item) -> f64;
}

pub struct Drink;

impl Action for Drink {
    type Item = i32;
    fn execute(human: &mut Human, value : Self::Item) -> f64{
        if let Element::Water(_) = human.environment.read().unwrap()
            .get_element(human.position.x as usize, human.position.y as usize) {
                human.thirst.value = 100.min(human.thirst.value + value);
                return 50.0;
            }
        -1.0
    }
}
pub struct Eat;

impl Action for Eat {
    type Item = i32;
    fn execute(human: &mut Human, value : Self::Item) -> f64{
        if let Element::Tree(_) = human.environment.read().unwrap()
                                .get_element(human.position.x as usize, human.position.y as usize) {
            human.hunger.value = 100.min(human.hunger.value + value);
            return 50.0;
        }
        -1.0
    }
}

pub struct Move;

impl Action for Move {
    type Item = Position;
    fn execute(human: &mut Human, value : Self::Item) -> f64 {
        human.position = human.position + value;
        let world_limits = human.environment.read().unwrap().world_limits;

        if human.position.x < 0 || human.position.x >= world_limits.0 as i32 {
            human.position.x = human.position.x.clamp(0, world_limits.0 as i32 - 1);
            return -1.0;
        }
        if human.position.y < 0 || human.position.y >= world_limits.1 as i32 {
            human.position.y = human.position.y.clamp(0, world_limits.1 as i32 - 1);
            return -1.0
        }
        1.0
    }
}