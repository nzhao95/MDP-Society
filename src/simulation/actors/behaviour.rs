
use crate::learning::qlearning::{Agent, Policy, State};
use crate::simulation::actors::humans::Human;
use crate::simulation::world::{Element, Environment};
use crate::types::Position;

use rand::Rng;
use std::cmp::max;

pub struct QLBehaviour {
    policy : Policy
}

impl QLBehaviour {
    pub fn new() -> QLBehaviour {
        QLBehaviour {
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

impl Behaviour for QLBehaviour {
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
        if !self.alive {
            return -1000.0;
        }

        let hunger_reward = if self.hunger.value > 80 { 1.0 }
        else if self.hunger.value > 50 { 0.0 }
        else if self.hunger.value > 20 { -30.0 } 
        else { -100.0 };

        let thirst_reward = if self.thirst.value > 80 { 1.0 }
        else if self.thirst.value > 50 { 0.0 }
        else if self.thirst.value > 20 { -30.0 } 
        else { -100.0 };

        let env = self.environment.read().unwrap();
        let resource_reward = match env.get_element(self.position.x as usize, self.position.y as usize) {
            Element::Water(_) => 1.0,
            Element::Tree(_) => 1.0,
            _ => 0.0
        };

        let age_reward = (self.age as f64 - 100.0).max(0.0) * 10000.0;

        hunger_reward + thirst_reward +resource_reward + age_reward
    }

    fn simulate_action(&mut self, action : usize) -> (State, f64, bool) {
        let mut reward = 0.0;
        match action {
            0 => reward = Move::execute(self, Position::new(1, 0)),
            1 => reward = Move::execute(self, Position::new(-1, 0)),
            2 => reward = Move::execute(self, Position::new(0, 1)),
            3 => reward = Move::execute(self, Position::new(0, -1)),
            4 => reward = Drink::execute(self, 30),
            5 => reward = Eat::execute(self, 30),
            _ => ()
            
        }
        self.simulation_step_time();        
        (encode(&self), reward + self.compute_reward(), !self.alive || self.age > 10000)
    }

    fn choose_action(&self) -> usize {
        let behaviour = self.behaviour.read().unwrap();
        behaviour.predict_action(self)
    }

    fn do_action(&mut self, action : usize) {
        match action {
            0 => Move::execute(self, Position::new(1, 0)),
            1 => Move::execute(self, Position::new(-1, 0)),
            2 => Move::execute(self, Position::new(0, 1)),
            3 => Move::execute(self, Position::new(0, -1)),
            4 => Drink::execute(self, 30),
            5 => Eat::execute(self, 30),
            _ => 0.0
        };
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
        let previous_thirst = human.thirst.value;
        if let Element::Water(_) = human.environment.read().unwrap()
            .get_element(human.position.x as usize, human.position.y as usize) {
                human.thirst.value = 100.min(human.thirst.value + value);
                return (human.thirst.value - previous_thirst) as f64 * 10.0;
            }
        -1.0
    }
}
pub struct Eat;

impl Action for Eat {
    type Item = i32;
    fn execute(human: &mut Human, value : Self::Item) -> f64{
        let previous_hunger = human.hunger.value;
        
        if let Element::Tree(_) = human.environment.read().unwrap()
                                .get_element(human.position.x as usize, human.position.y as usize) {
            human.hunger.value = 100.min(human.hunger.value + value);
            return (human.hunger.value - previous_hunger) as f64* 10.0;
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

        0.0
    }
} 


fn encode(human: &Human) -> State {
    let env = human.environment.read().unwrap();
    let thirst_state = match human.thirst.value {
        v if v > 80 => 0,
        v if v > 50 => 1,
        v if v > 20 => 2,
        _ => 3,
    };

    let hunger_state = match human.hunger.value {
        v if v > 80 => 0,
        v if v > 50 => 1,
        v if v > 20 => 2,
        _ => 3,
    };

    let closest_lake = env.closest_lake(human);
    let lake_direction = *closest_lake - human.position;
    let lake_direction_state = 
        if lake_direction.x.abs() > lake_direction.y.abs() {
            if lake_direction.x >= 0 { 0 } 
            else { 1 }
        } else {
            if lake_direction.y >= 0 { 2 } 
            else { 3 }
        };

    let closest_forest = env.closest_forest(human);
    let forest_direction = *closest_forest - human.position;
    let forest_direction_state = 
        if forest_direction.x.abs() > forest_direction.y.abs() {
            if forest_direction.x >= 0 { 0 } 
            else { 1 }
        } else {
            if forest_direction.y >= 0 { 2 } 
            else { 3 }
        };

    let current_element = match env.get_element(human.position.x as usize, human.position.y as usize) {
        Element::Water(_) => 0,
        Element::Tree(_) => 1,
        _ => 2,
    };

    // Calculate the key using the encoded states
    let key = (((((human.position.x as usize * env.world_limits.1 + human.position.y as usize) * 4
        + thirst_state as usize) * 4
        + hunger_state as usize) * 4
        + lake_direction_state as usize) * 4
        + forest_direction_state as usize) * 3
        + current_element as usize;

    State { key: key as usize }
}


fn nb_states(human : &Human) -> usize {
    let env = human.environment.read().unwrap();
    env.world_limits.0      // World Height
    * env.world_limits.1    // World Width
    * 4                     // Thirst States
    * 4                     // Hunger States    
    * 4                     // Closeness to Forest States
    * 4                     // Closeness to Lake States
    * 3                     // Current Element
}